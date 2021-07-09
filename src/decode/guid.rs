use super::unescape::{to_hex, unescape, UnescapeError};
use crate::Guid;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum GuidError {
    #[error("Could not unescape: {0}")]
    UnescapeError(#[from] UnescapeError),
    #[error("GUID has the wrong lenght: expected 32 got {0}")]
    WrongLen(usize),
    #[error("GUID is duplicate")]
    GuidDuplicate,
}

pub(super) fn to_guid(guid: &str) -> Result<Guid, GuidError> {
    let guid = unescape(guid)?;
    let guid_len = guid.len();

    if guid_len != 32 {
        return Err(GuidError::WrongLen(guid_len));
    }

    let mut result = [0; 16];

    for (i, b) in guid.as_bytes().chunks(2).enumerate() {
        result[i] = to_hex(b[0])? << 4 | to_hex(b[1])?;
    }

    Ok(result)
}

pub(super) fn decode_guid(guid_str: &str, guid: &mut Option<Guid>) -> Result<(), GuidError> {
    if guid.is_none() {
        *guid = Some(to_guid(guid_str)?);
        Ok(())
    } else {
        Err(GuidError::GuidDuplicate)
    }
}
