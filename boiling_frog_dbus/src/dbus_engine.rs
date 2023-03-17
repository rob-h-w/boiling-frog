use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::dbus_session::DbusSession;
use crate::mutex_helpers::lock;
use crate::simple_types::{Fan, Temp};

#[derive(Debug)]
pub struct DbusEngine {
    session: Arc<Mutex<DbusSession>>,
}

impl DbusEngine {
    pub fn new() -> Result<DbusEngine, Box<dyn Error + Send + Sync>> {
        let session = Arc::new(Mutex::new(DbusSession::new()));
        DbusSession::run(&session)?;

        Ok(DbusEngine { session })
    }

    pub fn fan(&self) -> Result<Fan, Box<dyn Error + Send + Sync>> {
        Ok(lock(&self.session)?.fan())
    }

    pub fn temp(&self) -> Result<Temp, Box<dyn Error + Send + Sync>> {
        Ok(lock(&self.session)?.temp())
    }
}
