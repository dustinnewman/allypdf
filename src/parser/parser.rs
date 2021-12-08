use std::{collections::BTreeMap, convert::TryFrom};

use super::lexer::Token;
use crate::{
    filter::{decode, Filter},
    operators::operators::Operator,
    util::{hex_string_to_string, literal_string_to_string, name_to_name},
};

const FILTER: &[u8] = b"Filter";
const SIZE: &[u8] = b"Size";
const ROOT: &[u8] = b"Root";

#[derive(Debug, PartialEq)]
pub struct CrossReference {
    offset: u64,
    generation_number: u32,
    in_use: bool,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct IndirectReference {
    pub object_number: u32,
    pub generation_number: u32,
}

#[derive(Debug, PartialEq)]
pub struct Stream {
    dict: Dictionary,
    content: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct IndirectObject {
    pub object_number: u32,
    pub generation_number: u32,
    pub object: Box<Object>,
}

#[derive(Debug, PartialEq)]
pub struct XrefSubsection {
    start_number: u32,
    subsection_length: u32,
    references: Vec<CrossReference>,
}

#[derive(Debug, PartialEq)]
pub struct XrefSection {
    subsections: Vec<XrefSubsection>,
}

#[derive(Debug, PartialEq)]
pub struct Trailer {
    pub size: u64,
    pub root: IndirectReference,
    pub dictionary: Dictionary,
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
    Trailer(Trailer),
    Xref(XrefSection),
    StartXref(u64),
    Operator(Operator),
    Null,
}

pub type Name = Vec<u8>;
pub type Dictionary = BTreeMap<Name, Object>;

pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Self { tokens, pos: 0 }
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
        let object = match self.pop()? {
            Token::Boolean(b) => Object::Boolean(*b),
            Token::Null => Object::Null,
            Token::Real(r) => Object::Real(*r),
            Token::Header(m, n) => Object::Header(*m, *n),
            Token::Trailer => self.trailer()?,
            Token::Integer(i) => {
                let i = *i;
                self.integer(i)?
            }
            Token::LiteralString(lit) => Object::String(literal_string_to_string(*lit)?),
            Token::HexString(hex) => Object::String(hex_string_to_string(*hex)?),
            Token::Name(name) => Object::Name(name_to_name(*name)?),
            Token::Operator(op) => Object::Operator(*op),
            Token::F => Object::Operator(Operator::FillPath),
            Token::N => Object::Operator(Operator::EndPathNoFill),
            Token::Xref => self.xref(),
            Token::StartXref => self.start_xref()?,
            Token::LBracket => self.array()?,
            Token::DoubleLThan => self.dictionary()?,
            _ => return None,
        };
        Some(object)
    }

    fn xref_subsection(&mut self, xref_subsection: &mut XrefSubsection) {
        while let (
            Some(Token::Integer(offset)),
            Some(Token::Integer(generation_number)),
            Some(in_use),
        ) = (self.peek(), self.nth(1), self.nth(2))
        {
            let in_use = match in_use {
                Token::F => false,
                Token::N => true,
                _ => break,
            };
            let offset = *offset as u64;
            let generation_number = *generation_number as u32;
            let xref = CrossReference {
                offset,
                generation_number,
                in_use,
            };
            xref_subsection.references.push(xref);
            self.seek(3);
        }
    }

    fn xref(&mut self) -> Object {
        let mut xref_section = XrefSection {
            subsections: vec![],
        };
        while let (Some(Token::Integer(first)), Some(Token::Integer(entries))) =
            (self.peek(), self.nth(1))
        {
            let start_number = *first as u32;
            let subsection_length = *entries as u32;
            self.seek(2);
            let mut xref_subsection = XrefSubsection {
                start_number,
                subsection_length,
                references: vec![],
            };
            self.xref_subsection(&mut xref_subsection);
            xref_section.subsections.push(xref_subsection);
        }
        Object::Xref(xref_section)
    }

