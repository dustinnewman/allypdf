use crate::util::{LTHAN, RTHAN, TILDE};

const RADIX: u32 = 85;
const MAX_DEC: u32 = 84; // b'u' - 33
const MIN_ENC: u8 = 33; // b'!'
const MAX_ENC: u8 = 117; // b'u'

pub fn ascii_85_decode(content: &[u8]) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    let mut counter: u32 = 0;
    let mut chunk: u32 = 0;
    let mut content = content;
    if content[0] == LTHAN && content[1] == TILDE {
        content = &content[2..];
    }
    if content[content.len() - 2] == TILDE && content[content.len() - 1] == RTHAN {
        content = &content[..content.len() - 2];
    }
    let iter = content.iter().filter(|c| !c.is_ascii_whitespace());
    for &byte in iter {
        if byte == b'z' {
            if counter == 0 {
                buf.extend_from_slice(&[0, 0, 0, 0]);
            } else {
                return None;
            }
        }
        if byte < MIN_ENC || byte > MAX_ENC {
            return None;
        }
        let digit = (byte - 33) as u32;
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
        chunk += MAX_DEC * RADIX.pow(4 - counter);
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
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let text = "9jqo^BlbD-BleB1DJ+*+F(f,q".as_bytes();
        let vec = "Man is distinguished".as_bytes().to_vec();
        assert_eq!(ascii_85_decode(text).unwrap(), vec);
    }

    #[test]
    fn test_whitespace() {
        let text = "9jqo^Bl bD-BleB1DJ+*+F(f,q".as_bytes();
        let vec = "Man is distinguished".as_bytes().to_vec();
        assert_eq!(ascii_85_decode(text).unwrap(), vec);
    }
}
