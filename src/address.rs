use crate::{Autolaunch, Launchd, NonceTcp, Systemd, Tcp, Unix, Unixexec};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Addresses(pub Vec<Address>);

/// This represents a DBus [server address].
///
/// [server address]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Address {
    Unix(Unix),
    Launchd(Launchd),
    Tcp(Tcp),
    NonceTcp(NonceTcp),
    Unixexec(Unixexec),
    Systemd(Systemd),
    Autolaunch(Autolaunch),
}

impl Address {
    pub fn is_connectable(&self) -> bool {
        match self {
            Address::Unix(unix) => unix.is_connectable(),
            Address::Launchd(launchd) => launchd.is_connectable(),
            Address::Tcp(tcp) => tcp.is_connectable(),
            Address::NonceTcp(nonce_tcp) => nonce_tcp.is_connectable(),
            Address::Unixexec(unixexec) => unixexec.is_connectable(),
            Address::Systemd(systemd) => systemd.is_connectable(),
            Address::Autolaunch(autolaunch) => autolaunch.is_connectable(),
        }
    }

    pub fn is_listenable(&self) -> bool {
        match self {
            Address::Unix(unix) => unix.is_listenable(),
            Address::Launchd(launchd) => launchd.is_listenable(),
            Address::Tcp(tcp) => tcp.is_listenable(),
            Address::NonceTcp(nonce_tcp) => nonce_tcp.is_listenable(),
            Address::Unixexec(unixexec) => unixexec.is_listenable(),
            Address::Systemd(systemd) => systemd.is_listenable(),
            Address::Autolaunch(autolaunch) => autolaunch.is_listenable(),
        }
    }
}
