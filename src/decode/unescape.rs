use std::fmt::{Display, Formatter, Result as FmtResult};
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum UnescapeError {
    #[error("Unknown byte: 0x{0:02x}")]
    UnknownByte(u8),
    #[error("State is not normal: {0}")]
    UnescapeState(UnescapeState),
    #[error("FromUtf8Error: {0}")]
    FromUtf8Error(#[from] FromUtf8Error),
}

/// The states of the unescape machine.
#[derive(Debug, Clone)]
pub enum UnescapeState {
    Normal,
    FirstHex,
    SecondHex(u8),
}

impl Display for UnescapeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            UnescapeState::Normal => write!(f, "Normal"),
            UnescapeState::FirstHex => write!(f, "First Hex"),
            UnescapeState::SecondHex(h) => write!(f, "Second Hex: 0x{:02x}", h),
        }
    }
}

#[inline]
fn is_optionally_escaped(c: u8) -> bool {
    // *?
    // [-0-9A-Za-z_/.\]
    c.is_ascii_alphanumeric() || c == b'-' || c == b'_' || c == b'/' || c == b'.' || c == b'\\'
}

pub(super) fn to_hex(c: u8) -> Result<u8, UnescapeError> {
    match c {
        b'0' => Ok(0),
        b'1' => Ok(1),
        b'2' => Ok(2),
        b'3' => Ok(3),
        b'4' => Ok(4),
        b'5' => Ok(5),
        b'6' => Ok(6),
        b'7' => Ok(7),
        b'8' => Ok(8),
        b'9' => Ok(9),
        b'a' | b'A' => Ok(10),
        b'b' | b'B' => Ok(11),
        b'c' | b'C' => Ok(12),
        b'd' | b'D' => Ok(13),
        b'e' | b'E' => Ok(14),
        b'f' | b'F' => Ok(15),
        c => Err(UnescapeError::UnknownByte(c)),
    }
}

pub(super) fn unescape(string: &str) -> Result<String, UnescapeError> {
    let mut result: Vec<u8> = Vec::with_capacity(string.len());
    let mut decode_state = UnescapeState::Normal;
    for c in string.bytes() {
        match decode_state {
            UnescapeState::Normal => {
                if is_optionally_escaped(c) {
                    result.push(c);
                } else if c == b'%' {
                    decode_state = UnescapeState::FirstHex;
                } else {
                    return Err(UnescapeError::UnknownByte(c));
                }
            }
            UnescapeState::FirstHex => {
                let first_hex = to_hex(c)?;
                decode_state = UnescapeState::SecondHex(first_hex << 4);
            }
            UnescapeState::SecondHex(first_hex) => {
                let second_hex = to_hex(c)?;
                result.push(first_hex | second_hex);
                decode_state = UnescapeState::Normal;
            }
        }
    }

    if let UnescapeState::Normal = decode_state {
        let string = String::from_utf8(result)?;
        Ok(string)
    } else {
        Err(UnescapeError::UnescapeState(decode_state))
    }
}
