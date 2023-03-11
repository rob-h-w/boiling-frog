use std::fmt::Debug;

pub trait Observer: Debug + Send + Sync {
    fn on_event(&self);
}
