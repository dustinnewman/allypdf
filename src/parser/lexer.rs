use crate::error::{Result, PdfError};
use crate::util::{Byte, SPACE, CARRIAGE_RETURN, LINE_FEED, LPAREN, RPAREN, LTHAN, RTHAN, LBRACKET, RBRACKET, LBRACE, RBRACE, FSLASH, PERCENT, PERIOD, is_decimal, is_hexadecimal, byte_to_numeric, is_whitespace, is_regular, is_newline};

enum Character {
    Regular(Byte),
    Delimiter(Byte),
    Whitespace(Byte),
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    Name(&'a [u8]),
    LiteralString(&'a [u8]),
    HexString(&'a [u8]),
    DoubleLThan,
    DoubleRThan,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    // Major Minor (ex: 1.7 = (1, 7))
    Header(u8, u8),
    Reference,
    Obj,
    Endobj,
    Xref,
    StartXref,
    Stream(&'a [u8]),
    Null,
    Trailer,
    InUse(bool),
    Eof,
}

pub struct Lexer<'a> {
    pos: usize,
    len: usize,
    buf: &'a [Byte],
}

impl<'a> Lexer<'a> {
    pub fn new(buf: &'a [Byte]) -> Self {
        Self {
            pos: 0,
            len: buf.len(),
            buf,
        }
    }

    pub fn lex(&mut self) -> Vec<Token<'a>> {
        let mut vec = vec![];
        // TODO: This is susceptible to attack if someone uploads a PDF
        // without EOF marker
        loop {
            if let Some(token) = self.next() {
                let do_break = token == Token::Eof;
                vec.push(token);
                if do_break {
                    break;
                }
            }
        }
        vec
    }

    fn next(&mut self) -> Option<Token<'a>> {
        let curr = self.peek()?;
        if is_whitespace(curr) {
            self.skip_whitespace();
        }
        let curr = self.pop()?;
        let token: Token = match curr {
            LTHAN => match self.peek() {
                Some(LTHAN) => {
                    self.advance();
                    Token::DoubleLThan
                },
                _ => self.hex_string()?,
            },
            RTHAN => match self.peek() {
                Some(RTHAN) => {
                    self.advance();
                    Token::DoubleRThan
                },
                _ => return None,
            },
            b'x' => match (self.pop(), self.pop(), self.pop()) {
                (Some(b'r'), Some(b'e'), Some(b'f')) => Token::Xref,
                _ => return None,
            },
            b't' => match (self.pop(), self.pop(), self.pop()) {
                (Some(b'r'), Some(b'u'), Some(b'e')) => Token::Boolean(true),
                (Some(b'r'), Some(b'a'), Some(b'i')) => self.trailer()?,
                _ => return None,
            },
            b'f' => self.f()?,
            b's' => match (self.pop(), self.pop()) {
                (Some(b't'), Some(b'r')) => self.stream()?,
                (Some(b't'), Some(b'a')) => self.startxref()?,
                _ => return None,
            },
            b'n' => self.n()?,
            b'o' => match (self.pop(), self.pop()) {
                (Some(b'b'), Some(b'j')) => Token::Obj,
                _ => return None,
            },
            b'e' => self.endobj()?,
            b'0'..=b'9' | b'+' | b'-' | PERIOD => self.number()?,
            PERCENT => self.percent()?,
            LPAREN => self.literal_string()?,
            LBRACKET => Token::LBracket,
            RBRACKET => Token::RBracket,
            LBRACE => Token::LBrace,
            RBRACE => Token::RBrace,
            FSLASH => self.name()?,
            b'R' => Token::Reference,
            _ => return None,
        };
        Some(token)
    }

    fn f(&mut self) -> Option<Token<'a>> {
        match self.slice(self.pos, 2)? {
            [SPACE, CARRIAGE_RETURN] |
            [SPACE, LINE_FEED] |
            [CARRIAGE_RETURN, LINE_FEED] => {
                self.seek(2);
                Some(Token::InUse(false))
            },
            // SPEC_BREAK: Technically just a newline does not meet the
            // criteria to be an in-use identifier, but let's accept it
            // because sometimes a CR-LF combo can get flattened to just
            // a newline during transmission
            [LINE_FEED, _] => {
                self.seek(1);
                Some(Token::InUse(false))
            },
            [b'a', b'l'] => match self.slice(self.pos + 2, 2)? {
                [b's', b'e'] => {
                    self.seek(4);
                    Some(Token::Boolean(false))
                },
                _ => None
            },

            _ => None,
        }
    }

