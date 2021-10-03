use std::collections::BTreeMap;

use super::lexer::{Lexer, Token};
use crate::util::{CARRIAGE_RETURN, BACK_SPACE, FORM_FEED, LINE_FEED, TAB, LPAREN, RPAREN, POUND, BSLASH, is_octal, slice_to_numeric};

#[derive(Debug, PartialEq)]
pub struct CrossReference {
    offset: u64,
    generation_number: u32,
    in_use: bool,
}

#[derive(Debug, PartialEq)]
pub struct IndirectReference {
    object_number: u32,
    generation_number: u32,
}

#[derive(Debug, PartialEq)]
pub struct Stream {
    dict: Dictionary,
    content: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct IndirectObject {
    object_number: u32,
    generation_number: u32,
    object: Box<Object>,
}

#[derive(Debug, PartialEq)]
pub enum Object {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    Name(Name),
    String(Vec<u8>),
    Array(Vec<Object>),
    Dictionary(Dictionary),
    IndirectReference(IndirectReference),
    CrossReference(CrossReference),
    IndirectObject(IndirectObject),
    Stream(Stream),
    Header(u8, u8),
    Trailer,
    StartXref(u64),
    Null,
}

type Name = Vec<u8>;
type Dictionary = BTreeMap<Name, Object>;

pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Self {
            tokens,
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Object> {
        let mut vec = vec![];
        loop {
            if let Some(object) = self.next() {
                vec.push(object);
            } else {
                break;
            }
        }
        vec
    }

    fn next(&mut self) -> Option<Object> {
        let token = self.pop()?;
        let object = match token {
            Token::Boolean(b) => Object::Boolean(*b),
            Token::Null => Object::Null,
            Token::Real(r) => Object::Real(*r),
            Token::Header(m, n) => Object::Header(*m, *n),
            Token::Integer(i) => {
                let i = *i;
                self.integer(i)?
            },
            Token::LiteralString(lit) => {
                let lit = *lit;
                self.literal_string_to_string(lit)?
            },
            Token::HexString(hex) => {
                let hex = *hex;
                self.hex_string_to_string(hex)?
            }
            Token::Name(name) => {
                let name = *name;
                self.name_to_name(name)?
            }
            Token::StartXref => self.start_xref()?,
            Token::LBracket => self.array()?,
            Token::DoubleLThan => self.dictionary()?,
            _ => return None,
        };
        Some(object)
    }

    fn start_xref(&mut self) -> Option<Object> {
        match self.peek()? {
            Token::Integer(i) => {
                let i = *i as u64;
                self.advance();
                Some(Object::StartXref(i))
            },
            _ => None
        }
    }

    fn array(&mut self) -> Option<Object> {
        let mut depth: u32 = 1;
        let start = self.pos;
        loop {
            match self.pop()? {
                Token::LBracket => depth += 1,
                Token::RBracket => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                },
                _ => continue,
            }
        }
        let tokens = &self.tokens[start..self.pos];
        let mut parser = Parser::new(tokens);
        let array = parser.parse();
        Some(Object::Array(array))
    }

    fn stream_content(&self, dict: &Dictionary, content: &[u8]) -> Option<Vec<u8>> {
        let vec = content.to_vec();
        // Iterate through filters
        Some(vec)
    }

    fn dictionary(&mut self) -> Option<Object> {
        let mut dict = BTreeMap::new();
        loop {
            if let Some(Token::DoubleRThan) = self.peek() {
                self.advance();
                // Handle stream after dict
                if let Some(Token::Stream(_)) = self.peek() {
                    // Parse stream
                    let content = match self.peek() {
                        Some(Token::Stream(x)) => x,
                        _ => break,
                    };
                    if let Some(vec) = self.stream_content(&dict, content) {
                        self.advance();
                        let stream = Stream {
                            dict,
                            content: vec,
                        };
                        return Some(Object::Stream(stream));
                    }
                }
                break;
            }
            let key = match self.next() {
                Some(o) => o,
                None => break,
            };
            let name = match key {
                Object::Name(vec) => vec,
                _ => break,
            };
            let value = match self.next() {
                Some(o) => o,
                None => break,
            };
            dict.insert(name, value);
        }
        Some(Object::Dictionary(dict))
    }

