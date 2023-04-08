use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

use log::error;
use zbus::blocking::fdo::ObjectManagerProxy;
use zbus::blocking::Connection;
use zbus::names::OwnedInterfaceName;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Str};

use crate::config::{ACTIVE_SENSORS_PATH, INDICATOR_SENSORS_SERVICE};
use crate::dbus_info::DbusInfo;
use crate::metric::Metric;
use crate::mutex_helpers::lock;
use crate::simple_types::{Fan, Temp};
use crate::sorted_property_observer::builder;

#[derive(Debug)]
pub(crate) struct DbusSession {
    cached_fan: Fan,
    cached_temp: Temp,
}

impl DbusSession {
    pub(crate) fn new() -> DbusSession {
        DbusSession {
            cached_fan: Fan {
                label: "".to_string(),
                value: 0 as f64,
                units: "".to_string(),
            },
            cached_temp: Temp {
                label: "".to_string(),
                value: 0 as f64,
                units: "".to_string(),
            },
        }
    }

    pub(crate) fn fan(&self) -> Fan {
        self.cached_fan.clone()
    }

    pub(crate) fn temp(&self) -> Temp {
        self.cached_temp.clone()
    }

    pub(crate) fn run(
        session_ref: &Arc<Mutex<DbusSession>>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let connection = Connection::session().map_err(|e| {
            error!("zbus signal: {e}");
            e
        })?;
        let object_manager_proxy: ObjectManagerProxy = ObjectManagerProxy::builder(&connection)
            .destination(INDICATOR_SENSORS_SERVICE)?
            .path(ACTIVE_SENSORS_PATH)?
            .build()?;
        let managed_objects = object_manager_proxy.get_managed_objects()?;
        log_out(&managed_objects);
        let (fan_objects, temp_objects) = parse_objects(&managed_objects);

        let fan_callback_session = session_ref.clone();
        builder()
            .with_metrics(&fan_objects)
            .and()
            .with_on_change_callback(&Arc::new(Mutex::new(Box::new(
                move |label: String, value: f64, units: String| {
                    let mut locked = lock(&fan_callback_session)?;
                    locked.cached_fan.label = label;
                    locked.cached_fan.units = units;
                    locked.cached_fan.value = value;
                    Ok(())
                },
            ))))
            .build()?;

        let temp_callback_session = session_ref.clone();
        builder()
            .with_metrics(&temp_objects)
            .and()
            .with_on_change_callback(&Arc::new(Mutex::new(Box::new(
                move |label: String, value: f64, units: String| {
                    let mut locked = lock(&temp_callback_session)?;
                    locked.cached_temp.label = label;
                    locked.cached_temp.units = units;
                    locked.cached_temp.value = value;
                    Ok(())
                },
            ))))
            .build()?;

        Ok(())
    }
}

fn parse_objects(
    objects: &HashMap<OwnedObjectPath, HashMap<OwnedInterfaceName, HashMap<String, OwnedValue>>>,
) -> (Vec<Metric>, Vec<Metric>) {
    let mut fans = Vec::new();
    let mut temps = Vec::new();
    for (path, owned_object_path_map) in objects {
        for (interface_name, value_map) in owned_object_path_map {
            let dbus_info = DbusInfo::new(interface_name, path);
            let metric_option = Metric::try_metric(&dbus_info, value_map);
            if let Some(metric) = metric_option {
                if !metric.get_value().dbus_info.path.contains("/virtual/") {
                    match metric {
                        Metric::Fan(_) => fans.push(metric.clone()),
                        Metric::Temp(_) => temps.push(metric.clone()),
                    };
                }
            }
        }
    }

    (fans, temps)
}

fn log_out(
    objects: &HashMap<OwnedObjectPath, HashMap<OwnedInterfaceName, HashMap<String, OwnedValue>>>,
) {
    for (path, path_map) in objects.iter() {
        for (iname, map) in path_map.iter() {
            let dbus_info = DbusInfo::new(iname, path);
            if let Some(metric) = Metric::try_metric(&dbus_info, map) {
                println!("metric = {}", metric);
            } else {
                println!("could not handle {}", dbus_info);

                for (value_name, owned_value) in map.iter() {
                    if let Some(value) = &owned_value.downcast_ref::<Str>() {
                        println!(
                            "path = {}, interface name = {}, value_name = {}, \
                    owned_value = {}",
                            path, iname, value_name, value
                        );
                    } else {
                        println!(
                            "path = {}, interface name = {}, value_name = {}",
                            path, iname, value_name
                        );
                    }
                }
            }
        }
    }
}
