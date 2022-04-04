use std::collections::BTreeMap;
use std::convert::TryFrom;

use crate::parser::parser::{Dictionary, Name, Object};
use crate::{error::PdfError, parser::parser::ObjectKind};

use super::cmap::{CIDOperator, CMap, CMapFile, CMapWritingMode};

const WMODE: &[u8] = b"WMode";
const CID_SYSTEM_INFO: &[u8] = b"CIDSystemInfo";

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
                _ => break,
            }
        }
        let cmap_file = CMapFile::new(
            self.name?,
            self.cid_system_info,
            self.writing_mode,
            self.cmap,
        );
        Some(cmap_file)
    }

    fn name(&mut self, name: &'a Name) {
        if name == WMODE {
            if let Some(Object {
                kind: ObjectKind::Integer(i),
                ..
            }) = self.pop()
            {
                if let Ok(writing_mode) = CMapWritingMode::try_from(*i) {
                    self.writing_mode = writing_mode;
                }
            }
        } else if name == CID_SYSTEM_INFO {
            self.cid_system_info_dict();
        }
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
mod tests {}
