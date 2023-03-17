use std::error::Error;

pub mod dbus_engine;
pub mod mutex_helpers;
pub mod simple_types;
pub type GenericError = Box<dyn Error + Send + Sync>;

mod dbus_session;
mod max_fan;
mod max_temp;
