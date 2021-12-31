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
pub const SQUOTE: Byte = b'\x27'; // '
pub const DQUOTE: Byte = b'\x22'; // "

pub type Byte = u8;

#[macro_export]
macro_rules! integer {
    ($int:expr) => (
        Object::Integer($int)
    );
}

#[macro_export]
macro_rules! real {
    ($real:expr) => (
        Object::Real($real)
    );
}

#[macro_export]
macro_rules! boolean {
    ($boolean:expr) => (
        Object::Boolean($boolean)
    );
}

#[macro_export]
macro_rules! name {
    ($str:expr) => (
        Object::Name($str.as_bytes().to_vec())
    );
}

#[macro_export]
macro_rules! array {
    () => (
        Object::Array(vec![])
    );
    ($($elem:expr),+ $(,)?) => (
        Object::Array(vec![$($elem),+])
    );
}

#[macro_export]
macro_rules! dict {
    ($($key:expr => $val:expr),*) => (
        Object::Dictionary(BTreeMap::from([
            $(($key.to_vec(), $val),)*
        ]))
    );
}

#[macro_export]
macro_rules! indirect_reference {
    ($object_number:expr) => (
        Object::IndirectReference(IndirectReference {
            object_number: $object_number,
            generation_number: 0,
        })
    );
    ($object_number:expr, $generation_number:expr) => (
        Object::IndirectReference(IndirectReference {
            object_number: $object_number,
            generation_number: $generation_number,
        })
    )
}

pub fn is_whitespace(byte: Byte) -> bool {
    match byte {
        SPACE | CARRIAGE_RETURN | LINE_FEED | TAB | NULL_BYTE | FORM_FEED => true,
        _ => false,
    }
}

pub fn isnt_whitespace(byte: Byte) -> bool {
    !is_whitespace(byte)
}

pub fn is_delimiter(byte: Byte) -> bool {
    match byte {
        LPAREN | RPAREN | LTHAN | RTHAN | LBRACKET | RBRACKET | LBRACE | RBRACE | FSLASH
        | PERCENT => true,
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
        _ => false,
    }
}

pub fn is_decimal(byte: Byte) -> bool {
    match byte {
        b'0'..=b'9' => true,
        _ => false,
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
            _ => None,
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

pub fn literal_string_to_string(literal: &[u8]) -> Option<Vec<u8>> {
    let mut vec = vec![];
    let mut original_pos = 0;
    let original_len = literal.len();
    while original_pos < original_len {
        let original_curr = literal[original_pos];
        if original_curr == BSLASH {
            let next = literal[original_pos + 1];
            match next {
                b'n' => vec.push(LINE_FEED),
                b'r' => vec.push(CARRIAGE_RETURN),
                b't' => vec.push(TAB),
                b'b' => vec.push(BACK_SPACE),
                b'f' => vec.push(FORM_FEED),
                LPAREN => vec.push(LPAREN),
                RPAREN => vec.push(RPAREN),
                BSLASH => vec.push(BSLASH),
                c if is_octal(c) => {
                    let mut octal_len = 1;
                    if original_pos + 2 < original_len && is_octal(literal[original_pos + 2]) {
                        octal_len += 1;
                        if original_pos + 3 < original_len && is_octal(literal[original_pos + 3]) {
                            octal_len += 1;
                        }
                    }
                    let slice = &literal[original_pos + 1..original_pos + 1 + octal_len];
                    let code = slice_to_numeric(slice, 8)?;
                    vec.push(code);
                    original_pos += octal_len - 1;
                }
                _ => (),
            };
            original_pos += 2;
        } else {
            vec.push(original_curr);
            original_pos += 1;
        }
    }
    Some(vec)
}

pub fn hex_string_to_string(hex_string: &[u8]) -> Option<Vec<u8>> {
    let mut vec = vec![];
    let mut hex_pos = 0;
    let hex_len = hex_string.len();
    while hex_pos + 1 < hex_len {
        let first = hex_string[hex_pos];
        let second = hex_string[hex_pos + 1];
        let slice = [first, second];
        let code = slice_to_numeric(&slice, 16)?;
        vec.push(code);
        hex_pos += 2;
    }
    if hex_pos < hex_len {
        let first = hex_string[hex_len - 1];
        let code = slice_to_numeric(&[first, b'0'], 16)?;
        vec.push(code);
    }
    Some(vec)
}

pub fn name_to_name(name: &[u8]) -> Option<Vec<u8>> {
    let mut vec = vec![];
    let mut original_pos = 0;
    let original_len = name.len();
    while original_pos < original_len {
        let original_curr = name[original_pos];
        if original_curr == POUND {
            let first = name[original_pos + 1];
            let second = name[original_pos + 2];
            let hex = [first, second];
            let code = slice_to_numeric(&hex, 16)?;
            vec.push(code);
            original_pos += 3;
        } else {
            vec.push(original_curr);
            original_pos += 1;
        }
    }
    Some(vec)
}
