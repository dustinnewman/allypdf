// Whitespace characters
pub const NULL_BYTE: Byte = b'\0';
pub const SPACE: Byte = b'\x20';
pub const CARRIAGE_RETURN: Byte = b'\r';
pub const BACK_SPACE: Byte = b'\x08';
pub const FORM_FEED: Byte = b'\x0C';
pub const LINE_FEED: Byte = b'\n';
pub const TAB: Byte = b'\t';
// Delimiter characters
pub const LPAREN: Byte = b'\x28'; // (
pub const RPAREN: Byte = b'\x29'; // )
pub const LTHAN: Byte = b'\x3C'; // <
pub const RTHAN: Byte = b'\x3E'; // >
pub const LBRACKET: Byte = b'\x5B'; // [
pub const RBRACKET: Byte = b'\x5D'; // ]
pub const LBRACE: Byte = b'\x7B'; // {
pub const RBRACE: Byte = b'\x7D'; // }
pub const FSLASH: Byte = b'\x2F'; // /
pub const PERCENT: Byte = b'%';
// Other characters
pub const BSLASH: Byte = b'\x5C'; // \
pub const POUND: Byte = b'\x23'; // #
pub const PERIOD: Byte = b'.';
pub const TILDE: Byte = b'~';

pub type Byte = u8;

pub fn is_whitespace(byte: Byte) -> bool {
    match byte {
        SPACE |
        CARRIAGE_RETURN |
        LINE_FEED |
        TAB |
        NULL_BYTE |
        FORM_FEED => true,
        _ => false,
    }
}

pub fn isnt_whitespace(byte: Byte) -> bool {
    !is_whitespace(byte)
}

pub fn is_delimiter(byte: Byte) -> bool {
    match byte {
        LPAREN |
        RPAREN |
        LTHAN |
        RTHAN |
        LBRACKET |
        RBRACKET |
        LBRACE |
        RBRACE |
        FSLASH |
        PERCENT => true,
        _ => false,
    }
}

pub fn is_regular(byte: Byte) -> bool {
    !(is_whitespace(byte) || is_delimiter(byte))
}

pub fn is_newline(byte: Byte) -> bool {
    byte == LINE_FEED
}

pub fn is_comment(byte: Byte) -> bool {
    byte == PERCENT
}

pub fn is_whitespace_or_comment(byte: Byte) -> bool {
    is_whitespace(byte) || is_comment(byte)
}

pub fn is_octal(byte: Byte) -> bool {
    match byte {
        b'0'..=b'7' => true,
        _ => false
    }
}

pub fn is_decimal(byte: Byte) -> bool {
    match byte {
        b'0'..=b'9' => true,
        _ => false
    }
}

pub fn is_hexadecimal(byte: Byte) -> bool {
    match byte {
        b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => true,
        _ => false,
    }
}

pub fn byte_to_numeric(byte: Byte, radix: u8) -> Option<u8> {
    if radix == 10 {
        if is_decimal(byte) {
            Some(byte - b'0')
        } else {
            None
        }
    } else if radix == 16 {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(10 + (byte - b'a')),
            b'A'..=b'F' => Some(10 + (byte - b'A')),
            _ => None
        }
    } else if radix == 8 {
        if is_octal(byte) {
            Some(byte - b'0')
        } else {
            None
        }
    } else {
        None
    }
}

pub fn slice_to_numeric(slice: &[Byte], radix: u8) -> Option<u8> {
    let mut result = 0;
    for byte in slice {
        match byte_to_numeric(*byte, radix) {
            Some(x) => {
                result *= radix;
                result += x;
            }
            None => return None,
        }
    }
    Some(result)
}
