use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

use log::error;
use zbus::blocking::{Connection, PropertyIterator};
use zbus::blocking::fdo::PropertiesProxy;
use zbus::CacheProperties;
use zbus::names::InterfaceName;

use crate::config::INDICATOR_SENSORS_SERVICE;
use crate::dbus_session::DbusSession;
use crate::GenericError;
use crate::metric::Metric;
use crate::mutex_helpers::lock;

type Callback = Arc<Mutex<Box<dyn Fn(String, f64, String) -> Result<(), GenericError> +
Send + Sync
+ 'static>>>;
type OptionalCallback = Option<Callback>;

pub(crate) struct SortedPropertyObserverBuilder {
    callback: OptionalCallback,
    metrics: Vec<Metric>,
}

pub(crate) fn builder() -> SortedPropertyObserverBuilder {
    SortedPropertyObserverBuilder { callback: None, metrics: vec![] }
}

impl SortedPropertyObserverBuilder {
    pub(crate) fn and(&mut self) -> &mut SortedPropertyObserverBuilder {
        self
    }

    pub(crate) fn build(&mut self) -> Result<(), GenericError> {
        spawn_workers(self)?;
        Ok(())
    }

    pub(crate) fn with_metrics(&mut self, metrics: &Vec<Metric>) -> &mut
    SortedPropertyObserverBuilder {
        self.metrics = metrics.clone();
        self
    }

    pub(crate) fn with_on_change_callback(&mut self, callback: &Callback) -> &mut
    SortedPropertyObserverBuilder {
        self.callback = Some(callback.clone());
        self
    }
}


fn spawn_workers(
    source: & SortedPropertyObserverBuilder,
) -> Result<(), GenericError> {
    for metric in source.metrics.clone() {
        let callback = if source.callback.is_some() {
            Some(source.callback.as_ref().unwrap().clone())
        } else {
            None
        };
        spawn(move || run(&metric, &callback).expect("property handler thread runs"));
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
        .path(path.as_str().clone())?
        .interface(InterfaceName::try_from(
            (&value).dbus_info.interface_name.as_str().clone(),
        )?)?
        .build()?)
}

fn run(metric: &Metric, callback: &OptionalCallback) -> Result<(), GenericError> {
    {
        let connection = Connection::session().map_err(|e| {
            error!("zbus signal: {e}");
            e
        })?;

        let value = metric.get_value().clone();
        let property = make_property(&connection, metric)?;
        let mut changed_signal: PropertyIterator<f64> = property.receive_property_changed("Value");
        println!("listening for {}", value.label);
        loop {
            let change = changed_signal.next().unwrap();
            println!(
                "{} changed to {}{}",
                value.label,
                change.get()?,
                value.units
            );

            if let Some(c) = callback {
                let locked = lock(&c)?;
                locked(value.label.clone(), change.get()?, value.units.clone())?;
            }
        }
    }
}

#[derive(Clone, Debug)]
struct PropertyValue<'a> {
    name: &'a str,
    value: f64,
}
