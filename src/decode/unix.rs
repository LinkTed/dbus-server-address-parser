use super::{
    unescape::{unescape, UnescapeError},
    GuidError,
};
use crate::{Unix, UnixType};
use std::convert::TryFrom;
use thiserror::Error;

use super::guid::decode_guid;

#[derive(Error, Debug, Clone)]
pub enum UnixError {
    #[error("Could not unescape: {0}")]
    UnescapeError(#[from] UnescapeError),
    #[error("GUID error: {0}")]
    GuidError(#[from] GuidError),
    #[error("Unknown key")]
    UnknownKey,
    #[error("Unknown runtime")]
    UnknownRuntime,
    #[error("Type is duplicate")]
    TypeDuplicate,
    #[error("Type is missing")]
    TypeMissing,
}

impl Unix {
    fn decode_type(type_str: &str, r#type: &mut Option<UnixType>) -> Result<(), UnixError> {
        if r#type.is_none() {
            *r#type = Some(UnixType::try_from(type_str)?);
            Ok(())
        } else {
            Err(UnixError::TypeDuplicate)
        }
    }
}

impl TryFrom<&str> for Unix {
    type Error = UnixError;

    fn try_from(server_address: &str) -> Result<Self, Self::Error> {
        let mut r#type = None;
        let mut guid = None;

        for key_value in server_address.split(',') {
            if let Some(guid_str) = key_value.strip_prefix("guid=") {
                decode_guid(guid_str, &mut guid)?;
            } else {
                Unix::decode_type(key_value, &mut r#type)?;
            }
        }

        if let Some(r#type) = r#type {
            Ok(Unix { r#type, guid })
        } else {
            Err(UnixError::TypeMissing)
        }
    }
}

impl UnixType {
    fn decode_path(path: &str) -> Result<UnixType, UnixError> {
        let path = UnixType::Path(unescape(path)?);
        Ok(path)
    }

    fn decode_dir(dir: &str) -> Result<UnixType, UnixError> {
        let dir = UnixType::Dir(unescape(dir)?);
        Ok(dir)
    }

    fn decode_tmpdir(tmpdir: &str) -> Result<UnixType, UnixError> {
        let tmpdir = UnixType::Tmpdir(unescape(tmpdir)?);
        Ok(tmpdir)
    }

    fn decode_abstract(abstract_: &str) -> Result<UnixType, UnixError> {
        let abstract_ = UnixType::Abstract(unescape(abstract_)?);
        Ok(abstract_)
    }

    fn decode_runtime(runtime: &str) -> Result<UnixType, UnixError> {
        let runtime = unescape(runtime)?;
        if &runtime == "yes" {
            Ok(UnixType::Runtime)
        } else {
            Err(UnixError::UnknownRuntime)
        }
    }
}

impl TryFrom<&str> for UnixType {
    type Error = UnixError;

    fn try_from(server_address: &str) -> Result<Self, Self::Error> {
        if let Some(path) = server_address.strip_prefix("path=") {
            UnixType::decode_path(path)
        } else if let Some(dir) = server_address.strip_prefix("dir=") {
            UnixType::decode_dir(dir)
        } else if let Some(tmpdir) = server_address.strip_prefix("tmpdir=") {
            UnixType::decode_tmpdir(tmpdir)
        } else if let Some(abstract_) = server_address.strip_prefix("abstract=") {
            UnixType::decode_abstract(abstract_)
        } else if let Some(runtime) = server_address.strip_prefix("runtime=") {
            UnixType::decode_runtime(runtime)
        } else {
            Err(UnixError::UnknownKey)
        }
    }
}
