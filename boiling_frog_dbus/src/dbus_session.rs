use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

use crate::config::{ACTIVE_SENSORS_PATH, INDICATOR_SENSORS_SERVICE};
use crate::dbus_info::DbusInfo;
use crate::mutex_helpers::lock;
use log::error;
use zbus::blocking::fdo::ObjectManagerProxy;
use zbus::blocking::Connection;
use zbus::names::OwnedInterfaceName;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Str};
use zbus::CacheProperties;

use crate::max_fan::MaxFanProxyBlocking;
use crate::max_temp::MaxTempProxyBlocking;
use crate::metric::Metric;
use crate::metric_value::MetricValue;
use crate::simple_types::{Fan, Temp};

macro_rules! refresh_unlocked {
    ($unlocked:expr, $refresh:ident, $destination:expr) => {{
        $unlocked
            .$refresh(&$destination)
            .map_err(|e| {
                error!("refresh error: {e}");
                e
            })
            .ok();
    }};
}

macro_rules! refresh {
    ($session:expr, $refresh:ident, $destination:expr) => {{
        {
            let unlocked = lock(&$session);
            if (unlocked.is_ok()) {
                refresh_unlocked!(unlocked.unwrap(), $refresh, $destination);
            }
        }
    }};
}

#[derive(Debug)]
pub(crate) struct DbusSession {
    cached_fan: Fan,
    cached_temp: Temp,
}

impl DbusSession {
    pub(crate) fn new() -> DbusSession {
        DbusSession {
            cached_fan: Fan {
                value: 0 as f64,
                units: "".to_string(),
            },
            cached_temp: Temp {
                value: 0 as f64,
                units: "".to_string(),
            },
        }
    }

    fn refresh_temp(
        &mut self,
        proxy: &MaxTempProxyBlocking,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.cached_temp = Temp {
            value: proxy.value()?,
            units: proxy.units()?,
        };

        Ok(())
    }

    fn refresh_fan(
        &mut self,
        proxy: &MaxFanProxyBlocking,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.cached_fan = Fan {
            value: proxy.value()?,
            units: proxy.units()?,
        };

        Ok(())
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
            .cache_properties(CacheProperties::Lazily)
            .destination(INDICATOR_SENSORS_SERVICE)?
            .path(ACTIVE_SENSORS_PATH)?
            .build()?;
        let managed_objects = object_manager_proxy.get_managed_objects()?;
        log_out(&managed_objects);
        let (fan_objects, temp_objects) = parse_objects(&managed_objects);

        let session = session_ref.clone();

        let temp = MaxTempProxyBlocking::builder(&connection)
            .cache_properties(CacheProperties::Lazily)
            .build()?;

        let fan = MaxFanProxyBlocking::builder(&connection)
            .cache_properties(CacheProperties::Lazily)
            .build()?;

        let mut fan_changed_signal = fan.receive_value_changed();
        let mut temp_changed_signal = temp.receive_value_changed();

        DbusSession::refresh(&session, &fan, &temp)?;

        let fan_session = session.clone();

        spawn(move || loop {
            fan_changed_signal.next().unwrap();
            refresh!(fan_session, refresh_fan, fan);
        });

        spawn(move || loop {
            temp_changed_signal.next().unwrap();
            refresh!(session, refresh_temp, temp);
        });

        Ok(())
    }

    fn refresh(
        session: &Arc<Mutex<DbusSession>>,
        fan: &MaxFanProxyBlocking,
        temp: &MaxTempProxyBlocking,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut unlocked_session = lock(session)?;
        refresh_unlocked!(unlocked_session, refresh_fan, fan);
        refresh_unlocked!(unlocked_session, refresh_temp, temp);
        Ok(())
    }
}

fn parse_objects(
    objects: &HashMap<OwnedObjectPath, HashMap<OwnedInterfaceName, HashMap<String, OwnedValue>>>,
) -> (Vec<MetricValue>, Vec<MetricValue>) {
    let mut fans = Vec::new();
    let mut temps = Vec::new();
    for (path, owned_object_path_map) in objects {
        for (interface_name, value_map) in owned_object_path_map {
            let dbus_info = DbusInfo::new(interface_name, path);
            let metric_option = Metric::try_metric(&dbus_info, value_map);
            if metric_option.is_some() {
                match metric_option.unwrap() {
                    Metric::Fan(metric) => fans.push(metric),
                    Metric::Temp(metric) => temps.push(metric),
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
