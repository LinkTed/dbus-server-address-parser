#[cfg(target_family = "windows")]
use super::unescape::{unescape, UnescapeError};
use super::{guid::decode_guid, GuidError};
use crate::{Autolaunch, Guid};
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum AutolaunchError {
    #[cfg(target_family = "windows")]
    #[error("Could not unescape: {0}")]
    UnescapeError(#[from] UnescapeError),
    #[error("GUID error: {0}")]
    GuidError(#[from] GuidError),
    #[error("Unknown key")]
    UnknownKey,
    #[cfg(target_family = "windows")]
    #[error("Env is duplicate")]
    ScopeDuplicate,
}

impl Autolaunch {
    #[cfg(target_family = "windows")]
    fn decode_scope(scope_str: &str, scope: &mut Option<String>) -> Result<(), AutolaunchError> {
        if scope.is_none() {
            let scope_str = unescape(scope_str)?;
            *scope = Some(scope_str);
            Ok(())
        } else {
            Err(AutolaunchError::ScopeDuplicate)
        }
    }

    #[cfg(target_family = "windows")]
    fn decode_key_value(
        key_value: &str,
        scope: &mut Option<String>,
        guid: &mut Option<Guid>,
    ) -> Result<(), AutolaunchError> {
        if let Some(scope_str) = key_value.strip_prefix("scope=") {
            Autolaunch::decode_scope(scope_str, scope)
        } else if let Some(guid_str) = key_value.strip_prefix("guid=") {
            decode_guid(guid_str, guid)?;
            Ok(())
        } else {
            Err(AutolaunchError::UnknownKey)
        }
    }

    #[cfg(target_family = "unix")]
    fn decode_key_value(key_value: &str, guid: &mut Option<Guid>) -> Result<(), AutolaunchError> {
        if let Some(guid_str) = key_value.strip_prefix("guid=") {
            decode_guid(guid_str, guid)?;
            Ok(())
        } else {
            Err(AutolaunchError::UnknownKey)
        }
    }
}

impl TryFrom<&str> for Autolaunch {
    type Error = AutolaunchError;

    #[cfg(target_family = "windows")]
    fn try_from(server_address: &str) -> Result<Self, Self::Error> {
        let mut scope = None;
        let mut guid = None;

        if !server_address.is_empty() {
            for key_value in server_address.split(',') {
                Autolaunch::decode_key_value(key_value, &mut scope, &mut guid)?;
            }
        }

        Ok(Autolaunch { scope, guid })
    }

    #[cfg(target_family = "unix")]
    fn try_from(server_address: &str) -> Result<Self, Self::Error> {
        let mut guid = None;

        if !server_address.is_empty() {
            for key_value in server_address.split(',') {
                Autolaunch::decode_key_value(key_value, &mut guid)?;
            }
        }

        Ok(Autolaunch { guid })
    }
}