    fn trailer(&mut self) -> Option<Object> {
        if let Some(Object::Dictionary(d)) = self.next() {
            let size = d.get(SIZE)?;
            let root = d.get(ROOT)?;
            if let Object::Integer(size) = size {
                if let Object::IndirectReference(root) = root {
                    let trailer = Trailer {
                        size: *size as u64,
                        root: *root,
                        dictionary: d,
                    };
                    return Some(Object::Trailer(trailer));
                }
            }
        }
        None
    }

    fn start_xref(&mut self) -> Option<Object> {
        if let Token::Integer(i) = self.peek()? {
            let i = *i as u64;
            self.advance();
            Some(Object::StartXref(i))
        } else {
            None
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
                }
                _ => continue,
            }
        }
        let tokens = &self.tokens[start..self.pos];
        let mut parser = Parser::new(tokens);
        let array = parser.parse();
        Some(Object::Array(array))
    }

    fn stream_content(&self, dict: &Dictionary, content: &[u8]) -> Option<Vec<u8>> {
        let mut vec = content.to_vec();
        let mut filters = vec![];
        match dict.get(&FILTER.to_vec()) {
            Some(Object::Name(name)) => {
                if let Ok(filter) = Filter::try_from(name) {
                    filters.push(filter);
                } else {
                    return None;
                }
            }
            Some(Object::Array(names)) => {
                for name in names {
                    if let Object::Name(name) = name {
                        if let Ok(filter) = Filter::try_from(name) {
                            filters.push(filter);
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            }
            _ => (),
        };
        // Iterate through filters
        for filter in filters {
            vec = decode(&vec, filter, dict)?;
        }
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
                        let stream = Stream { dict, content: vec };
                        return Some(Object::Stream(stream));
                    }
                }
                break;
            }
            let key = match self.next() {
                Some(Object::Name(vec)) => vec,
                _ => break,
            };
            let value = match self.next() {
                Some(o) => o,
                None => break,
            };
            dict.insert(key, value);
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
            }
            (Some(Token::Integer(i)), Some(Token::Obj)) => {
                let generation_number = *i as u32;
                self.seek(2);
                let object = self.next()?;
                match self.pop()? {
                    Token::Endobj => Some(Object::IndirectObject(IndirectObject {
                        object_number: number as u32,
                        generation_number,
                        object: Box::new(object),
                    })),
                    _ => None,
                }
            }
            (Some(Token::Integer(i)), Some(Token::F)) => {
                let generation_number = *i as u32;
                self.seek(2);
                Some(Object::CrossReference(CrossReference {
                    offset: number as u64,
                    generation_number,
                    in_use: false,
                }))
            }
            (Some(Token::Integer(i)), Some(Token::N)) => {
                let generation_number = *i as u32;
                self.seek(2);
                Some(Object::CrossReference(CrossReference {
                    offset: number as u64,
                    generation_number,
                    in_use: true,
                }))
            }
            _ => Some(Object::Integer(number)),
        }
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
    use super::*;
    use crate::parser::lexer::Lexer;
    use std::fs;
    use std::path::PathBuf;

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
        let objects: Vec<Object> = vec![
            Object::IndirectReference(indirect_ref),
            Object::Boolean(false),
        ];
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
    fn test_cross_reference_2() {
        // SPEC_BREAK
        let text = "0 8 
        0000000000 65535 f
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let cross_ref = CrossReference {
            offset: 0,
            generation_number: 65535,
            in_use: false,
        };
        let expected: Vec<Object> = vec![
            Object::Integer(0),
            Object::Integer(8),
            Object::CrossReference(cross_ref),
        ];
        let objects = parser.parse();
        assert_eq!(objects, expected);
    }

    #[test]
    fn test_parser_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/hello.pdf");
        let file = fs::read(d).unwrap();
        let mut lexer = Lexer::new(&file);
        let tokens = lexer.lex();
        // assert_eq!(tokens, vec![]);
        let mut parser = Parser::new(&tokens);
        let objects = parser.parse();
        assert_eq!(objects, vec![]);
    }
}
