use std::sync::{Arc, Mutex, MutexGuard};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Failed to lock a mutex: {}", .msg)]
pub struct LockError {
    pub msg: String,
}

pub(crate) fn lock<T>(lockable: &Arc<Mutex<T>>) -> Result<MutexGuard<T>, LockError> {
    lockable.lock().map_err(|e| LockError {
        msg: e.to_string().clone(),
    })
}
