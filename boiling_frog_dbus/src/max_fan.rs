use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "com.github.alexmurray.IndicatorSensors.ActiveSensor",
    default_service = "com.github.alexmurray.IndicatorSensors",
    default_path = "/com/github/alexmurray/IndicatorSensors/ActiveSensors/libsensors\
    /asus_isa_0000/0"
)]
pub(crate) trait MaxFan {
    #[dbus_proxy(property)]
    fn value(&self) -> zbus::Result<f64>;
    #[dbus_proxy(property)]
    fn units(&self) -> zbus::Result<String>;
}
