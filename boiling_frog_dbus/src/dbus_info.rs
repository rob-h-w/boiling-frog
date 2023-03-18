use std::fmt::{Display, Formatter};
use zbus::names::OwnedInterfaceName;
use zbus::zvariant::OwnedObjectPath;

#[derive(Clone, Debug)]
pub(crate) struct DbusInfo {
    pub interface_name: String,
    pub path: String,
}

impl Display for DbusInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_str("DbusInfo { ");
        let _ = write!(f, "{:?}", self);
        f.write_str(" }")
    }
}

impl DbusInfo {
    pub(crate) fn new(interface_name: &OwnedInterfaceName, path: &OwnedObjectPath) -> DbusInfo {
        DbusInfo {
            interface_name: interface_name.to_string(),
            path: path.to_string(),
        }
    }
}
