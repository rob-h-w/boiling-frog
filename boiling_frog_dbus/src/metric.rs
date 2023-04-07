use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::dbus_info::DbusInfo;
use crate::error::UnknownMetricUnitsError;
use zbus::zvariant::OwnedValue;

use crate::metric_value::MetricValue;
use crate::GenericError;

#[derive(Clone, Debug)]
pub(crate) enum Metric {
    Fan(MetricValue),
    Temp(MetricValue),
}

impl Display for Metric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Metric::Fan(value) => {
                let _ = f.write_str("Fan { ");
                let _ = write!(f, "{:?}", value);
                f.write_str(" }")
            }
            Metric::Temp(value) => {
                let _ = f.write_str("Temp { ");
                let _ = write!(f, "{:?}", value);
                f.write_str(" }")
            }
        }
    }
}

impl Metric {
    pub(crate) fn try_metric(
        dbus_info: &DbusInfo,
        map: &HashMap<String, OwnedValue>,
    ) -> Option<Metric> {
        return Metric::metric_from(dbus_info, map).ok();
    }

    pub(crate) fn metric_from(
        dbus_info: &DbusInfo,
        map: &HashMap<String, OwnedValue>,
    ) -> Result<Metric, GenericError> {
        let value = MetricValue::value_from(dbus_info, map)?;
        match (&value.units).trim() {
            "RPM" => Ok(Metric::Fan(value)),
            "℃" => Ok(Metric::Temp(value)),
            "°C" => Ok(Metric::Temp(value)),
            "℉" => Ok(Metric::Temp(value)),
            "°F" => Ok(Metric::Temp(value)),
            _ => Err(Box::new(UnknownMetricUnitsError {
                units: value.units.clone(),
            })),
        }
    }

    pub fn get_value(&self) -> &MetricValue {
        match self {
            Metric::Fan(value) => value,
            Metric::Temp(value) => value,
        }
    }
}
