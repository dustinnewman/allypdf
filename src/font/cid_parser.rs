use std::convert::TryFrom;
use std::ops::RangeInclusive;

use super::cid_operator::CIDOperator;
use super::font::CidSystemInfo;
use crate::cmaps::cid::Cid;
use crate::cmaps::cmap::{
    CMap, CMapWritingMode, CidChar, CidRange, Codespace, DEFAULT_CODE_SPACE_RANGE,
    MAX_CODE_SPACE_LENGTH,
};
use crate::parser::parser::ObjectKind;
use crate::parser::parser::{Name, Object};
use crate::util::reduce_slice_to_numeric;

const WMODE: &[u8] = b"WMode";
const CID_SYSTEM_INFO: &[u8] = b"CIDSystemInfo";
const CMAP_NAME: &[u8] = b"CMapName";
const REGISTRY: &[u8] = b"Registry";
const ORDERING: &[u8] = b"Ordering";
const SUPPLEMENT: &[u8] = b"Supplement";

pub struct CMapFileParser<'a> {
    objects: &'a [Object],
    pos: usize,
    name: Option<&'a Name>,
    registry: Option<&'a Name>,
    ordering: Option<&'a Name>,
    supplement: Option<u32>,
    writing_mode: CMapWritingMode,
    codespace: Option<Codespace<'a>>,
    cid_range: Vec<CidRange>,
    cid_chars: Vec<CidChar>,
}

impl<'a> CMapFileParser<'a> {
    pub fn new(objects: &'a [Object]) -> Self {
        Self {
            objects,
            pos: 0,
            name: None,
            registry: None,
            ordering: None,
            supplement: None,
            writing_mode: CMapWritingMode::default(),
            codespace: None,
            cid_range: vec![],
            cid_chars: vec![],
        }
    }