    fn number(&mut self) -> Option<Token<'a>> {
        let start = self.pos - 1;
        let mut is_real = false;
        loop {
            match self.peek() {
                Some(c) if is_decimal(c) => self.advance(),
                Some(PERIOD) => {
                    self.advance();
                    is_real = true
                },
                _ => break,
            }
        }
        let buf = &(self.buf[start..self.pos]);
        if is_real {
            let number = std::str::from_utf8(buf).ok().and_then(|x| x.parse().ok())?;
            Some(Token::Real(number))
        } else {
            let number = std::str::from_utf8(buf).ok().and_then(|x| x.parse().ok())?;
            Some(Token::Integer(number))
        }
    }

    fn trailer(&mut self) -> Option<Token<'a>> {
        let string = "ler".as_bytes();
        let token = match self.slice(self.pos, string.len()) {
            Some(x) if x == string => {
                self.seek(string.len());
                Token::Trailer
            },
            _ => return None,
        };
        Some(token)
    }

    fn startxref(&mut self) -> Option<Token<'a>> {
        let string = "rtxref".as_bytes();
        let token = match self.slice(self.pos, string.len()) {
            Some(x) if x == string => {
                self.seek(string.len());
                Token::StartXref
            },
            _ => return None,
        };
        Some(token)
    }

    fn endobj(&mut self) -> Option<Token<'a>> {
        let string = "ndobj".as_bytes();
        let token = match self.slice(self.pos, string.len()) {
            Some(x) if x == string => {
                self.seek(string.len());
                Token::Endobj
            },
            _ => return None,
        };
        Some(token)
    }

    fn n(&mut self) -> Option<Token<'a>> {
        match self.slice(self.pos, 2)? {
            [SPACE, CARRIAGE_RETURN] |
            [SPACE, LINE_FEED] |
            [CARRIAGE_RETURN, LINE_FEED] => {
                self.seek(2);
                Some(Token::InUse(true))
            },
            // SPEC_BREAK: Technically just a newline does not meet the
            // criteria to be an in-use identifier, but let's accept it
            // because sometimes a CR-LF combo can get flattened to just
            // a newline during transmission
            [LINE_FEED, _] => {
                self.seek(1);
                Some(Token::InUse(true))
            },
            [b'u', b'l'] => match self.nth(2) {
                Some(b'l') => {
                    self.seek(3);
                    Some(Token::Null)
                },
                _ => None,
            },
            _ => None,
        }
    }

    fn stream(&mut self) -> Option<Token<'a>> {
        let stream_string = "eam".as_bytes();
        let endstream_string = "endstream".as_bytes();
        match self.slice(self.pos, stream_string.len()) {
            Some(x) if x == stream_string => {
                // PDF32000_2008 7.3.8.1 paragraph 5
                self.get_next_char_while(|c| c != LINE_FEED);
                // Skip newline character
                self.advance();
                let start = self.pos;
                loop {
                    self.skip_whitespace();
                    match self.slice(self.pos, endstream_string.len()) {
                        Some(y) if y == endstream_string => {
                            let end = self.pos;
                            self.seek(endstream_string.len());
                            return Some(Token::Stream(&self.buf[start..end]));
                        },
                        Some(_) => {
                            self.advance();
                        },
                        None => {
                            return None;
                        },
                    }
                }
            },
            _ => None
        }
    }

    fn hex_string(&mut self) -> Option<Token<'a>> {
        let string = self.get_next_char_while(is_hexadecimal);
        Some(Token::HexString(string))
    }

    fn literal_string(&mut self) -> Option<Token<'a>> {
        // Start at 1 because we have already seen first LPAREN
        let mut depth: u32 = 1;
        let start = self.pos;
        while self.pos < self.len {
            let curr = self.pop()?;
            if curr == LPAREN {
                depth += 1;
            } else if curr == RPAREN {
                depth -= 1;
                // End of string
                if depth == 0 {
                    return Some(Token::LiteralString(&self.buf[start..self.pos-1]));
                }
            }
        }
        None
    }

    fn name(&mut self) -> Option<Token<'a>> {
        let start = self.pos;
        loop {
            match self.peek() {
                Some(c) if is_whitespace(c) => {
                    break;
                },
                Some(FSLASH) => {
                    break;
                },
                Some(c) if is_regular(c) => {
                    self.advance();
                    continue;
                },
                _ => break,
            };
        }
        Some(Token::Name(&self.buf[start..self.pos]))
    }

    fn eof(&mut self) -> Option<Token<'a>> {
        let string = "EOF".as_bytes();
        let token = match self.slice(self.pos, string.len()) {
            Some(x) if x == string => {
                self.seek_end();
                Token::Eof
            },
            _ => {
                self.next_line();
                return None
            },
        };
        Some(token)
    }
    
    fn header_version(&mut self) -> Option<Token<'a>> {
        let token = match (self.nth(1), self.nth(2), self.nth(3), self.nth(4), self.nth(5), self.nth(6)) {
            (Some(b'D'), Some(b'F'), Some(b'-'), Some(m), Some(b'.'), Some(n)) if is_decimal(m) && is_decimal(n) => {
                // Skip past "DF-m.n"
                self.seek(7);
                let major = byte_to_numeric(m, 10)?;
                let minor = byte_to_numeric(n, 10)?;
                Token::Header(major, minor)
            },
            _ => {
                self.next_line();
                return None
            },
        };
        Some(token)
    }

    fn percent(&mut self) -> Option<Token<'a>> {
        let token = match self.peek() {
            Some(PERCENT) => {
                self.advance();
                self.eof()?
            },
            Some(b'P') => self.header_version()?,
            _ => {
                    self.next_line();
                    return None
            },
        };
        Some(token)
    }

    fn advance(&mut self) {
        self.seek(1)
    }

    fn pop(&mut self) -> Option<Byte> {
        let prev = self.peek();
        self.advance();
        prev
    }

    fn nth(&self, n: usize) -> Option<Byte> {
        if self.pos < self.len - n {
            Some(self.buf[self.pos + n])
        } else {
            None
        }
    }

    fn peek(&self) -> Option<Byte> {
        self.nth(0)
    }

    fn peek_next(&self) -> Option<Byte> {
        self.nth(1)
    }

    fn get_next_char_while(&mut self, cond: fn(Byte) -> bool) -> &'a [Byte] {
        let start = self.pos;
        loop {
            match self.peek() {
                Some(c) if cond(c) => {
                    self.advance();
                }
                _ => {
                    return &self.buf[start..self.pos];
                }
            }
        }
    }

    fn seek(&mut self, n: usize) {
        if self.pos <= self.len - n {
            self.pos += n
        }
    }

    fn seek_start(&mut self) {
        self.pos = 0
    }

    fn seek_end(&mut self) {
        self.pos = self.len - 1
    }

    fn slice(&self, start: usize, length: usize) -> Option<&'a [Byte]> {
        if start + length > self.len {
            None
        } else {
            Some(&self.buf[start..start+length])
        }
    }

    fn skip_while(&mut self, predicate: impl Fn(Byte) -> bool) {
        loop {
            if let Some(c) = self.peek() {
                if predicate(c) && self.pos < self.len {
                    self.advance();
                } else {
                    return;
                }
            } else {
                return;
            }
        }
    }

    fn skip_until(&mut self, predicate: impl Fn(Byte) -> bool) {
        loop {
            if let Some(c) = self.peek() {
                if !predicate(c) && self.pos < self.len {
                    self.advance();
                } else {
                    return;
                }
            } else {
                return;
            }
        }
    }

    fn back_until(&mut self, predicate: impl Fn(Byte) -> bool) -> Result<usize> {
        self.buf[..self.pos]
            .iter()
            .rposition(|&b| predicate(b))
            .ok_or(PdfError::BOF)
    }

    fn skip_whitespace(&mut self) {
        self.skip_while(is_whitespace)
    }

    fn next_line(&mut self) {
        self.skip_until(is_newline)
    }

    fn skip_comments(&mut self) {
        match self.peek() {
            Some(c) if c == PERCENT => self.skip_until(is_newline),
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_next() {
        let text = "%PDF-1.4
        5 0 obj
        endobj
        startxref
        57184
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = vec![Token::Header(1, 4), Token::Integer(5), Token::Integer(0), Token::Obj, Token::Endobj, Token::StartXref, Token::Integer(57184), Token::Eof];
        assert_eq!(lexer.lex(), tokens);
    }

    #[test]
    fn test_dict_name() {
        let text = "<</Producer(GPL Ghostscript 8.71)>>
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = vec![Token::DoubleLThan, Token::Name("Producer".as_bytes()), Token::LiteralString(&"GPL Ghostscript 8.71".as_bytes()), Token::DoubleRThan, Token::Eof];
        assert_eq!(lexer.lex(), tokens);
    }

    #[test]
    fn test_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/hello.pdf");
        let file = fs::read(d).unwrap();
        let mut lexer = Lexer::new(&file);
        let tokens = vec![Token::Header(1, 4)];
        assert_eq!(lexer.lex(), tokens);
    }
}
