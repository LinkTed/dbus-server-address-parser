use super::{
    guid::to_guid,
    unescape::{unescape, UnescapeError},
    GuidError,
};
use crate::{Guid, Unixexec};
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum UnixexecError {
    #[error("Could not unescape: {0}")]
    UnescapeError(#[from] UnescapeError),
    #[error("GUID error: {0}")]
    GuidError(#[from] GuidError),
    #[error("Unknown key")]
    UnknownKey,
    #[error("Missing path value")]
    MissingPath,
}

impl Unixexec {
    fn decode_value(server_address: &str, key: &str) -> Result<Option<String>, UnixexecError> {
        for key_value in server_address.split(',') {
            if let Some(value) = key_value.strip_prefix(key) {
                let value = unescape(value)?;
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    fn decode_path(server_address: &str) -> Result<String, UnixexecError> {
        if let Some(path) = Unixexec::decode_value(server_address, "path=")? {
            Ok(path)
        } else {
            Err(UnixexecError::MissingPath)
        }
    }

    fn decode_arg(server_address: &str, i: usize) -> Result<Option<String>, UnixexecError> {
        let key = format!("argv{}=", i);
        Unixexec::decode_value(server_address, &key)
    }

    fn decode_guid(server_address: &str) -> Result<Option<Guid>, UnixexecError> {
        if let Some(guid) = Unixexec::decode_value(server_address, "guid=")? {
            let guid = to_guid(&guid)?;
            Ok(Some(guid))
        } else {
            Ok(None)
        }
    }

    fn check_argv(&self, key: &str) -> Result<(), UnixexecError> {
        if let Some(i) = key.strip_prefix("argv") {
            match i.parse::<usize>() {
                Ok(i) => {
                    if i <= self.argv.len() {
                        Ok(())
                    } else {
                        Err(UnixexecError::UnknownKey)
                    }
                }
                Err(_) => Err(UnixexecError::UnknownKey),
            }
        } else {
            Err(UnixexecError::UnknownKey)
        }
    }

    fn check_keys(&self, server_address: &str) -> Result<(), UnixexecError> {
        for key_value in server_address.split(',') {
            if let Some((key, _)) = key_value.split_once('=') {
                if key != "path" && key != "guid" {
                    self.check_argv(key)?;
                }
            } else {
                return Err(UnixexecError::UnknownKey);
            }
        }
        Ok(())
    }
}

impl TryFrom<&str> for Unixexec {
    type Error = UnixexecError;

    fn try_from(server_address: &str) -> Result<Self, Self::Error> {
        let path = Unixexec::decode_path(server_address)?;

        let argv0 = Unixexec::decode_arg(server_address, 0)?;

        let mut argv = Vec::new();
        let mut i = 1usize;
        while let Some(value) = Unixexec::decode_arg(server_address, i)? {
            argv.push(value);
            i += 1;
        }

        let guid = Unixexec::decode_guid(server_address)?;

        let unixexec = Unixexec {
            path,
            argv0,
            argv,
            guid,
        };

        unixexec.check_keys(server_address)?;

        Ok(unixexec)
    }
}
