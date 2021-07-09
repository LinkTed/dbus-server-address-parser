use crate::decode::unescape::{unescape, UnescapeError};
use crate::Family;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum FamilyError {
    #[error("Could not unescape: {0}")]
    UnescapeError(#[from] UnescapeError),
    #[error("Unknown value")]
    UnknownValue,
}

impl TryFrom<&str> for Family {
    type Error = FamilyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = unescape(value)?;
        match value.as_ref() {
            "ipv4" => Ok(Family::Ipv4),
            "ipv6" => Ok(Family::Ipv6),
            _ => Err(FamilyError::UnknownValue),
        }
    }
}
