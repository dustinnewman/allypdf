use std::convert::TryFrom;

pub mod cid_parser;
pub mod date;
pub mod lexer;
pub mod object;

use crate::filter::{decode, Filter};
use crate::operators::Operator;
use crate::util::{hex_string_to_string, literal_string_to_string, name_to_name};
use lexer::{Token, TokenKind};
use object::{
    CrossReference, Dictionary, IndirectObject, IndirectReference, Object, ObjectKind, Stream,
    Trailer, XrefSection, XrefSubsection,
};

const FILTER: &[u8] = b"Filter";
const SIZE: &[u8] = b"Size";
const ROOT: &[u8] = b"Root";
const LENGTH: &[u8] = b"Length";

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
        while let Some(object) = self.next() {
            vec.push(object);
        }
        vec
    }

    fn next(&mut self) -> Option<Object> {
        let token = self.pop()?;
        let offset = token.offset;
        let kind = match token.kind {
            TokenKind::Boolean(b) => ObjectKind::Boolean(b),
            TokenKind::Null => ObjectKind::Null,
            TokenKind::Real(r) => ObjectKind::Real(r),
            TokenKind::Header(header) => ObjectKind::Header(header),
            TokenKind::Trailer => self.trailer()?,
            TokenKind::Integer(i) => self.integer(i)?,
            TokenKind::LiteralString(lit) => ObjectKind::String(literal_string_to_string(lit)?),
            TokenKind::HexString(hex) => ObjectKind::String(hex_string_to_string(hex)?),
            TokenKind::Name(name) => ObjectKind::Name(name_to_name(name)?),
            TokenKind::Operator(op) => ObjectKind::Operator(op),
            TokenKind::CIDOperator(op) => ObjectKind::CIDOperator(op),
            TokenKind::F => ObjectKind::Operator(Operator::FillPath),
            TokenKind::N => ObjectKind::Operator(Operator::EndPathNoFill),
            TokenKind::Xref => self.xref(),
            TokenKind::StartXref => self.start_xref()?,
            TokenKind::LBracket => self.array()?,
            TokenKind::DoubleLThan => self.dictionary()?,
            _ => return None,
        };
        let object = Object { offset, kind };
        Some(object)
    }

    fn xref_subsection(&mut self, xref_subsection: &mut XrefSubsection) {
        while let (
            Some(Token {
                kind: TokenKind::Integer(offset),
                ..
            }),
            Some(Token {
                kind: TokenKind::Integer(generation_number),
                ..
            }),
            Some(in_use),
        ) = (self.peek(), self.nth(1), self.nth(2))
        {
            let in_use = match in_use.kind {
                TokenKind::F => false,
                TokenKind::N => true,
                _ => break,
            };
            let offset = *offset as u64;
            let generation_number = *generation_number as u32;
            self.seek(3);
            let xref = CrossReference {
                offset,
                generation_number,
                in_use,
            };
            xref_subsection.references.push(xref);
        }
    }

    fn xref(&mut self) -> ObjectKind {
        let mut xref_section = XrefSection {
            subsections: vec![],
        };
        while let (
            Some(Token {
                kind: TokenKind::Integer(start_number),
                ..
            }),
            Some(Token {
                kind: TokenKind::Integer(subsection_length),
                ..
            }),
        ) = (self.peek(), self.nth(1))
        {
            let start_number = *start_number as u32;
            let subsection_length = *subsection_length as u32;
            self.seek(2);
            let mut xref_subsection = XrefSubsection {
                start_number,
                subsection_length,
                references: vec![],
            };
            self.xref_subsection(&mut xref_subsection);
            xref_section.subsections.push(xref_subsection);
        }
        ObjectKind::Xref(xref_section)
    }

    fn trailer(&mut self) -> Option<ObjectKind> {
        let ObjectKind::Dictionary(dict) = self.next()?.kind else {
            return None;
        };
        let size = dict.get(SIZE)?;
        let root = dict.get(ROOT)?;
        let ObjectKind::Integer(size) = size.kind else {
            return None;
        };
        let ObjectKind::IndirectReference(root) = root.kind else {
            return None;
        };
        let trailer = Trailer {
            size: size as u64,
            root,
            dictionary: dict,
        };
        Some(ObjectKind::Trailer(trailer))
    }

    fn start_xref(&mut self) -> Option<ObjectKind> {
        if let Token {
            kind: TokenKind::Integer(i),
            ..
        } = self.peek()?
        {
            let i = *i as u64;
            self.advance();
            Some(ObjectKind::StartXref(i))
        } else {
            None
        }
    }

    fn array(&mut self) -> Option<ObjectKind> {
        let mut depth: u32 = 1;
        let start = self.pos;
        loop {
            let next_token = self.pop()?;
            match next_token.kind {
                TokenKind::LBracket => depth += 1,
                TokenKind::RBracket => {
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
        Some(ObjectKind::Array(array))
    }

    fn stream_content(&self, dict: &Dictionary, content: &[u8]) -> Option<Vec<u8>> {
        // SPEC_BREAK Technically there is always supposed to be a stream length
        // but for testing purposes we can allow omission
        let length = if let Some(Object {
            kind: ObjectKind::Integer(i),
            ..
        }) = dict.get(LENGTH)
        {
            *i as usize
        } else {
            content.len()
        };
        let filters = if let Some(object) = dict.get(FILTER) {
            Vec::<Filter>::try_from(object).ok()?
        } else {
            vec![]
        };
        // Iterate through filters
        let mut vec = content[0..length].to_vec();
        for filter in filters {
            vec = decode(&vec, filter, dict)?;
        }
        Some(vec)
    }

    fn dictionary(&mut self) -> Option<ObjectKind> {
        let mut dict = Dictionary::new();
        loop {
            if let Some(Token {
                kind: TokenKind::DoubleRThan,
                ..
            }) = self.peek()
            {
                self.advance();
                // Handle stream after dict
                if let Some(Token {
                    kind: TokenKind::Stream(_),
                    ..
                }) = self.peek()
                {
                    // Parse stream
                    let content = match self.peek() {
                        Some(Token {
                            kind: TokenKind::Stream(x),
                            ..
                        }) => x,
                        _ => break,
                    };
                    if let Some(vec) = self.stream_content(&dict, content) {
                        self.advance();
                        let stream = Stream { dict, content: vec };
                        return Some(ObjectKind::Stream(stream));
                    }
                }
                break;
            }
            let key = match self.next() {
                Some(Object {
                    kind: ObjectKind::Name(vec),
                    ..
                }) => vec,
                _ => break,
            };
            let value = match self.next() {
                Some(o) => o,
                None => break,
            };
            dict.insert(key, value);
        }
        Some(ObjectKind::Dictionary(dict))
    }

    fn integer(&mut self, number: i64) -> Option<ObjectKind> {
        match (self.peek(), self.nth(1)) {
            (
                Some(Token {
                    kind: TokenKind::Integer(i),
                    ..
                }),
                Some(Token {
                    kind: TokenKind::Reference,
                    ..
                }),
            ) => {
                let generation_number = *i as u32;
                self.seek(2);
                Some(ObjectKind::IndirectReference(IndirectReference {
                    object_number: number as u32,
                    generation_number,
                }))
            }
            (
                Some(Token {
                    kind: TokenKind::Integer(i),
                    ..
                }),
                Some(Token {
                    kind: TokenKind::Obj,
                    ..
                }),
            ) => {
                let generation_number = *i as u32;
                self.seek(2);
                let object = self.next()?;
                match self.pop()?.kind {
                    TokenKind::Endobj => Some(ObjectKind::IndirectObject(IndirectObject {
                        object_number: number as u32,
                        generation_number,
                        object: Box::new(object),
                    })),
                    _ => None,
                }
            }
            _ => Some(ObjectKind::Integer(number)),
        }
    }

    fn pop(&mut self) -> Option<&Token<'a>> {
        // TODO: Fix this
        // let token = self.peek();
        if self.pos == self.tokens.len() {
            return None;
        }
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
    use crate::parser::cid_parser::CIDOperator;
    use crate::parser::lexer::{Header, Lexer};
    use crate::{
        array, boolean, dict, indirect_object, indirect_reference, inner, integer, name, offset,
        real, stream, string, xref, xref_section,
    };
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_integer() {
        let text = "0
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let expected = vec![integer!(0)];
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_real() {
        let text = "0.02
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let expected = vec![real!(0.02)];
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_hex_string() {
        let text = "<4e6f762073686d6f7a206b6120706f702e> %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let expected = vec![string!("Nov shmoz ka pop.")];
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_hex_string_with_space() {
        let text = b"<4 E>\n%%EOF";
        let mut lexer = Lexer::new(text);
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let expected = vec![string!("N")];
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_indirect_ref() {
        let text = "17 0 R false";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let expected = vec![indirect_reference!(17), boolean!(false)];
        assert_eq!(parser.parse(), expected);
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
        let expected = vec![dict];
        assert_eq!(parser.parse(), expected);
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
        let expected = vec![dict];
        assert_eq!(parser.parse(), expected);
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
        let object = ObjectKind::IndirectObject(IndirectObject {
            object_number: 3,
            generation_number: 0,
            object: Box::new(dict),
        });
        let expected = vec![object];
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_cross_reference_multi_xref() {
        // SPEC_BREAK
        let text = "xref
        0 3
        0000000000 65535 f
        0000000015 00000 n
        0000000064 00000 n
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let expected = vec![xref_section![XrefSubsection {
            start_number: 0,
            subsection_length: 3,
            references: vec![
                xref!(0, 65535, false),
                xref!(15, 0, true),
                xref!(64, 0, true),
            ]
        }]];
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_cross_reference_single_xref() {
        // SPEC_BREAK
        let text = "xref
        0 8 
        0000000000 65535 f
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let xref_section = xref_section![XrefSubsection {
            start_number: 0,
            subsection_length: 8,
            references: vec![xref!(0, 65535, false),]
        }];
        let expected = vec![xref_section];
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_cross_reference_multi_xrefsection() {
        // SPEC_BREAK
        let text = "xref
        0 3
        0000000000 65535 f
        0000000015 00000 n
        0000000064 00000 n
        0 1
        0000000064 00001 n
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let xref_section = xref_section![
            XrefSubsection {
                start_number: 0,
                subsection_length: 3,
                references: vec![
                    xref!(0, 65535, false),
                    xref!(15, 0, true),
                    xref!(64, 0, true),
                ],
            },
            XrefSubsection {
                start_number: 0,
                subsection_length: 1,
                references: vec![xref!(64, 1, true)],
            }
        ];
        let expected = vec![xref_section];
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_stream_null_content() {
        let text = b"11 0 obj\n<<\n/Type /Image\n>>\nstream\r\n\0\nendstream\nendobj
        %%EOF";
        let mut lexer = Lexer::new(text);
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let dict = dict!(
            b"Type" => name!("Image")
        );
        let dict = inner!(dict, ObjectKind::Dictionary).expect("Image type is not a dictionary.");
        let stream = ObjectKind::Stream(Stream {
            dict,
            content: b"\0\n".to_vec(),
        });
        let stream = Object {
            offset: 31,
            kind: stream,
        };
        let object = ObjectKind::IndirectObject(IndirectObject {
            object_number: 11,
            generation_number: 0,
            object: Box::new(stream),
        });
        let expected = vec![object];
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_dict_and_float() {
        let text = b"12 0 obj
        <<\n/Type /ExtGState\n/SA false\n/SM 0.02\n/OP false\n/op false\n/OPM 1
        /BG2 /Default\n/UCR2 /Default\n/HT /Default\n/TR2 /Default\n>>\nendobj
        %%EOF";
        let mut lexer = Lexer::new(text);
        let tokens = lexer.lex();
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
        let kind = ObjectKind::IndirectObject(IndirectObject {
            object_number: 12,
            generation_number: 0,
            object: Box::new(dict),
        });
        let expected = vec![offset!(kind)];
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_trailer() {
        let text =
            b"trailer\n<</Root 1 0 R\n/ID [<01234567890ABCDEF> <01234567890ABCDEF>]\n/Size 8\n>>
        startxref\n491
        %%EOF";
        let mut lexer = Lexer::new(text);
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let dict = dict!(
            b"Root" => indirect_reference!(1),
            b"ID" => array![offset!(ObjectKind::String(vec![1, 35, 69, 103, 137, 10, 188, 222, 240])), offset!(ObjectKind::String(vec![1, 35, 69, 103, 137, 10, 188, 222, 240]))],
            b"Size" => integer!(8)
        );
        let dict = inner!(dict, ObjectKind::Dictionary).unwrap();
        let trailer = Trailer {
            size: 8,
            root: IndirectReference {
                object_number: 1,
                generation_number: 0,
            },
            dictionary: dict,
        };
        let trailer = offset!(ObjectKind::Trailer(trailer));
        let startxref = offset!(ObjectKind::StartXref(491));
        let expected = vec![trailer, startxref];
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_cmap_dictionary() {
        let text = b"/CIDSystemInfo\n<< /Registry (Adobe)\n   /Ordering (UCS)\n   /Supplement 0\n>>
        def\n/CMapName";
        let mut lexer = Lexer::new(text);
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let dict = dict!(
            b"Registry" => string!("Adobe"),
            b"Ordering" => string!("UCS"),
            b"Supplement" => integer!(0)
        );
        let def_op = Object {
            kind: ObjectKind::CIDOperator(CIDOperator::Def),
            offset: 0,
        };
        let expected = vec![name!("CIDSystemInfo"), dict, def_op, name!("CMapName")];
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_basic_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/hello.pdf");
        let file = fs::read(d).unwrap();
        let mut lexer = Lexer::new(&file);
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        let stream_content = b"BT\n/F0 12 Tf\n100 700 Td\n(Hello, World) Tj\nET\n".to_vec();
        let expected: Vec<Object> = vec![
            offset!(ObjectKind::Header(Header { major: 1, minor: 4 })),
            indirect_object!(
                1,
                dict!(
                    b"Pages" => indirect_reference!(2, 0),
                    b"Type" => name!("Catalog")
                )
            ),
            indirect_object!(
                2,
                dict!(
                        b"Count" => integer!(1),
                        b"Type" => name!("Pages"),
                        b"Kids" => array![indirect_reference!(3, 0)]
                )
            ),
            indirect_object!(
                3,
                dict!(
                        b"Type" => name!("Page"),
                        b"MediaBox" => array![integer!(0), integer!(0), integer!(612), integer!(792)],
                        b"Parent" => indirect_reference!(2, 0),
                        b"Resources" => indirect_reference!(4, 0),
                        b"Contents" => indirect_reference!(5, 0)
                )
            ),
            indirect_object!(
                4,
                dict!(
                    b"ProcSet" => array!(name!("PDF")),
                    b"Font" => indirect_reference!(6, 0)
                )
            ),
            indirect_object!(
                5,
                stream!(
                    stream_content,
                    b"Length" => indirect_reference!(7, 0)
                )
            ),
            indirect_object!(
                6,
                dict!(
                    b"F0" => indirect_reference!(8, 0)
                )
            ),
            indirect_object!(7, integer!(51)),
            indirect_object!(
                8,
                dict!(
                    b"Type" => name!("Font"),
                    b"Subtype" => name!("Type1"),
                    b"BaseFont" => name!("Helvetica")
                )
            ),
            xref_section![XrefSubsection {
                start_number: 0,
                subsection_length: 8,
                references: vec![
                    xref!(0, 65535, false),
                    xref!(15, 0, true),
                    xref!(64, 0, true),
                    xref!(121, 0, true),
                    xref!(225, 0, true),
                    xref!(274, 0, true),
                    xref!(372, 0, true),
                    xref!(403, 0, true),
                    xref!(421, 0, true),
                ]
            }],
            offset!(ObjectKind::Trailer(Trailer {
                size: 8,
                root: IndirectReference {
                    object_number: 1,
                    generation_number: 0
                },
                dictionary: Dictionary::from([
                    ("Root".into(), indirect_reference!(1, 0)),
                    (
                        "ID".into(),
                        array![
                            offset!(ObjectKind::String(vec![
                                1, 35, 69, 103, 137, 10, 188, 222, 240
                            ])),
                            offset!(ObjectKind::String(vec![
                                1, 35, 69, 103, 137, 10, 188, 222, 240
                            ]))
                        ]
                    ),
                    ("Size".into(), integer!(8))
                ])
            })),
            offset!(ObjectKind::StartXref(491)),
        ];
        assert_eq!(parser.parse(), expected);
    }
}
