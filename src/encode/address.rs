use crate::Address;
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Address {
    /// Encode [server addresses] separated by `;`.
    ///
    /// [server address]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses
    pub fn encode(addresses: &[Address]) -> String {
        let mut iter = addresses.iter();
        if let Some(address) = iter.next() {
            let mut result = address.to_string();
            for address in iter {
                result.push(';');
                result += &address.to_string();
            }
            result
        } else {
            String::new()
        }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Address::Unix(unix) => unix.fmt(f),
            Address::Launchd(launchd) => launchd.fmt(f),
            Address::Tcp(tcp) => tcp.fmt(f),
            Address::NonceTcp(nonce_tcp) => nonce_tcp.fmt(f),
            Address::Unixexec(unixexec) => unixexec.fmt(f),
            Address::Systemd(systemd) => systemd.fmt(f),
            Address::Autolaunch(autolaunch) => autolaunch.fmt(f),
        }
    }
}
