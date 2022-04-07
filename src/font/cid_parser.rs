use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::ops::RangeInclusive;

use crate::parser::parser::ObjectKind;
use crate::parser::parser::{Name, Object};
use crate::util::slice_to_numeric;

use super::cmap::{CIDOperator, CMap, CMapFile, CMapWritingMode, Codespace};

const WMODE: &[u8] = b"WMode";
const CID_SYSTEM_INFO: &[u8] = b"CIDSystemInfo";
const CMAP_NAME: &[u8] = b"CMapName";

pub struct CMapFileParser<'a> {
    objects: &'a [Object],
    pos: usize,
    cmap: CMap,
    name: Option<&'a Name>,
    cid_system_info: BTreeMap<&'a Name, &'a Object>,
    writing_mode: CMapWritingMode,
    codespace: Codespace,
}

impl<'a> CMapFileParser<'a> {
    pub fn new(objects: &'a [Object]) -> Self {
        Self {
            objects,
            pos: 0,
            name: None,
            cid_system_info: BTreeMap::new(),
            writing_mode: CMapWritingMode::default(),
            cmap: CMap::new(),
            codespace: Codespace::new(),
        }
    }

    pub fn parse(mut self) -> Option<CMapFile<'a>> {
        while let Some(object) = self.pop() {
            match &object.kind {
                ObjectKind::Name(name) => self.name(name),
                ObjectKind::CIDOperator(CIDOperator::BeginCodeSpaceRange) => self.code_space(),
                _ => continue,
            };
        }
        let cmap_file = CMapFile::new(
            self.name?,
            self.cid_system_info,
            self.writing_mode,
            self.cmap,
            self.codespace,
        );
        Some(cmap_file)
    }

    fn code_space(&mut self) -> Option<()> {
        while let Some(first) = self.pop() {
            let first = match &first.kind {
                ObjectKind::String(string) => string,
                _ => break,
            };
            let second = match &self.pop()?.kind {
                ObjectKind::String(string) => string,
                _ => break,
            };
            let len = first.len() as u8 / 2;
            let mut ranges = vec![];
            let chunks = first.chunks_exact(2).zip(second.chunks_exact(2));
            for (lower, upper) in chunks {
                let lower = slice_to_numeric(lower, 16)?;
                let upper = slice_to_numeric(upper, 16)?;
                let range = RangeInclusive::new(lower, upper);
                ranges.push(range);
            }
            self.codespace.entry(len).or_default().push(ranges);
        }
        None
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

    fn cid_system_info_dict(&mut self) {
        while let Some(object) = self.pop() {
            match &object.kind {
                ObjectKind::Name(name) => {
                    if let Some(value) = self.pop() {
                        self.cid_system_info.insert(name, value);
                    }
                }
                ObjectKind::CIDOperator(CIDOperator::End) => break,
                ObjectKind::Dictionary(dict) => {
                    for (key, value) in dict {
                        self.cid_system_info.insert(key, value);
                    }
                    break;
                }
                _ => continue,
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
    use super::*;
    use crate::{
        dict,
        font::{cid_parser::CMapFileParser, cmap::CIDOperator},
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
            name!("CMapName"),
            name!("Adobe-Identity-UCS"),
        ];
        let parser = CMapFileParser::new(&objects);
        let cmap_file = parser.parse().unwrap();
        assert_eq!(
            cmap_file
                .cid_system_info
                .get(&b"Registry".to_vec())
                .unwrap(),
            &&string!("Adobe")
        );
        assert_eq!(
            cmap_file
                .cid_system_info
                .get(&b"Ordering".to_vec())
                .unwrap(),
            &&string!("UCS")
        );
        assert_eq!(
            cmap_file
                .cid_system_info
                .get(&b"Supplement".to_vec())
                .unwrap(),
            &&integer!(0)
        );
        assert_eq!(cmap_file.cid_system_info.len(), 3);
        assert_eq!(cmap_file.name, b"Adobe-Identity-UCS");
    }

    #[test]
    fn test_cid_parser_codespace() {
        let objects = vec![
            name!("CMapName"),
            name!("Adobe-Identity-UCS"),
            integer!(4),
            Object {
                kind: ObjectKind::CIDOperator(CIDOperator::BeginCodeSpaceRange),
                offset: 0,
            },
            string!("00"),
            string!("80"),
            string!("8140"),
            string!("9ffc"),
            string!("a0"),
            string!("de"),
            string!("e040"),
            string!("fbec"),
            Object {
                kind: ObjectKind::CIDOperator(CIDOperator::EndCodeSpaceRange),
                offset: 0,
            },
        ];
        let parser = CMapFileParser::new(&objects);
        let cmap_file = parser.parse().unwrap();
        let expected = Codespace::from([
            (
                1,
                vec![
                    vec![RangeInclusive::new(0, 0x80)],
                    vec![RangeInclusive::new(0xa0, 0xde)],
                ],
            ),
            (
                2,
                vec![
                    vec![
                        RangeInclusive::new(0x81, 0x9f),
                        RangeInclusive::new(0x40, 0xfc),
                    ],
                    vec![
                        RangeInclusive::new(0xe0, 0xfb),
                        RangeInclusive::new(0x40, 0xec),
                    ],
                ],
            ),
        ]);
        assert_eq!(cmap_file.codespace, expected);
    }
}
