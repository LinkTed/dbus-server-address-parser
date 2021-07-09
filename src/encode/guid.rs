use super::escape::to_hex;
use crate::Guid;

pub(super) fn to_guid(guid: &Guid) -> String {
    let mut result = Vec::with_capacity(guid.len() * 2);
    for b in guid {
        result.push(to_hex((b & 0b1111_0000) >> 4));
        result.push(to_hex(b & 0b0000_1111));
    }
    String::from_utf8(result).unwrap()
}
