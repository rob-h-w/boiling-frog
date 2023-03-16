use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

use crate::dbus_session::DbusSession;
use crate::simple_types::{Fan, Temp};

#[derive(Debug)]
pub struct DbusEngine {
    session: Arc<Mutex<DbusSession>>,
}

impl DbusEngine {
    pub fn new() -> DbusEngine {
        let session = Arc::new(Mutex::new(DbusSession::new()));

        let write_session = session.clone();

        spawn(move || loop {
            write_session.lock().unwrap().update();
            sleep(Duration::from_millis(2000));
        });

        DbusEngine { session }
    }

    pub fn fan(&self) -> Fan {
        self.session
            .lock()
            .expect("Can lock Dbus session")
            .fan()
    }

    pub fn temp(&self) -> Temp {
        self.session
            .lock()
            .expect("Can lock Dbus session")
            .temp()
    }
}

impl Default for DbusEngine {
    fn default() -> Self {
        Self::new()
    }
}
