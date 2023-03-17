use std::sync::{Arc, Mutex, MutexGuard};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Failed to lock a mutex: {}", .msg)]
pub struct LockError {
    pub msg: String,
}

pub(crate) fn lock<T>(unlockable: &Arc<Mutex<T>>) -> Result<MutexGuard<T>, LockError> {
    unlockable.lock().map_err(|e| LockError {
        msg: e.to_string().clone(),
    })
}
