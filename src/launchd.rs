use crate::Guid;

/// This represents a DBus server address with the prefix [`launchd:`].
///
/// [`launchd:`]: https://dbus.freedesktop.org/doc/dbus-specification.html#transports-launchd
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Launchd {
    /// Path of the unix domain socket for the launchd created dbus-daemon.
    pub env: String,
    /// The GUID of the Address.
    pub guid: Option<Guid>,
}

impl Launchd {
    pub fn is_connectable(&self) -> bool {
        true
    }

    pub fn is_listenable(&self) -> bool {
        true
    }
}
