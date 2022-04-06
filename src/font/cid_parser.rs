use std::collections::BTreeMap;
use std::convert::TryFrom;

use crate::parser::parser::{Name, Object};
use crate::util::slice_to_numeric;
use crate::{parser::parser::ObjectKind};

use super::cmap::{CIDOperator, CMap, CMapFile, CMapWritingMode};

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
        );
        Some(cmap_file)
    }

    fn code_space(&mut self) -> Option<()> {
        let mut lower_range = vec![];
        let mut upper_range = vec![];
        while let Some(first) = self.pop() {
            let first = match &first.kind {
                ObjectKind::String(string) => string,
                _ => break,
            };
            let second = match &self.pop()?.kind {
                ObjectKind::String(string) => string,
                _ => break,
            };
            for bytes in first.chunks_exact(2) {
                let lower = slice_to_numeric(bytes, 16)?;
                lower_range.push(lower);
            }
            for bytes in second.chunks_exact(2) {
                let upper = slice_to_numeric(bytes, 16)?;
                upper_range.push(upper);
            }
        }
        None
    }

    fn name(&mut self, name: &'a Name) -> Option<()> {
        if name == WMODE {
            match self.pop()?.kind {
                ObjectKind::Integer(i) => self.writing_mode = CMapWritingMode::try_from(i).ok()?,
                _ => return None
            }
        } else if name == CID_SYSTEM_INFO {
            self.cid_system_info_dict();
        } else if name == CMAP_NAME {
            match &self.pop()?.kind {
                ObjectKind::Name(name) => self.name = Some(name),
                _ => return None
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
                },
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
    use crate::{offset, dict, string, integer, name, parser::parser::{Object, ObjectKind}, font::{cmap::{CIDOperator}, cid_parser::CMapFileParser}};

    #[test]
    fn test_cid_parser_pdf_dictionary() {
        let dict = dict!(
            b"Registry" => string!("Adobe"),
            b"Ordering" => string!("UCS"),
            b"Supplement" => integer!(0)
        );
        let def_op = Object {
            kind: ObjectKind::CIDOperator(CIDOperator::Def),
            offset: 0
        };
        let objects = vec![name!("CIDSystemInfo"), dict, def_op, name!("CMapName"), name!("Adobe-Identity-UCS")];
        let parser = CMapFileParser::new(&objects);
        let cmap_file = parser.parse().unwrap();
        assert_eq!(cmap_file.cid_system_info.get(&b"Registry".to_vec()).unwrap(), &&string!("Adobe"));
        assert_eq!(cmap_file.cid_system_info.get(&b"Ordering".to_vec()).unwrap(), &&string!("UCS"));
        assert_eq!(cmap_file.cid_system_info.get(&b"Supplement".to_vec()).unwrap(), &&integer!(0));
        assert_eq!(cmap_file.cid_system_info.len(), 3);
        assert_eq!(cmap_file.name, b"Adobe-Identity-UCS");
    }
}
