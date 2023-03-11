use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

use crate::dbus_session::DbusSession;
use crate::observer::Observer;
use crate::simple_types::Temp;

#[derive(Debug)]
pub struct DbusEngine {
    callback: Arc<Mutex<Option<Box<dyn Observer>>>>,
    session: Arc<Mutex<DbusSession>>,
}

impl DbusEngine {
    pub fn set_observer(&mut self, observer: Box<dyn Observer>) {
        *self.callback.borrow_mut().lock().unwrap() = Some(observer);
    }
}

impl DbusEngine {
    pub fn new() -> DbusEngine {
        let session = Arc::new(Mutex::new(DbusSession::new()));

        let write_session = session.clone();
        let callback: Arc<Mutex<Option<Box<dyn Observer>>>> = Arc::new(Mutex::new(None));
        let callable = callback.clone();

        spawn(move || loop {
            {
                let callback = callable.lock().expect("Could not get callback option");
                if write_session.lock().unwrap().update() && callback.is_some() {
                    callback.as_ref().unwrap().on_event();
                }
            }
            sleep(Duration::from_millis(200));
        });

        DbusEngine { callback, session }
    }

    pub fn temp(&self) -> Temp {
        self.session
            .lock()
            .expect("Could not lock Dbus session")
            .temp()
    }
}

impl Default for DbusEngine {
    fn default() -> Self {
        Self::new()
    }
}
