use std::borrow::Cow;
use std::convert::TryFrom;
use std::ops::RangeInclusive;
use std::vec::IntoIter;

use super::object::{Object, ObjectKind};
use crate::cmaps::cid::Cid;
use crate::cmaps::cmap::{
    BaseFontChar, BaseFontCharDestination, CMap, CMapWritingMode, CidChar, CidRange,
    CodespaceRange, DEFAULT_CODE_SPACE_RANGE, MAX_CODE_SPACE_LENGTH,
};
use crate::font::font::CidSystemInfo;
use crate::util::reduce_slice_to_numeric;

const WMODE: &[u8] = b"WMode";
const CID_SYSTEM_INFO: &[u8] = b"CIDSystemInfo";
const CMAP_NAME: &[u8] = b"CMapName";
const REGISTRY: &[u8] = b"Registry";
const ORDERING: &[u8] = b"Ordering";
const SUPPLEMENT: &[u8] = b"Supplement";

// beginrearrangedfont, endrearrangedfont, beginusematrix, endusematrix are
// not used in PDF - PDF 9.7.5.4.e
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CIDOperator {
    FindResource,
    Dict,
    Dup,
    Def,
    UseFont,
    UseCMap,
    Begin,
    End,
    BeginCMap,
    EndCMap,
    BeginCodeSpaceRange,
    EndCodeSpaceRange,
    BeginBfChar,
    EndBfChar,
    BeginBfRange,
    EndBfRange,
    BeginCIDChar,
    EndCIDChar,
    BeginCIDRange,
    EndCIDRange,
    BeginNotdefChar,
    EndNotdefChar,
    BeginNotdefRange,
    EndNotdefRange,
}

pub struct CMapFileParser {
    objects: IntoIter<Object>,
    pos: usize,
}

enum CMapStringKind {
    WritingMode,
    CMapName,
}

