use std::error::Error;
use std::sync::{Mutex};
use log::error;
use once_cell::sync::Lazy;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;
use zbus::Connection;

use crate::types::{Temp, MaxTempProxy};

pub static DBUS_ENGINE: Lazy<Mutex<Box<dyn DbusEngine>>> = Lazy::new(|| {
    Mutex::new(Box::new(DbusEngineImpl::new()))
});

pub trait DbusEngine: Send + Sync {
    fn temp(&self) -> &Temp;
}

struct DbusEngineImpl {
    runtime: Runtime,
    session_connection: Connection,
    cached_temp: Temp
}

impl DbusEngineImpl {
    fn new() -> DbusEngineImpl {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let session_connection = runtime.block_on(async {
            Connection::session()
                .await.map_err(|e| {
                error!("zbus signal: {e}");
                e
            })
                .unwrap()
        });
        let mut instance = DbusEngineImpl {
            runtime,
            session_connection,
            cached_temp: Temp { value: 0 as f64, units: "".to_string()}
        };

        instance.blocking_update();

        instance
    }

    fn blocking_update(&mut self) {
        let runtime = & self.runtime;
        let mut temp: Option<Temp> = None;

        runtime.block_on(async {
            temp = Some(self.get_temp().await.unwrap());
        });

        self.cached_temp = temp.unwrap();
    }

    async fn get_temp(&self) -> Result<Temp, Box<dyn Error + Send + Sync>> {
        let proxy = MaxTempProxy::new(&self.session_connection).await?;
        let value = proxy.value().await?;
        let units = proxy.units().await?.to_string();

        Ok(Temp {
            value,
            units
        })
    }

    async fn update(&mut self) {
        self.cached_temp = self.get_temp().await.unwrap();
    }
}

impl DbusEngine for DbusEngineImpl {
    fn temp(& self) -> &Temp {
        &self.cached_temp
    }
}