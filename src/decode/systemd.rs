use super::{guid::decode_guid, GuidError};
use crate::Systemd;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum SystemdError {
    #[error("GUID error: {0}")]
    GuidError(#[from] GuidError),
    #[error("Unknown key")]
    UnknownKey,
}

impl Systemd {
    fn decode_key_value(&mut self, key_value: &str) -> Result<(), SystemdError> {
        if let Some(guid) = key_value.strip_prefix("guid=") {
            decode_guid(guid, &mut self.guid)?;
            Ok(())
        } else {
            Err(SystemdError::UnknownKey)
        }
    }
}

impl TryFrom<&str> for Systemd {
    type Error = SystemdError;

    fn try_from(server_address: &str) -> Result<Self, Self::Error> {
        let mut systemd = Systemd { guid: None };
        if !server_address.is_empty() {
            for key_value in server_address.split(',') {
                systemd.decode_key_value(key_value)?;
            }
        }
        Ok(systemd)
    }
}
