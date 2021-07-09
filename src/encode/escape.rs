#[inline]
fn is_optionally_escaped(c: u8) -> bool {
    // *?
    // [-0-9A-Za-z_/.\]
    c.is_ascii_alphanumeric() || c == b'-' || c == b'_' || c == b'/' || c == b'.' || c == b'\\'
}

pub(super) fn to_hex(b: u8) -> u8 {
    match b {
        0 => b'0',
        1 => b'1',
        2 => b'2',
        3 => b'3',
        4 => b'4',
        5 => b'5',
        6 => b'6',
        7 => b'7',
        8 => b'8',
        9 => b'9',
        10 => b'a',
        11 => b'b',
        12 => b'c',
        13 => b'd',
        14 => b'e',
        15 => b'f',
        b => panic!("This should not happend: {}", b),
    }
}

fn add_hex(s: &mut Vec<u8>, b: u8) {
    s.push(b'%');
    s.push(to_hex((b & 0b1111_0000) >> 4));
    s.push(to_hex(b & 0b0000_1111));
}

pub(super) fn escape(path: &str) -> String {
    let mut result = Vec::with_capacity(path.len());
    for c in path.bytes() {
        if is_optionally_escaped(c) {
            result.push(c);
        } else {
            add_hex(&mut result, c);
        }
    }
    String::from_utf8(result).unwrap()
}
