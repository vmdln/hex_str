pub fn from_hex([a, b]: [u8; 2]) -> Option<u8> {
    let parse_hex = |v: u8| -> Option<u8> {
        let ret = match v {
            b'0'..=b'9' => v - b'0',
            b'a'..=b'f' => v - b'a' + 10,
            b'A'..=b'F' => v - b'A' + 10,
            _ => return None,
        };

        Some(ret)
    };

    let a = parse_hex(a)?;
    let b = parse_hex(b)?;

    Some(a << 4 | b)
}

pub fn from_hex_lower([a, b]: [u8; 2]) -> Option<u8> {
    let parse_hex = |v: u8| -> Option<u8> {
        let ret = match v {
            b'0'..=b'9' => v - b'0',
            b'a'..=b'f' => v - b'a' + 10,
            _ => return None,
        };

        Some(ret)
    };

    let a = parse_hex(a)?;
    let b = parse_hex(b)?;

    Some(a << 4 | b)
}

pub fn from_hex_upper([a, b]: [u8; 2]) -> Option<u8> {
    let parse_hex = |v: u8| -> Option<u8> {
        let ret = match v {
            b'0'..=b'9' => v - b'0',
            b'A'..=b'F' => v - b'A' + 10,
            _ => return None,
        };

        Some(ret)
    };

    let a = parse_hex(a)?;
    let b = parse_hex(b)?;

    Some(a << 4 | b)
}

pub fn to_hex_lower(v: u8) -> [u8; 2] {
    let helper = |v: u8| -> u8 {
        match v {
            v @ 0..=9 => v + b'0',
            v @ 10..=15 => v - 10 + b'a',
            _ => unreachable!(),
        }
    };

    let a = (v & 0xf0) >> 4;
    let b = v & 0x0f;

    [helper(a), helper(b)]
}

pub fn to_hex_upper(v: u8) -> [u8; 2] {
    let helper = |v: u8| -> u8 {
        match v {
            v @ 0..=9 => v + b'0',
            v @ 10..=15 => v - 10 + b'A',
            _ => unreachable!(),
        }
    };

    let a = (v & 0xf0) >> 4;
    let b = v & 0x0f;

    [helper(a), helper(b)]
}
