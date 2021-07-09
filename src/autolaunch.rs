use crate::Guid;

/// This represents a DBus server address with the prefix [`autolaunch:`].
///
/// [`autolaunch:`]: https://dbus.freedesktop.org/doc/dbus-specification.html#meta-transports-autolaunch
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Autolaunch {
    /// Scope of autolaunch (Windows only).
    #[cfg(target_family = "windows")]
    pub scope: Option<String>,
    /// The GUID of the Address.
    pub guid: Option<Guid>,
}

impl Autolaunch {
    pub fn is_connectable(&self) -> bool {
        true
    }

    #[cfg(target_family = "windows")]
    pub fn is_listenable(&self) -> bool {
        true
    }

    #[cfg(target_family = "unix")]
    pub fn is_listenable(&self) -> bool {
        false
    }
}
