use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

use log::error;
use zbus::blocking::fdo::PropertiesProxy;
use zbus::blocking::{Connection, PropertyIterator};
use zbus::names::InterfaceName;
use zbus::CacheProperties;

use crate::config::INDICATOR_SENSORS_SERVICE;
use crate::metric::Metric;
use crate::mutex_helpers::lock;
use crate::GenericError;

type Callback = Arc<
    Mutex<Box<dyn Fn(String, f64, String) -> Result<(), GenericError> + Send + Sync + 'static>>,
>;
type OptionalCallback = Option<Callback>;

pub(crate) struct SortedPropertyObserverBuilder {
    callback: OptionalCallback,
    metrics: Vec<Metric>,
}

impl Clone for SortedPropertyObserverBuilder {
    fn clone(&self) -> Self {
        SortedPropertyObserverBuilder {
            callback: if self.callback.is_some() {
                Some(self.callback.as_ref().unwrap().clone())
            } else {
                None
            },
            metrics: self.metrics.clone(),
        }
    }
}

pub(crate) fn builder() -> SortedPropertyObserverBuilder {
    SortedPropertyObserverBuilder {
        callback: None,
        metrics: vec![],
    }
}

impl SortedPropertyObserverBuilder {
    pub(crate) fn and(&mut self) -> &mut SortedPropertyObserverBuilder {
        self
    }

    pub(crate) fn build(&mut self) -> Result<(), GenericError> {
        spawn_workers(self)?;
        Ok(())
    }

    pub(crate) fn with_metrics(
        &mut self,
        metrics: &Vec<Metric>,
    ) -> &mut SortedPropertyObserverBuilder {
        self.metrics = metrics.clone();
        self
    }

    pub(crate) fn with_on_change_callback(
        &mut self,
        callback: &Callback,
    ) -> &mut SortedPropertyObserverBuilder {
        self.callback = Some(callback.clone());
        self
    }
}

fn spawn_workers(source: &SortedPropertyObserverBuilder) -> Result<(), GenericError> {
    let state = Arc::new(Mutex::new(State {
        builder: (*source).clone(),
        max: PropertyValue {
            name: "".to_string(),
            units: "".to_string(),
            value: 0.0,
        },
        samples: HashMap::new(),
    }));

    for metric in source.metrics.clone() {
        let state_ref = state.clone();
        spawn(move || run(&metric, state_ref).expect("property handler thread runs"));
    }

    Ok(())
}

fn make_property<'a, 'b: 'a>(
    connection: &'a Connection,
    metric: &'b Metric,
) -> Result<PropertiesProxy<'a>, GenericError> {
    let value = metric.get_value();
    let path = &value.dbus_info.path;
    Ok(PropertiesProxy::builder(&connection)
        .cache_properties(CacheProperties::Lazily)
        .destination(INDICATOR_SENSORS_SERVICE)?
        .path(path.as_str())?
        .interface(InterfaceName::try_from(
            (&value).dbus_info.interface_name.as_str(),
        )?)?
        .build()?)
}

fn run(metric: &Metric, state: Arc<Mutex<State>>) -> Result<(), GenericError> {
    {
        let connection = Connection::session().map_err(|e| {
            error!("zbus signal: {e}");
            e
        })?;

        let value = metric.get_value().clone();
        let property = make_property(&connection, metric)?;
        let mut changed_signal: PropertyIterator<f64> = property.receive_property_changed("Value");
        println!(
            "listening for {} = {}{}",
            value.label.clone(),
            value.value,
            value.units
        );
        update(
            &state,
            PropertyValue {
                name: value.label.clone(),
                units: value.units.clone(),
                value: value.value,
            },
        )?;
        loop {
            let change = changed_signal.next().unwrap();
            println!(
                "{} changed to {}{}",
                value.label,
                change.get()?,
                value.units
            );

            update(
                &state,
                PropertyValue {
                    name: value.label.clone(),
                    units: value.units.clone(),
                    value: change.get()?,
                },
            )?;
        }
    }
}

fn update(state: &Arc<Mutex<State>>, property_value: PropertyValue) -> Result<(), GenericError> {
    let mut locked_state = lock(&state)?;
    locked_state.insert(&property_value)?;
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
struct PropertyValue {
    name: String,
    units: String,
    value: f64,
}

#[derive(Clone)]
struct State {
    builder: SortedPropertyObserverBuilder,
    max: PropertyValue,
    samples: HashMap<String, PropertyValue>,
}

impl State {
    fn insert(&mut self, property_value: &PropertyValue) -> Result<(), GenericError> {
        self.samples
            .insert(property_value.name.clone(), property_value.clone());
        let old_max = &self.max;
        let new_max = self.max();

        if let Some(max) = new_max {
            if old_max != max {
                self.max = max.clone();
                self.call_callback()?;
            }
        }

        Ok(())
    }

    fn call_callback(&mut self) -> Result<(), GenericError> {
        if let Some(callback) = &self.builder.callback {
            if let Some(max) = self.max() {
                let locked_callback = lock(&callback)?;
                locked_callback(max.name.clone(), max.value, max.units.clone())?;
            }
        }

        Ok(())
    }

    fn max(&self) -> Option<&PropertyValue> {
        self.samples
            .values()
            .max_by(|left, right| left.value.total_cmp(&right.value))
    }
}
