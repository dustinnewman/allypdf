use crate::util::{LTHAN, RTHAN, TILDE};

const RADIX: u32 = 85;
const MIN_ENC: u8 = b'!';
const MAX_ENC: u8 = b'u';
const MAX_DEC: u8 = MAX_ENC - MIN_ENC;
const ALL_ZERO_SYMBOL: u8 = b'z';

pub fn ascii_85_decode(mut content: &[u8]) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    let mut counter: u32 = 0;
    let mut chunk: u32 = 0;
    if content[0] == LTHAN && content[1] == TILDE {
        content = &content[2..];
    }
    // Ignore EOD marker
    if content[content.len() - 2] == TILDE && content[content.len() - 1] == RTHAN {
        content = &content[..content.len() - 2];
    }
    let iter = content.iter().filter(|c| !c.is_ascii_whitespace());
    for &byte in iter {
        if byte == ALL_ZERO_SYMBOL {
            if counter == 0 {
                buf.extend_from_slice(&[0, 0, 0, 0]);
            } else {
                // 'z' in the middle of a group is forbidden
                return None;
            }
        }
        if byte < MIN_ENC || byte > MAX_ENC {
            return None;
        }
        let digit = (byte - MIN_ENC) as u32;
        chunk += digit * RADIX.pow(4 - counter);
        if counter == 4 {
            buf.extend_from_slice(&chunk.to_be_bytes());
            chunk = 0;
            counter = 0;
        } else {
            counter += 1;
        }
    }
    let mut to_remove = 0;
    while counter != 0 {
        chunk += (MAX_DEC as u32) * RADIX.pow(4 - counter);
        if counter == 4 {
            buf.extend_from_slice(&chunk.to_be_bytes());
            chunk = 0;
            counter = 0;
        } else {
            counter += 1;
        }
        to_remove += 1;
    }
    buf.drain((buf.len() - to_remove)..buf.len());
    Some(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let text = b"9jqo^BlbD-BleB1DJ+*+F(f,q";
        let vec = b"Man is distinguished".to_vec();
        assert_eq!(ascii_85_decode(text).unwrap(), vec);
    }

    #[test]
    fn test_whitespace() {
        let text = b"9jqo^Bl bD-BleB1DJ+*+F(f,q";
        let vec = b"Man is distinguished".to_vec();
        assert_eq!(ascii_85_decode(text).unwrap(), vec);
    }
}
