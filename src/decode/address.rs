use crate::{Address, Autolaunch, DecodeError, Launchd, NonceTcp, Systemd, Tcp, Unix, Unixexec};
use std::{convert::TryFrom, str::FromStr};

impl Address {
    /// Decode [server addresses] separated by `;`.
    ///
    /// [server address]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses
    pub fn decode(addresses: &str) -> Result<Vec<Address>, DecodeError> {
        let mut result = Vec::new();
        // Split by the ;, because it can have multiple addresses separated by a ;.
        for address in addresses.split(';') {
            let address = Address::try_from(address)?;
            result.push(address);
        }
        Ok(result)
    }
}

impl FromStr for Address {
    type Err = DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Address::try_from(s)
    }
}

impl TryFrom<&str> for Address {
    type Error = DecodeError;

    fn try_from(address: &str) -> Result<Self, Self::Error> {
        if let Some(unix) = address.strip_prefix("unix:") {
            let unix = Unix::try_from(unix)?;
            Ok(Address::Unix(unix))
        } else if let Some(tcp) = address.strip_prefix("tcp:") {
            let tcp = Tcp::try_from(tcp)?;
            Ok(Address::Tcp(tcp))
        } else if let Some(launchd) = address.strip_prefix("launchd:") {
            let launchd = Launchd::try_from(launchd)?;
            Ok(Address::Launchd(launchd))
        } else if let Some(nonce_tcp) = address.strip_prefix("nonce-tcp:") {
            let nonce_tcp = NonceTcp::try_from(nonce_tcp)?;
            Ok(Address::NonceTcp(nonce_tcp))
        } else if let Some(unixexec) = address.strip_prefix("unixexec:") {
            let unixexec = Unixexec::try_from(unixexec)?;
            Ok(Address::Unixexec(unixexec))
        } else if let Some(systemd) = address.strip_prefix("systemd:") {
            let systemd = Systemd::try_from(systemd)?;
            Ok(Address::Systemd(systemd))
        } else if let Some(autolaunch) = address.strip_prefix("autolaunch:") {
            let autolaunch = Autolaunch::try_from(autolaunch)?;
            Ok(Address::Autolaunch(autolaunch))
        } else {
            Err(DecodeError::UnknownType)
        }
    }
}
