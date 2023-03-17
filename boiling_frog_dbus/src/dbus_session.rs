use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

use log::error;
use zbus::blocking::Connection;
use zbus::CacheProperties;

use crate::max_fan::MaxFanProxyBlocking;
use crate::max_temp::MaxTempProxyBlocking;
use crate::simple_types::{Fan, Temp};

macro_rules! refresh_unlocked {
    ($unlocked:expr, $refresh:ident, $destination:expr, $message:literal) => {{
        $unlocked.$refresh(&$destination).expect($message);
    }};
}

macro_rules! refresh {
    ($session:expr, $refresh:ident, $destination:expr, $message:literal) => {{
        refresh_unlocked!($session.lock().unwrap(), $refresh, $destination, $message);
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

    pub(crate) fn run(session_ref: &Arc<Mutex<DbusSession>>) {
        let connection = Connection::session()
            .map_err(|e| {
                error!("zbus signal: {e}");
                e
            })
            .unwrap();
        let session = session_ref.clone();

        let temp = MaxTempProxyBlocking::builder(&connection)
            .cache_properties(CacheProperties::Lazily)
            .build()
            .unwrap();

        let fan = MaxFanProxyBlocking::builder(&connection)
            .cache_properties(CacheProperties::Lazily)
            .build()
            .unwrap();

        let mut fan_changed_signal = fan.receive_value_changed();
        let mut temp_changed_signal = temp.receive_value_changed();

        DbusSession::refresh(&session, &fan, &temp);

        let fan_session = session.clone();

        spawn(move || loop {
            fan_changed_signal.next().unwrap();
            refresh!(fan_session, refresh_fan, fan, "Update fan speed");
        });

        spawn(move || loop {
            temp_changed_signal.next().unwrap();
            refresh!(session, refresh_temp, temp, "Update temperature");
        });
    }

    fn refresh(
        session: &Arc<Mutex<DbusSession>>,
        fan: &MaxFanProxyBlocking,
        temp: &MaxTempProxyBlocking,
    ) {
        let mut unlocked_session = session.lock().unwrap();
        refresh_unlocked!(unlocked_session, refresh_fan, fan, "Update fan speed");
        refresh_unlocked!(unlocked_session, refresh_temp, temp, "Update temperature");
    }
}