    fn integer(&mut self, number: i64) -> Option<Object> {
        match (self.peek(), self.nth(1)) {
            (Some(Token::Integer(i)), Some(Token::Reference)) => {
                let generation_number = *i as u32;
                self.seek(2);
                Some(Object::IndirectReference(IndirectReference {
                    object_number: number as u32,
                    generation_number,
                }))
            },
            (Some(Token::Integer(i)), Some(Token::Obj)) => {
                let generation_number = *i as u32;
                self.seek(2);
                let object = self.next()?;
                match self.pop()? {
                    Token::Endobj => {
                        Some(Object::IndirectObject(IndirectObject {
                            object_number: number as u32,
                            generation_number,
                            object: Box::new(object),
                        }))
                    },
                    _ => None,
                }
            },
            (Some(Token::Integer(i)), Some(Token::InUse(in_use))) => {
                let generation_number = *i as u32;
                let in_use = *in_use;
                self.seek(2);
                Some(Object::CrossReference(CrossReference {
                    offset: number as u64,
                    generation_number,
                    in_use,
                }))
            },
            _ => Some(Object::Integer(number)),
        }
    }

    fn literal_string_to_string(&self, literal: &[u8]) -> Option<Object> {
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
                    },
                    _ => (),
                };
                original_pos += 2;
            } else {
                vec.push(original_curr);
                original_pos += 1;
            }
        }
        Some(Object::String(vec))
    }

    fn hex_string_to_string(&self, hex_string: &[u8]) -> Option<Object> {
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
        Some(Object::String(vec))
    }

    fn name_to_name(&self, name: &[u8]) -> Option<Object> {
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
        Some(Object::Name(vec))
    }

    fn pop(&mut self) -> Option<&Token<'a>> {
        // TODO: Fix this
        // let token = self.peek();
        self.advance();
        Some(&self.tokens[self.pos - 1])
    }

    fn advance(&mut self) {
        self.seek(1);
    }

    fn seek(&mut self, n: usize) {
        if self.pos + n <= self.tokens.len() {
            self.pos += n;
        }
    }

    fn peek(&self) -> Option<&Token<'a>> {
        self.nth(0)
    }

    fn nth(&self, n: usize) -> Option<&Token<'a>> {
        if self.pos + n < self.tokens.len() {
            Some(&self.tokens[self.pos + n])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_integer() {
        let text = "0
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let objects: Vec<Object> = vec![Object::Integer(0)];
        assert_eq!(parser.parse(), objects);
    }

    #[test]
    fn test_indirect_ref() {
        let text = "17 0 R false
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let indirect_ref = IndirectReference {
            object_number: 17,
            generation_number: 0,
        };
        let objects: Vec<Object> = vec![Object::IndirectReference(indirect_ref), Object::Boolean(false)];
        assert_eq!(parser.parse(), objects);
    }

    #[test]
    fn test_dict() {
        let text = "<</A /B /C /D>>
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let mut dict = BTreeMap::new();
        dict.insert(b"A".to_vec(), Object::Name(b"B".to_vec()));
        dict.insert(b"C".to_vec(), Object::Name(b"D".to_vec()));
        let objects: Vec<Object> = vec![Object::Dictionary(dict)];
        assert_eq!(parser.parse(), objects);
    }

    #[test]
    fn test_nested_dict() {
        let text = "<</A /B /C <</D /E>>>>
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let mut dict = BTreeMap::new();
        dict.insert(b"A".to_vec(), Object::Name(b"B".to_vec()));
        let mut sub_dict = BTreeMap::new();
        sub_dict.insert(b"D".to_vec(), Object::Name(b"E".to_vec()));
        dict.insert(b"C".to_vec(), Object::Dictionary(sub_dict));
        let objects: Vec<Object> = vec![Object::Dictionary(dict)];
        assert_eq!(parser.parse(), objects);
    }

    #[test]
    fn test_cross_reference() {
        // SPEC_BREAK
        let text = "0000000421 00000 n
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let cross_ref = CrossReference {
            offset: 421,
            generation_number: 0,
            in_use: true,
        };
        let objects: Vec<Object> = vec![Object::CrossReference(cross_ref)];
        assert_eq!(parser.parse(), objects);
    }

    #[test]
    fn test_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/hello.pdf");
        let file = fs::read(d).unwrap();
        let mut lexer = Lexer::new(&file);
        let tokens = lexer.lex();
        // assert_eq!(tokens, vec![]);
        let mut parser = Parser::new(&tokens);
        assert_eq!(parser.parse(), vec![]);
    }
}