use super::{
    guid::{decode_guid, GuidError},
    unescape::{unescape, UnescapeError},
    FamilyError,
};
use crate::{Family, NonceTcp};
use std::convert::TryFrom;
use std::num::ParseIntError;
use thiserror::Error;

/// An enum representing all errors, which can occur during decoding a socket address of a
/// [`nonce-tcp:`]. For example: `nonce-tcp:host=127.0.0.1,port=30900`.
///
/// [`nonce-tcp:`]: https://dbus.freedesktop.org/doc/dbus-specification.html#transports-nonce-tcp-sockets
#[derive(Debug, Clone, Error)]
pub enum NonceTcpError {
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
    #[error("Noncefile is duplicate")]
    NoncefileDuplicate,
    #[error("Family decode error")]
    FamilyError(#[from] FamilyError),
}

impl NonceTcp {
    fn decode_host(&mut self, host: &str) -> Result<(), NonceTcpError> {
        if self.host.is_none() {
            self.host = Some(unescape(host)?);
            Ok(())
        } else {
            Err(NonceTcpError::HostDuplicate)
        }
    }

    fn decode_bind(&mut self, bind: &str) -> Result<(), NonceTcpError> {
        if self.bind.is_none() {
            self.bind = Some(unescape(bind)?);
            Ok(())
        } else {
            Err(NonceTcpError::BindDuplicate)
        }
    }

    fn decode_port(&mut self, port: &str) -> Result<(), NonceTcpError> {
        if self.port.is_none() {
            let port = unescape(port)?.parse()?;
            self.port = Some(port);
            Ok(())
        } else {
            Err(NonceTcpError::PortDuplicate)
        }
    }

    fn decode_family(&mut self, family: &str) -> Result<(), NonceTcpError> {
        if self.family.is_none() {
            self.family = Some(Family::try_from(family)?);
            Ok(())
        } else {
            Err(NonceTcpError::FamilyDuplicate)
        }
    }

    fn decode_noncefile(&mut self, noncefile: &str) -> Result<(), NonceTcpError> {
        if self.noncefile.is_none() {
            let noncefile = unescape(noncefile)?;
            self.noncefile = Some(noncefile);
            Ok(())
        } else {
            Err(NonceTcpError::NoncefileDuplicate)
        }
    }

    fn decode_key_value(&mut self, key_value: &str) -> Result<(), NonceTcpError> {
        if let Some(host) = key_value.strip_prefix("host=") {
            self.decode_host(host)
        } else if let Some(bind) = key_value.strip_prefix("bind=") {
            self.decode_bind(bind)
        } else if let Some(port) = key_value.strip_prefix("port=") {
            self.decode_port(port)
        } else if let Some(family) = key_value.strip_prefix("family=") {
            self.decode_family(family)
        } else if let Some(noncefile) = key_value.strip_prefix("noncefile=") {
            self.decode_noncefile(noncefile)
        } else if let Some(guid) = key_value.strip_prefix("guid=") {
            decode_guid(guid, &mut self.guid)?;
            Ok(())
        } else {
            Err(NonceTcpError::UnknownKey)
        }
    }
}

impl TryFrom<&str> for NonceTcp {
    type Error = NonceTcpError;

    fn try_from(server_address: &str) -> Result<Self, Self::Error> {
        let mut tcp = NonceTcp {
            host: None,
            bind: None,
            port: None,
            family: None,
            noncefile: None,
            guid: None,
        };

        for key_value in server_address.split(',') {
            tcp.decode_key_value(key_value)?;
        }

        Ok(tcp)
    }
}
