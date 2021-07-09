use super::guid::{decode_guid, GuidError};
use crate::decode::unescape::{unescape, UnescapeError};
use crate::decode::FamilyError;
use crate::{Family, Tcp};
use std::convert::TryFrom;
use std::num::ParseIntError;
use thiserror::Error;

/// An enum representing all errors, which can occur during decoding a socket address of a
/// [`tcp:`]. For example: `tcp:host=127.0.0.1,port=30900`.
///
/// [`tcp:`]: https://dbus.freedesktop.org/doc/dbus-specification.html#transports-tcp-sockets
#[derive(Debug, Clone, Error)]
pub enum TcpError {
    #[error("Could not unescape: {0}")]
    UnescapeError(#[from] UnescapeError),
    #[error("Port parse: {0}")]
    PortParseError(#[from] ParseIntError),
    #[error("GUID error: {0}")]
    GuidError(#[from] GuidError),
    #[error("Unknown key")]
    UnknownKey,
    #[error("Host is duplicate")]
    HostDuplicate,
    #[error("Bind is duplicate")]
    BindDuplicate,
    #[error("Port is duplicate")]
    PortDuplicate,
    #[error("Family is duplicate")]
    FamilyDuplicate,
    #[error("Family decode error")]
    FamilyError(#[from] FamilyError),
}

impl Tcp {
    fn decode_host(&mut self, host: &str) -> Result<(), TcpError> {
        if self.host.is_none() {
            self.host = Some(unescape(host)?);
            Ok(())
        } else {
            Err(TcpError::HostDuplicate)
        }
    }

    fn decode_bind(&mut self, bind: &str) -> Result<(), TcpError> {
        if self.bind.is_none() {
            self.bind = Some(unescape(bind)?);
            Ok(())
        } else {
            Err(TcpError::BindDuplicate)
        }
    }

    fn decode_port(&mut self, port: &str) -> Result<(), TcpError> {
        if self.port.is_none() {
            let port = unescape(port)?.parse()?;
            self.port = Some(port);
            Ok(())
        } else {
            Err(TcpError::PortDuplicate)
        }
    }

    fn decode_family(&mut self, family: &str) -> Result<(), TcpError> {
        if self.family.is_none() {
            self.family = Some(Family::try_from(family)?);
            Ok(())
        } else {
            Err(TcpError::FamilyDuplicate)
        }
    }

    fn decode_key_value(&mut self, key_value: &str) -> Result<(), TcpError> {
        if let Some(host) = key_value.strip_prefix("host=") {
            self.decode_host(host)
        } else if let Some(bind) = key_value.strip_prefix("bind=") {
            self.decode_bind(bind)
        } else if let Some(port) = key_value.strip_prefix("port=") {
            self.decode_port(port)
        } else if let Some(family) = key_value.strip_prefix("family=") {
            self.decode_family(family)
        } else if let Some(guid) = key_value.strip_prefix("guid=") {
            decode_guid(guid, &mut self.guid)?;
            Ok(())
        } else {
            Err(TcpError::UnknownKey)
        }
    }
}

impl TryFrom<&str> for Tcp {
    type Error = TcpError;

    fn try_from(server_address: &str) -> Result<Self, Self::Error> {
        let mut tcp = Tcp {
            host: None,
            bind: None,
            port: None,
            family: None,
            guid: None,
        };

        for key_value in server_address.split(',') {
            tcp.decode_key_value(key_value)?;
        }

        Ok(tcp)
    }
}
