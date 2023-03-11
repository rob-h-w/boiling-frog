use zbus::dbus_proxy;

pub struct Temp {
    pub value: f64,
    pub units: String
}

#[dbus_proxy(
interface = "com.github.alexmurray.IndicatorSensors.ActiveSensor",
default_service = "com.github.alexmurray.IndicatorSensors",
default_path = "/com/github/alexmurray/IndicatorSensors/ActiveSensors/virtual/max"
)]
pub(crate) trait MaxTemp {
    #[dbus_proxy(property)]
    fn value(&self) -> zbus::Result<f64>;
    #[dbus_proxy(property)]
    fn units(&self) -> zbus::Result<String>;
}