impl<'a> CMapFileParser {
    pub fn new(objects: Vec<Object>) -> Self {
        Self {
            objects: objects.into_iter(),
            pos: 0,
        }
    }

    fn next(&mut self) -> Option<Object> {
        self.objects.next()
    }

    pub fn parse(mut self) -> Option<CMap<'a>> {
        let mut cmap = CMap::default();
        while let Some(object) = self.next() {
            match &object.kind {
                ObjectKind::Name(name) if name == &WMODE => match self.next()?.kind {
                    ObjectKind::Integer(i) => {
                        if let Ok(writing_mode) = CMapWritingMode::try_from(i) {
                            cmap.writing_mode = writing_mode;
                        }
                    }
                    _ => continue,
                },
                ObjectKind::Name(name) if name == &CID_SYSTEM_INFO => {
                    self.cid_system_info_dict(&mut cmap.cid_system_info)
                }
                ObjectKind::Name(name) if name == &CMAP_NAME => match self.next()?.kind {
                    ObjectKind::Name(name) => cmap.name = name.0.into(),
                    _ => continue,
                },
                ObjectKind::CIDOperator(CIDOperator::BeginCodeSpaceRange) => {
                    self.code_space(cmap.codespace.ranges.to_mut())
                }
                ObjectKind::CIDOperator(CIDOperator::BeginCIDRange) => {
                    self.cid_range(cmap.cid_range.to_mut())
                }
                ObjectKind::CIDOperator(CIDOperator::BeginCIDChar) => {
                    self.cid_char(cmap.cid_chars.to_mut())
                }
                ObjectKind::CIDOperator(CIDOperator::BeginBfChar) => {
                    self.base_font_chars(cmap.base_font_chars.to_mut())
                }
                _ => continue,
            };
        }
        Some(cmap)
    }

    fn cid_char(&mut self, vec: &mut Vec<CidChar>) {
        while let Some(Object {
            kind: ObjectKind::String(char_code),
            ..
        }) = self.next()
        {
            let char_code = reduce_slice_to_numeric(&char_code);
            let Some(object) = self.next() else {
                break
            };
            let cid = match object.kind {
                ObjectKind::Integer(i) => i as Cid,
                _ => break,
            };
            let cid_char = CidChar::new(char_code, cid);
            vec.push(cid_char);
        }
    }

    fn base_font_chars(&mut self, vec: &mut Vec<BaseFontChar>) {
        // CID Font spec 7.4
        // srcCode and destCode must be specified as hexadecimal strings.
        // dstCharname must be a PostScript language name object.
        while let Some(Object {
            kind: ObjectKind::String(char_code),
            ..
        }) = self.next()
        {
            let char_code = reduce_slice_to_numeric(&char_code);
            let dest = if let Some(object) = self.next() {
                match object.kind {
                    ObjectKind::String(string) => {
                        let dest_code = reduce_slice_to_numeric(&string);
                        BaseFontCharDestination::Cid(dest_code)
                    }
                    ObjectKind::Name(name) => BaseFontCharDestination::CharName(name),
                    _ => break,
                }
            } else {
                break;
            };
            let base_font_char = BaseFontChar::new(char_code, dest);
            vec.push(base_font_char);
        }
    }

    fn cid_range(&mut self, vec: &mut Vec<CidRange>) {
        while let Some(Object {
            kind: ObjectKind::String(start),
            ..
        }) = self.next()
        {
            let start = reduce_slice_to_numeric(&start);
            let Some(object) = self.next() else {
                break
            };
            let ObjectKind::String(end) = object.kind else {
                break
            };
            let end = reduce_slice_to_numeric(&end);
            let Some(object) = self.next() else {
                break
            };
            let cid = match object.kind {
                ObjectKind::Integer(i) => i as u32,
                _ => break,
            };
            let range = CidRange::new(start, end, cid);
            vec.push(range);
        }
    }

    fn code_space(&mut self, vec: &mut Vec<CodespaceRange>) {
        while let Some(Object {
            kind: ObjectKind::String(first),
            ..
        }) = self.next()
        {
            let Some(object) = self.next() else {
                break
            };
            let ObjectKind::String(second) = object.kind else {
                break
            };
            let mut range = [DEFAULT_CODE_SPACE_RANGE; MAX_CODE_SPACE_LENGTH];
            // We backfill the codespace range so if the bytes are 00 to 80
            // then the codespace range will be [0..0, 0..0, 0..0, 00..=80]
            // If the bytes are abcdefab to fedcbaef then the range will be
            // [ab..=ef, ef..=ba, cd..=dc, ab..=fe]
            for (i, (&lower, &upper)) in first.iter().zip(second.iter()).rev().enumerate() {
                range[MAX_CODE_SPACE_LENGTH - 1 - i] = RangeInclusive::new(lower, upper);
            }
            vec.push(range);
        }
    }

    fn match_cid_system_info_pair(
        &mut self,
        name: Vec<u8>,
        value: &Object,
        cid_info: &mut CidSystemInfo<'a>,
    ) {
        match name.as_ref() {
            REGISTRY => {
                if let Ok(v) = <&Vec<u8>>::try_from(value) {
                    cid_info.registry = Cow::Owned(v.clone());
                }
            }
            ORDERING => {
                if let Ok(v) = <&Vec<u8>>::try_from(value) {
                    cid_info.ordering = Cow::Owned(v.clone());
                }
            }
            SUPPLEMENT => {
                if let Ok(i) = u32::try_from(value) {
                    cid_info.supplement = i;
                }
            }
            _ => (),
        }
    }

    fn cid_system_info_dict(&mut self, cid_info: &mut CidSystemInfo<'a>) {
        while let Some(object) = self.next() {
            if let ObjectKind::Name(name) = object.kind {
                let Some(next) = self.next() else {
                    break
                };
                self.match_cid_system_info_pair(name.0, &next, cid_info);
            } else if let ObjectKind::Dictionary(dict) = object.kind {
                for (key, val) in dict.0.into_iter() {
                    self.match_cid_system_info_pair(key, &val, cid_info);
                }
                // Return after parsing the dictionary since there is only
                // one dictionary per CID system info
                break;
            } else if let ObjectKind::CIDOperator(CIDOperator::End) = &object.kind {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmaps::cmap::Codespace;
    use crate::operators::operators::Operator;
    use crate::{
        dict, integer, name, offset,
        parser::object::{Object, ObjectKind},
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
            offset!(ObjectKind::CIDOperator(CIDOperator::BeginCodeSpaceRange)),
            offset!(ObjectKind::String(vec![0x0])),
            offset!(ObjectKind::String(vec![0x80])),
            offset!(ObjectKind::CIDOperator(CIDOperator::EndCodeSpaceRange)),
            name!("CMapName"),
            name!("Adobe-Identity-UCS"),
        ];
        let parser = CMapFileParser::new(objects);
        let cmap = parser.parse().unwrap();
        assert_eq!(cmap.cid_system_info.registry.to_vec(), b"Adobe");
        assert_eq!(cmap.cid_system_info.ordering.to_vec(), b"UCS");
        assert_eq!(cmap.cid_system_info.supplement, 0);
        assert_eq!(cmap.name.to_vec(), b"Adobe-Identity-UCS");
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
            offset!(ObjectKind::CIDOperator(CIDOperator::BeginCodeSpaceRange)),
            offset!(ObjectKind::String(vec![0x0])),
            offset!(ObjectKind::String(vec![0x80])),
            offset!(ObjectKind::String(vec![0x81, 0x40])),
            offset!(ObjectKind::String(vec![0x9f, 0xfc])),
            offset!(ObjectKind::String(vec![0xa0])),
            offset!(ObjectKind::String(vec![0xde])),
            offset!(ObjectKind::String(vec![0xe0, 0x40])),
            offset!(ObjectKind::String(vec![0xfb, 0xec])),
            offset!(ObjectKind::CIDOperator(CIDOperator::EndCodeSpaceRange)),
        ];
        let parser = CMapFileParser::new(objects);
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
    fn test_cid_parser_bf() {
        let objects = vec![
            name!("CIDInit"),
            name!("ProcSet"),
            offset!(ObjectKind::CIDOperator(CIDOperator::FindResource)),
            offset!(ObjectKind::CIDOperator(CIDOperator::Begin)),
            integer!(12),
            offset!(ObjectKind::CIDOperator(CIDOperator::Dict)),
            offset!(ObjectKind::CIDOperator(CIDOperator::Begin)),
            offset!(ObjectKind::CIDOperator(CIDOperator::BeginCMap)),
            name!("CIDSystemInfo"),
            dict!(
                b"Ordering" => string!("USC"),
                b"Registry" => string!("Adobe"),
                b"Supplement" => integer!(0)
            ),
            offset!(ObjectKind::CIDOperator(CIDOperator::Def)),
            name!("CMapName"),
            name!("Adobe-Identity-UCS"),
            offset!(ObjectKind::CIDOperator(CIDOperator::Def)),
            name!("CMapType"),
            integer!(2),
            offset!(ObjectKind::CIDOperator(CIDOperator::Def)),
            integer!(1),
            offset!(ObjectKind::CIDOperator(CIDOperator::BeginCodeSpaceRange)),
            Object {
                offset: 212,
                kind: ObjectKind::String(vec![0]),
            },
            Object {
                offset: 216,
                kind: ObjectKind::String(vec![255]),
            },
            offset!(ObjectKind::CIDOperator(CIDOperator::EndCodeSpaceRange)),
            Object {
                offset: 235,
                kind: ObjectKind::Operator(Operator::EndPathNoFill),
            },
            Object {
                offset: 236,
                kind: ObjectKind::Operator(Operator::SetGrayNonstroke),
            },
            offset!(ObjectKind::CIDOperator(CIDOperator::BeginBfChar)),
            Object {
                offset: 253,
                kind: ObjectKind::String(vec![34]),
            },
            Object {
                offset: 257,
                kind: ObjectKind::String(vec![6, 68, 6, 39]),
            },
            offset!(ObjectKind::CIDOperator(CIDOperator::EndBfChar)),
            integer!(2),
            offset!(ObjectKind::CIDOperator(CIDOperator::BeginBfRange)),
            Object {
                offset: 294,
                kind: ObjectKind::String(vec![33]),
            },
            Object {
                offset: 298,
                kind: ObjectKind::String(vec![33]),
            },
            Object {
                offset: 302,
                kind: ObjectKind::String(vec![6, 69]),
            },
            Object {
                offset: 309,
                kind: ObjectKind::String(vec![35]),
            },
            Object {
                offset: 313,
                kind: ObjectKind::String(vec![35]),
            },
            Object {
                offset: 317,
                kind: ObjectKind::String(vec![6, 51]),
            },
            offset!(ObjectKind::CIDOperator(CIDOperator::End)),
            offset!(ObjectKind::CIDOperator(CIDOperator::Def)),
            offset!(ObjectKind::CIDOperator(CIDOperator::End)),
            offset!(ObjectKind::CIDOperator(CIDOperator::End)),
        ];
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
        let cmap_parser = CMapFileParser::new(objects);
        let cmap = cmap_parser.parse().unwrap();
        assert!(false);
    }
}
