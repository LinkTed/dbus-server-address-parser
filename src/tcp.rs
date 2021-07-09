use crate::{Family, Guid};

/// This represents a DBus server address with the prefix [`tcp:`].
///
/// [`tcp:`]: https://dbus.freedesktop.org/doc/dbus-specification.html#transports-tcp-sockets
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tcp {
    /// DNS name or IP address.
    pub host: Option<String>,
    /// The interface on which the server will listen to.
    pub bind: Option<String>,
    /// The TCP port the server will open or the client will connect to.
    pub port: Option<u16>,
    /// Is the family key/value pair.
    pub family: Option<Family>,
    /// The GUID of the Address.
    pub guid: Option<Guid>,
}

impl Tcp {
    pub fn is_connectable(&self) -> bool {
        if let Some(port) = self.port {
            port != 0 && self.host.is_some()
        } else {
            false
        }
    }

    pub fn is_listenable(&self) -> bool {
        true
    }
}
