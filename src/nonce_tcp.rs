use crate::{Family, Guid};

/// This represents a DBus server address with the prefix [`nonce-tcp:`].
///
/// [`nonce-tcp:`]: https://dbus.freedesktop.org/doc/dbus-specification.html#transports-nonce-tcp-sockets
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonceTcp {
    /// DNS name or IP address.
    pub host: Option<String>,
    /// The interface on which the server will listen to.
    pub bind: Option<String>,
    /// The TCP port the server will open or the client will connect to.
    pub port: Option<u16>,
    /// The type of socket family.
    pub family: Option<Family>,
    /// File location containing the secret.
    pub noncefile: Option<String>,
    /// The GUID of the Address.
    pub guid: Option<Guid>,
}

impl NonceTcp {
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
