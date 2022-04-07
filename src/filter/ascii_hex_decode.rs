use crate::util::{byte_to_numeric, is_whitespace, RTHAN};

pub fn ascii_hex_decode(content: &[u8]) -> Option<Vec<u8>> {
    let mut buf = Vec::with_capacity(content.len() * 2);
    let mut iter = content.iter().filter(|&&b| !is_whitespace(b));
    while let Some(&first) = iter.next() {
        if first == RTHAN {
            break;
        }
        let first = byte_to_numeric(first, 16)? * 16;
        match iter.next() {
            Some(&b) if b == RTHAN => {
                buf.push(first);
                break;
            }
            Some(&b) => {
                let second = byte_to_numeric(b, 16)?;
                buf.push(first + second);
            }
            None => {
                buf.push(first);
                break;
            }
        };
    }
    Some(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let text = "61 62 2e6364   65".as_bytes();
        let vec = "ab.cde".as_bytes().to_vec();
        assert_eq!(ascii_hex_decode(text).unwrap(), vec);
    }

    #[test]
    fn test_2() {
        let text = "61 62 2e6364   657>".as_bytes();
        let vec = "ab.cdep".as_bytes().to_vec();
        assert_eq!(ascii_hex_decode(text).unwrap(), vec);
    }

    #[test]
    fn test_3() {
        let text = "7>".as_bytes();
        let vec = "p".as_bytes().to_vec();
        assert_eq!(ascii_hex_decode(text).unwrap(), vec);
    }
}
