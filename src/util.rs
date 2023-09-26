use crate::parser::object::Name;

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
pub const PLUS: Byte = b'\x2B'; // +
pub const MINUS: Byte = b'\x2D'; // -
pub const Z: Byte = b'\x5A'; // Z

pub type Byte = u8;

#[macro_export]
macro_rules! inner {
    ($outer:expr, $inner:path) => {
        match $outer {
            Object {
                kind: $inner(i), ..
            } => Some(i),
            _ => None,
        }
    };
}

#[macro_export]
macro_rules! offset {
    ($kind:expr) => {
        Object {
            offset: 0,
            kind: $kind,
        }
    };
    ($offset:expr, $kind:expr) => {
        Object {
            offset: $offset,
            kind: $kind,
        }
    };
}

#[macro_export]
macro_rules! integer {
    ($int:expr) => {
        offset!(ObjectKind::Integer($int))
    };
}

#[macro_export]
macro_rules! real {
    ($real:expr) => {
        offset!(ObjectKind::Real($real))
    };
}

#[macro_export]
macro_rules! boolean {
    ($boolean:expr) => {
        offset!(ObjectKind::Boolean($boolean))
    };
}

#[macro_export]
macro_rules! name {
    ($str:expr) => {
        offset!(ObjectKind::Name($crate::parser::object::Name(
            $str.as_bytes().to_vec()
        )))
    };
}

#[macro_export]
macro_rules! string {
    ($str:expr) => {
        offset!(ObjectKind::String($str.as_bytes().to_vec()))
    };
}

#[macro_export]
macro_rules! operator {
    ($op:expr) => {
        offset!(ObjectKind::Operator($op))
    };
}

#[macro_export]
macro_rules! array {
    () => (
        offset!(ObjectKind::Array(vec![]))
    );
    ($($elem:expr),+ $(,)?) => (
        offset!(ObjectKind::Array(vec![$($elem),+]))
    );
}

#[macro_export]
macro_rules! dict {
    ($($key:expr => $val:expr),*) => (
        offset!(ObjectKind::Dictionary($crate::parser::object::Dictionary::from([
            $(($key.to_vec(), $val),)*
        ])))
    );
}

#[macro_export]
macro_rules! stream {
    ($content:expr, $($key:expr => $val:expr),*) => (
        offset!(ObjectKind::Stream(Stream {
            dict: $crate::parser::object::Dictionary::from([
                $(($key.to_vec(), $val),)*
            ]),
            content: $content,
        }))
    );
}

#[macro_export]
macro_rules! xref {
    ($offset:expr, $generation_number:expr, $in_use:expr) => {
        CrossReference {
            offset: $offset,
            generation_number: $generation_number,
            in_use: $in_use,
        }
    };
}

#[macro_export]
macro_rules! xref_section {
    ($($elem:expr),+ $(,)?) => (
        offset!(ObjectKind::Xref(XrefSection {
            subsections: vec![$($elem),+]
        }))
    );
}

#[macro_export]
macro_rules! indirect_reference {
    ($object_number:expr) => {
        offset!(ObjectKind::IndirectReference(IndirectReference {
            object_number: $object_number,
            generation_number: 0,
        }))
    };
    ($object_number:expr, $generation_number:expr) => {
        offset!(ObjectKind::IndirectReference(IndirectReference {
            object_number: $object_number,
            generation_number: $generation_number,
        }))
    };
}

#[macro_export]
macro_rules! indirect_object {
    ($object_number:expr, $object:expr) => {
        offset!(ObjectKind::IndirectObject(IndirectObject {
            object_number: $object_number,
            generation_number: 0,
            object: Box::new($object)
        }))
    };
    ($object_number:expr, $generation_number:expr, $object:expr) => {
        offset!(ObjectKind::IndirectObject(IndirectObject {
            object_number: $object_number,
            generation_number: $generation_number,
            object: Box::new($object)
        }))
    };
}

pub fn is_whitespace(byte: Byte) -> bool {
    matches!(
        byte,
        SPACE | CARRIAGE_RETURN | LINE_FEED | TAB | NULL_BYTE | FORM_FEED
    )
}

