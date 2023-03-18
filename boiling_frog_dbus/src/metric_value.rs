use crate::dbus_info::DbusInfo;
use crate::error::{BadPropertyTypeError, MissingPropertyError};
use crate::GenericError;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use zbus::zvariant::{OwnedValue, Str};

macro_rules! thing_from {
    ($t:ident, $value:expr) => {{
        $value
            .ok_or(MissingPropertyError {})?
            .downcast_ref::<$t>()
            .ok_or(BadPropertyTypeError {})
    }};
}

#[derive(Debug)]
pub(crate) struct MetricValue {
    pub dbus_info: DbusInfo,
    pub label: String,
    pub units: String,
    pub value: f64,
}

impl Display for MetricValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl MetricValue {
    pub(crate) fn value_from(
        dbus_info: &DbusInfo,
        map: &HashMap<String, OwnedValue>,
    ) -> Result<MetricValue, GenericError> {
        let label = thing_from!(Str, map.get("Label"))?.to_string();
        let units = thing_from!(Str, map.get("Units"))?.to_string();
        let value = thing_from!(f64, map.get("Value"))?;

        Ok(MetricValue {
            dbus_info: dbus_info.clone(),
            label,
            units,
            value: value.clone(),
        })
    }
}
