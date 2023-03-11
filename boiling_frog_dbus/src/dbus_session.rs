use std::error::Error;

use log::error;
use zbus::blocking::Connection;

use crate::max_temp::MaxTempProxyBlocking;
use crate::simple_types::Temp;

#[derive(Debug)]
pub(crate) struct DbusSession {
    cached_temp: Temp,
    session_connection: Connection,
}

impl DbusSession {
    pub(crate) fn new() -> DbusSession {
        let mut it = DbusSession {
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
        let old = self.cached_temp.clone();
        self.cached_temp = self.get_temp().expect("Could not get max temperature");

        old != self.cached_temp
    }

    fn get_temp(&self) -> Result<Temp, Box<dyn Error + Send + Sync>> {
        let proxy = MaxTempProxyBlocking::new(&self.session_connection)?;
        let value = proxy.value().expect("Could not get temperature value");
        let units = proxy.units().expect("Could not get temperature units");

        Ok(Temp { value, units })
    }

    pub(crate) fn temp(&self) -> Temp {
        self.cached_temp.clone()
    }
}
