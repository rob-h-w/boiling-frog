use std::error::Error;

use log::error;
use zbus::blocking::Connection;

use crate::max_fan::MaxFanProxyBlocking;
use crate::max_temp::MaxTempProxyBlocking;
use crate::simple_types::{Fan, Temp};

#[derive(Debug)]
pub(crate) struct DbusSession {
    cached_fan: Fan,
    cached_temp: Temp,
    session_connection: Connection,
}

impl DbusSession {
    pub(crate) fn new() -> DbusSession {
        let mut it = DbusSession {
            cached_fan: Fan {
                value: 0 as f64,
                units: "".to_string()
            },
            cached_temp: Temp {
                value: 0 as f64,
                units: "".to_string(),
            },
            session_connection: Connection::session()
                .map_err(|e| {
                    error!("zbus signal: {e}");
                    e
                })
                .unwrap(),
        };

        it.update();
        it
    }

    pub(crate) fn update(&mut self) -> bool {
        let old_fan = self.cached_fan.clone();
        let old_temperature = self.cached_temp.clone();
        self.cached_fan = self.get_fan().expect("Could not get max fan");
        self.cached_temp = self.get_temp().expect("Could not get max temperature");

        old_fan != self.cached_fan || old_temperature != self.cached_temp
    }

    fn get_temp(&self) -> Result<Temp, Box<dyn Error + Send + Sync>> {
        let proxy = MaxTempProxyBlocking::new(&self.session_connection)?;

        Ok(Temp {
            value: proxy.value()?,
            units: proxy.units()?
        })
    }

    fn get_fan(&self) -> Result<Fan, Box<dyn Error + Send + Sync>> {
        let proxy = MaxFanProxyBlocking::new(&self.session_connection)?;

        Ok(Fan {
            value: proxy.value()?,
            units: proxy.units()?
        })
    }

    pub(crate) fn fan(&self) -> Fan {
        self.cached_fan.clone()
    }

    pub(crate) fn temp(&self) -> Temp {
        self.cached_temp.clone()
    }
}