    pub fn parse(mut self) -> Option<CMap<'a>> {
        while let Some(object) = self.pop() {
            match &object.kind {
                ObjectKind::Name(name) => self.name(name),
                ObjectKind::CIDOperator(CIDOperator::BeginCodeSpaceRange) => self.code_space(),
                ObjectKind::CIDOperator(CIDOperator::BeginCIDRange) => self.cid_range(),
                ObjectKind::CIDOperator(CIDOperator::BeginCIDChar) => self.cid_char(),
                _ => continue,
            };
        }
        let cmap = CMap {
            name: self.name?,
            cid_system_info: CidSystemInfo {
                registry: self.registry?,
                ordering: self.ordering?,
                supplement: self.supplement?,
            },
            writing_mode: self.writing_mode,
            codespace: self.codespace.unwrap_or_default(),
            cid_chars: self.cid_chars.into(),
            cid_range: self.cid_range.into(),
        };
        Some(cmap)
    }

    fn cid_char(&mut self) -> Option<()> {
        while let Some(object) = self.pop() {
            let char_code = match &object.kind {
                ObjectKind::String(string) => string,
                _ => break,
            };
            let char_code = reduce_slice_to_numeric(char_code);
            let cid = match &self.pop()?.kind {
                ObjectKind::Integer(i) => *i as Cid,
                _ => break,
            };
            let cid_char = CidChar::new(char_code, cid);
            self.cid_chars.push(cid_char);
        }
        Some(())
    }

    fn cid_range(&mut self) -> Option<()> {
        while let Some(start) = self.pop() {
            let start = match &start.kind {
                ObjectKind::String(string) => string,
                _ => break,
            };
            let start = reduce_slice_to_numeric(start);
            let end = match &self.pop()?.kind {
                ObjectKind::String(string) => string,
                _ => break,
            };
            let end = reduce_slice_to_numeric(end);
            let cid = match &self.pop()?.kind {
                ObjectKind::Integer(i) => *i as u32,
                _ => break,
            };
            let range = CidRange::new(start, end, cid);
            self.cid_range.push(range);
        }
        Some(())
    }

    fn code_space(&mut self) -> Option<()> {
        let mut ranges = vec![];
        while let Some(first) = self.pop() {
            let first = match &first.kind {
                ObjectKind::String(string) => string,
                _ => break,
            };
            let second = match &self.pop()?.kind {
                ObjectKind::String(string) => string,
                _ => break,
            };
            let mut range = [DEFAULT_CODE_SPACE_RANGE; MAX_CODE_SPACE_LENGTH];
            // We backfill the codespace range so if the bytes are 00 to 80
            // then the codespace range will be [0..0, 0..0, 0..0, 00..=80]
            // If the bytes are abcdefab to fedcbaef then the range will be
            // [ab..=ef, ef..=ba, cd..=dc, ab..=fe]
            for (i, (&lower, &upper)) in first.iter().zip(second.iter()).rev().enumerate() {
                range[MAX_CODE_SPACE_LENGTH - 1 - i] = RangeInclusive::new(lower, upper);
            }
            ranges.push(range);
        }
        self.codespace = Some(Codespace::from(ranges.into()));
        Some(())
    }

    fn name(&mut self, name: &'a Name) -> Option<()> {
        if name == WMODE {
            match self.pop()?.kind {
                ObjectKind::Integer(i) => self.writing_mode = CMapWritingMode::try_from(i).ok()?,
                _ => return None,
            }
        } else if name == CID_SYSTEM_INFO {
            self.cid_system_info_dict();
        } else if name == CMAP_NAME {
            match &self.pop()?.kind {
                ObjectKind::Name(name) => self.name = Some(name),
                _ => return None,
            };
        }
        Some(())
    }

    fn match_cid_system_info_pair(&mut self, name: &Name, value: Option<&'a Object>) {
        if name == REGISTRY {
            if let Some(Object {
                kind: ObjectKind::String(reg),
                ..
            }) = value
            {
                self.registry = Some(reg);
            }
        } else if name == ORDERING {
            if let Some(Object {
                kind: ObjectKind::String(ord),
                ..
            }) = value
            {
                self.ordering = Some(ord);
            }
        } else if name == SUPPLEMENT {
            if let Some(Object {
                kind: ObjectKind::Integer(i),
                ..
            }) = value
            {
                self.supplement = Some(*i as u32);
            }
        }
    }

    fn cid_system_info_dict(&mut self) {
        while let Some(object) = self.pop() {
            if let ObjectKind::Name(name) = &object.kind {
                let next = self.pop();
                self.match_cid_system_info_pair(name, next);
            } else if let ObjectKind::Dictionary(dict) = &object.kind {
                for (key, val) in dict.iter() {
                    self.match_cid_system_info_pair(key, Some(val));
                }
                // Return after parsing the dictionary since there is only
                // one dictionary per CID system info
                break;
            } else if let ObjectKind::CIDOperator(CIDOperator::End) = &object.kind {
                break;
            }
        }
    }

    fn pop(&mut self) -> Option<&'a Object> {
        if self.pos == self.objects.len() {
            return None;
        }
        self.advance();
        Some(&self.objects[self.pos - 1])
    }

    fn advance(&mut self) {
        self.seek(1);
    }

    fn seek(&mut self, n: usize) {
        if self.pos + n <= self.objects.len() {
            self.pos += n;
        }
    }

    fn peek(&self) -> Option<&'a Object> {
        self.nth(0)
    }

    fn nth(&self, n: usize) -> Option<&'a Object> {
        if self.pos + n < self.objects.len() {
            Some(&self.objects[self.pos + n])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::{
        dict,
        font::{cid_operator::CIDOperator, cid_parser::CMapFileParser},
        integer, name, offset,
        parser::parser::{Object, ObjectKind},
        string,
    };

    #[test]
    fn test_cid_parser_pdf_dictionary() {
        let dict = dict!(
            b"Registry" => string!("Adobe"),
            b"Ordering" => string!("UCS"),
            b"Supplement" => integer!(0)
        );
        let def_op = Object {
            kind: ObjectKind::CIDOperator(CIDOperator::Def),
            offset: 0,
        };
        let objects = vec![
            name!("CIDSystemInfo"),
            dict,
            def_op,
            integer!(1),
            Object {
                kind: ObjectKind::CIDOperator(CIDOperator::BeginCodeSpaceRange),
                offset: 0,
            },
            Object {
                kind: ObjectKind::String(vec![0x0]),
                offset: 0,
            },
            Object {
                kind: ObjectKind::String(vec![0x80]),
                offset: 0,
            },
            Object {
                kind: ObjectKind::CIDOperator(CIDOperator::EndCodeSpaceRange),
                offset: 0,
            },
            name!("CMapName"),
            name!("Adobe-Identity-UCS"),
        ];
        let parser = CMapFileParser::new(&objects);
        let cmap = parser.parse().unwrap();
        assert_eq!(cmap.cid_system_info.registry, b"Adobe");
        assert_eq!(cmap.cid_system_info.ordering, b"UCS");
        assert_eq!(cmap.cid_system_info.supplement, 0);
        assert_eq!(cmap.name, b"Adobe-Identity-UCS");
    }

    #[test]
    fn test_cid_parser_codespace() {
        let objects = vec![
            name!("CIDSystemInfo"),
            dict!(
                b"Registry" => string!("Adobe"),
                b"Ordering" => string!("UCS"),
                b"Supplement" => integer!(0)
            ),
            name!("CMapName"),
            name!("Adobe-Identity-UCS"),
            integer!(4),
            Object {
                kind: ObjectKind::CIDOperator(CIDOperator::BeginCodeSpaceRange),
                offset: 0,
            },
            Object {
                kind: ObjectKind::String(vec![0x0]),
                offset: 0,
            },
            Object {
                kind: ObjectKind::String(vec![0x80]),
                offset: 0,
            },
            Object {
                kind: ObjectKind::String(vec![0x81, 0x40]),
                offset: 0,
            },
            Object {
                kind: ObjectKind::String(vec![0x9f, 0xfc]),
                offset: 0,
            },
            Object {
                kind: ObjectKind::String(vec![0xa0]),
                offset: 0,
            },
            Object {
                kind: ObjectKind::String(vec![0xde]),
                offset: 0,
            },
            Object {
                kind: ObjectKind::String(vec![0xe0, 0x40]),
                offset: 0,
            },
            Object {
                kind: ObjectKind::String(vec![0xfb, 0xec]),
                offset: 0,
            },
            Object {
                kind: ObjectKind::CIDOperator(CIDOperator::EndCodeSpaceRange),
                offset: 0,
            },
        ];
        let parser = CMapFileParser::new(&objects);
        let cmap = parser.parse().unwrap();
        let ranges = vec![
            [
                RangeInclusive::new(0x00, 0x00),
                RangeInclusive::new(0x00, 0x00),
                RangeInclusive::new(0x00, 0x00),
                RangeInclusive::new(0x00, 0x80),
            ],
            [
                RangeInclusive::new(0x00, 0x00),
                RangeInclusive::new(0x00, 0x00),
                RangeInclusive::new(0x81, 0x9f),
                RangeInclusive::new(0x40, 0xfc),
            ],
            [
                RangeInclusive::new(0x00, 0x00),
                RangeInclusive::new(0x00, 0x00),
                RangeInclusive::new(0x00, 0x00),
                RangeInclusive::new(0xa0, 0xde),
            ],
            [
                RangeInclusive::new(0x00, 0x00),
                RangeInclusive::new(0x00, 0x00),
                RangeInclusive::new(0xe0, 0xfb),
                RangeInclusive::new(0x40, 0xec),
            ],
        ];
        let expected = Codespace::from(ranges.into());
        assert_eq!(cmap.codespace, expected);
        assert!(cmap.codespace.contains(0x79));
        assert!(cmap.codespace.contains(0x86A9));
        assert!(!cmap.codespace.contains(0x8010));
        assert!(!cmap.codespace.contains(0x8210));
    }

    #[test]
    fn test_cid_parser_predefined_cmap() {
        let args: Vec<String> = std::env::args().collect();
        let text = std::fs::read_to_string(&args[2]).unwrap();
        let text = text.as_bytes();
        let mut lexer = crate::parser::lexer::Lexer::new(text);
        let tokens = lexer.lex();
        let mut parser = crate::parser::parser::Parser::new(&tokens);
        let objects = parser.parse();
        let cmap_parser = CMapFileParser::new(&objects);
        let cmap = cmap_parser.parse().unwrap();
        println!("{}", std::str::from_utf8(cmap.name).unwrap());
        println!(
            "{}",
            std::str::from_utf8(cmap.cid_system_info.registry).unwrap()
        );
        println!(
            "{}",
            std::str::from_utf8(cmap.cid_system_info.ordering).unwrap()
        );
        println!("{}", cmap.cid_system_info.supplement);
        println!("{:?}", cmap.writing_mode);
        println!("{:?}", cmap.codespace.ranges);
        println!("{:?}", cmap.cid_chars);
        println!("{:?}", cmap.cid_range);
        assert!(false);
    }
}
