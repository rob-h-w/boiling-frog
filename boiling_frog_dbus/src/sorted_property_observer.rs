use std::sync::{Arc, Mutex};
use std::thread::spawn;

use log::error;
use zbus::blocking::fdo::PropertiesProxy;
use zbus::blocking::{Connection, PropertyIterator};
use zbus::names::InterfaceName;
use zbus::CacheProperties;

use crate::config::INDICATOR_SENSORS_SERVICE;
use crate::dbus_session::DbusSession;
use crate::metric::Metric;
use crate::GenericError;

#[derive(Debug)]
pub(crate) struct SortedPropertyObserver {
    max: Arc<Mutex<f64>>,
    metrics: Vec<Metric>,
    session: Arc<Mutex<DbusSession>>,
}

impl SortedPropertyObserver {
    pub fn new(
        session: &Arc<Mutex<DbusSession>>,
        metrics: Vec<Metric>,
    ) -> Result<SortedPropertyObserver, GenericError> {
        let observer = SortedPropertyObserver {
            max: Arc::new(Mutex::new(0f64)),
            metrics: metrics.clone(),
            session: session.clone(),
        };

        for metric in observer.metrics.clone() {
            spawn(move || run(&metric).expect("property handler thread runs"));
        }

        Ok(observer)
    }
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

fn run(metric: &Metric) -> Result<(), GenericError> {
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
            )
        }
    }
}
