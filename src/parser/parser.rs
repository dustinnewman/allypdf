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
    use crate::{boolean, integer, real, name, array, dict, indirect_reference};
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_integer() {
        let text = "0
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let objects = vec![integer!(0)];
        assert_eq!(parser.parse(), objects);
    }

    #[test]
    fn test_parser_real() {
        let text = "0.02
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let objects = vec![real!(0.02)];
        assert_eq!(parser.parse(), objects);
    }

    #[test]
    fn test_indirect_ref() {
        let text = "17 0 R false
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let objects = vec![
            indirect_reference!(17),
            boolean!(false),
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
        let dict = dict!(
            b"A" => name!("B"),
            b"C" => name!("D")
        );
        let objects = vec![dict];
        assert_eq!(parser.parse(), objects);
    }

    #[test]
    fn test_nested_dict() {
        let text = "<</A /B /C <</D /E>>>>
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let dict = dict!(
            b"A" => name!("B"),
            b"C" => dict!(
                b"D" => name!("E")
            )
        );
        let objects = vec![dict];
        assert_eq!(parser.parse(), objects);
    }

    #[test]
    fn test_nested_dict_newline() {
        let text = b"3 0 obj\n<<\n/ProcSet [/PDF /Text /ImageB ]\n/Font <<\n/F1 4 0 R\n>>\n/XObject <<\n/Im1 11 0 R\n>>\n/ExtGState <<\n/GS1 12 0 R\n>>\n>>\nendobj
        %%EOF";
        let mut lexer = Lexer::new(text);
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let ext_gstate = dict!(
            b"GS1" => indirect_reference!(12)
        );
        let x_object = dict!(
            b"Im1" => indirect_reference!(11)
        );
        let font = dict!(
            b"F1" => indirect_reference!(4)
        );
        let array = array![name!("PDF"), name!("Text"), name!("ImageB")];
        let dict = dict!(
            b"ProcSet" => array,
            b"Font" => font,
            b"XObject" => x_object,
            b"ExtGState" => ext_gstate
        );
        let object = Object::IndirectObject(IndirectObject {
            object_number: 3,
            generation_number: 0,
            object: Box::new(dict),
        });
        let expected = vec![object];
        assert_eq!(parser.parse(), expected);
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
        let objects = vec![Object::CrossReference(cross_ref)];
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
        let expected = vec![
            integer!(0),
            integer!(8),
            Object::CrossReference(cross_ref),
        ];
        let objects = parser.parse();
        assert_eq!(objects, expected);
    }

    #[test]
    fn test_parser_stream_null_content() {
        let text = b"11 0 obj\n<<\n/Type /Image\n>>\nstream\r\n\0\nendstream\nendobj
        %%EOF";
        let mut lexer = Lexer::new(text);
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let dict = BTreeMap::from([
            (b"Type".to_vec(), name!("Image"))
        ]);
        let stream = Object::Stream(Stream {
            dict,
            content: b"\0\n".to_vec(),
        });
        let object = Object::IndirectObject(IndirectObject {
            object_number: 11,
            generation_number: 0,
            object: Box::new(stream),
        });
        let expected = vec![object];
        let objects = parser.parse();
        assert_eq!(objects, expected);
    }

    #[test]
    fn test_parser_dict_and_float() {
        let text = b"12 0 obj
        <<\n/Type /ExtGState\n/SA false\n/SM 0.02\n/OP false\n/op false\n/OPM 1
        /BG2 /Default\n/UCR2 /Default\n/HT /Default\n/TR2 /Default\n>>\nendobj
        %%EOF";
        let mut lexer = Lexer::new(text);
        let tokens = lexer.lex();
        println!("{:?}", tokens);
        let mut parser = Parser::new(&tokens);
        let dict = dict!(
            b"Type" => name!("ExtGState"),
            b"SA" => boolean!(false),
            b"SM" => real!(0.02),
            b"OP" => boolean!(false),
            b"op" => boolean!(false),
            b"OPM" => integer!(1),
            b"BG2" => name!("Default"),
            b"UCR2" => name!("Default"),
            b"HT" => name!("Default"),
            b"TR2" => name!("Default")
        );
        let object = Object::IndirectObject(IndirectObject {
            object_number: 12,
            generation_number: 0,
            object: Box::new(dict),
        });
        let expected = vec![object];
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_parser_basic_file() {
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

    #[test]
    fn test_parser_postscript_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/heinz.pdf");
        let file = fs::read(d).unwrap();
        let mut lexer = Lexer::new(&file);
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let objects = parser.parse();
        assert_eq!(objects, vec![]);
    }
}
