use super::{
    guid::decode_guid,
    unescape::{unescape, UnescapeError},
    GuidError,
};
use crate::{Guid, Launchd};
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum LaunchdError {
    #[error("Could not unescape: {0}")]
    UnescapeError(#[from] UnescapeError),
    #[error("GUID error: {0}")]
    GuidError(#[from] GuidError),
    #[error("Unknown key")]
    UnknownKey,
    #[error("Env is duplicate")]
    EnvDuplicate,
    #[error("Env is missing")]
    EnvMissing,
}

impl Launchd {
    fn decode_env(env_str: &str, env: &mut Option<String>) -> Result<(), LaunchdError> {
        if env.is_none() {
            let env_str = unescape(env_str)?;
            *env = Some(env_str);
            Ok(())
        } else {
            Err(LaunchdError::EnvDuplicate)
        }
    }

    fn decode_key_value(
        key_value: &str,
        env: &mut Option<String>,
        guid: &mut Option<Guid>,
    ) -> Result<(), LaunchdError> {
        if let Some(env_str) = key_value.strip_prefix("env=") {
            Launchd::decode_env(env_str, env)
        } else if let Some(guid_str) = key_value.strip_prefix("guid=") {
            decode_guid(guid_str, guid)?;
            Ok(())
        } else {
            Err(LaunchdError::UnknownKey)
        }
    }
}

impl TryFrom<&str> for Launchd {
    type Error = LaunchdError;

    fn try_from(server_address: &str) -> Result<Self, Self::Error> {
        let mut env = None;
        let mut guid = None;

        for key_value in server_address.split(',') {
            Launchd::decode_key_value(key_value, &mut env, &mut guid)?;
        }

        if let Some(env) = env {
            Ok(Launchd { env, guid })
        } else {
            Err(LaunchdError::EnvMissing)
        }
    }
}