pub fn is_delimiter(byte: Byte) -> bool {
    matches!(
        byte,
        LPAREN | RPAREN | LTHAN | RTHAN | LBRACKET | RBRACKET | LBRACE | RBRACE | FSLASH | PERCENT
    )
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
    matches!(byte, b'0'..=b'7')
}

pub fn byte_to_numeric(byte: Byte, radix: u8) -> Option<u8> {
    if radix == 10 {
        if byte.is_ascii_digit() {
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

pub fn slice_to_numeric(slice: &[Byte], radix: u8) -> Option<u32> {
    let mut result: u32 = 0;
    for byte in slice {
        match byte_to_numeric(*byte, radix) {
            Some(x) => {
                result *= radix as u32;
                result += x as u32;
            }
            None => return None,
        }
    }
    Some(result)
}

pub fn reduce_slice_to_numeric(slice: &[u8]) -> u32 {
    let mut result: u32 = 0;
    for &x in slice {
        result = (result << 8) | x as u32;
    }
    result
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
                    vec.push(code as u8);
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

fn hex_byte_to_byte(bytes: &[u8]) -> Option<u8> {
    let first = *bytes.first().unwrap_or(&b'0');
    let second = *bytes.get(1).unwrap_or(&b'0');
    let first = byte_to_numeric(first, 16)?;
    let second = byte_to_numeric(second, 16)?;
    Some(first << 4 | second)
}

pub fn hex_string_to_string(hex_string: &[u8]) -> Option<Vec<u8>> {
    // We need to ignore whitespace so we have to filter it out before
    // anything else is done
    hex_string
        .iter()
        .filter(|c| c.is_ascii_hexdigit())
        .copied()
        .collect::<Vec<u8>>()
        .chunks(2)
        .map(hex_byte_to_byte)
        .collect()
}

pub fn hex_string_to_utf16(hex_string: &[u8]) -> Vec<u16> {
    let mut vec = vec![];
    for bytes in hex_string.chunks(4) {
        let first = hex_byte_to_byte(&bytes[0..2]).unwrap_or_default() as u16;
        let second = hex_byte_to_byte(&bytes[2..4]).unwrap_or_default() as u16;
        let ch = (first << 8) | second;
        vec.push(ch);
    }
    vec
}

pub fn name_to_name(name: &[u8]) -> Option<Name> {
    let mut vec = vec![];
    let mut original_pos = 0;
    let original_len = name.len();
    while original_pos < original_len {
        let original_curr = name[original_pos];
        if original_curr == POUND {
            let hex = &name[original_pos + 1..original_pos + 3];
            let code = hex_byte_to_byte(hex)?;
            vec.push(code);
            original_pos += 3;
        } else {
            vec.push(original_curr);
            original_pos += 1;
        }
    }
    Some(Name(vec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_basic() {
        let name = b"Name1";
        let expected = Name(b"Name1".to_vec());
        assert_eq!(name_to_name(name).unwrap(), expected);
    }

    #[test]
    fn test_name_with_hash() {
        let name = b"Lime#20Green";
        let expected = Name(b"Lime Green".to_vec());
        assert_eq!(name_to_name(name).unwrap(), expected);
    }

    #[test]
    fn test_name_with_two_hashes() {
        let name = b"paired#28#29parentheses";
        let expected = Name(b"paired()parentheses".to_vec());
        assert_eq!(name_to_name(name).unwrap(), expected);
    }

    #[test]
    fn test_hex_string_even_length() {
        let bytes = b"4e6f762073686d6f7a206b6120706f702e";
        let string = hex_string_to_string(bytes);
        assert!(string.is_some());
        assert_eq!(string.unwrap(), b"Nov shmoz ka pop.".to_vec());
    }

    #[test]
    fn test_hex_string_odd_length() {
        let bytes = b"01234567890ABCDEF";
        let string = hex_string_to_string(bytes);
        assert!(string.is_some());
        assert_eq!(
            string.unwrap(),
            vec![1, 35, 69, 103, 137, 10, 188, 222, 240]
        );
    }

    #[test]
    fn test_hex_string_to_utf16() {
        let bytes = b"06440627";
        assert_eq!(hex_string_to_utf16(bytes), vec![1604, 1575]);
    }
}
