use std::error::Error;
use log::error;
use zbus::{Connection, dbus_proxy};

#[dbus_proxy(
interface = "com.github.alexmurray.IndicatorSensors.ActiveSensor",
default_service = "com.github.alexmurray.IndicatorSensors",
default_path = "/com/github/alexmurray/IndicatorSensors/ActiveSensors/virtual/max"
)]
pub trait MaxTemp {
    #[dbus_proxy(property)]
    fn value(&self) -> zbus::Result<f64>;
    #[dbus_proxy(property)]
    fn units(&self) -> zbus::Result<String>;
}

pub struct Temp {
    pub value: f64,
    pub units: String
}

pub async fn get_temp<'a>() -> Result<Temp, Box<dyn Error + Send + Sync>> {
    let conn = Connection::session()
        .await
        .map_err(|e| {
            error!("zbus signal: {e}");
            e
        })
        .unwrap();

    let proxy = MaxTempProxy::new(&conn).await?;
    let value = proxy.value().await?;
    let units = proxy.units().await?.to_string();

    Ok(Temp {
        value,
        units
    })
}
