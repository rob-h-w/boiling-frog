use thiserror::Error;

#[derive(Error, Debug)]
#[error("Unrecognized metric unit type: {}", .units)]
pub struct UnknownMetricUnitsError {
    pub units: String,
}

#[derive(Error, Debug)]
#[error("Metric is missing a property")]
pub struct MissingPropertyError {}

#[derive(Error, Debug)]
#[error("Metric metric had an unexpected type")]
pub struct BadPropertyTypeError {}
