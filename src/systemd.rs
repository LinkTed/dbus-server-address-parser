use crate::Guid;

/// This represents a DBus server address with the prefix [`systemd`].
///
/// [`systemd:`]: https://dbus.freedesktop.org/doc/dbus-specification.html#transports-systemd
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Systemd {
    /// The GUID of the Address.
    pub guid: Option<Guid>,
}

impl Systemd {
    pub fn is_connectable(&self) -> bool {
        false
    }

    pub fn is_listenable(&self) -> bool {
        true
    }
}
