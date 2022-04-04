use std::{collections::BTreeMap, convert::TryFrom};

use crate::{
    error::PdfError,
    parser::parser::{Name, Object},
};

use super::font::{CharCode, Cid};

#[derive(Debug, Clone, Copy, PartialEq)]
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
    BeginRearrangedFont,
    EndRearrangedFont,
    BeginUseMatrix,
    EndUseMatrix,
}

#[derive(Debug)]
pub enum CMapWritingMode {
    Horizontal,
    Vertical,
}

impl Default for CMapWritingMode {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl TryFrom<i64> for CMapWritingMode {
    type Error = PdfError;

    fn try_from(i: i64) -> Result<Self, Self::Error> {
        // "An entry of 0 defines horizontal writing from left to right; an
        // entry of 1 defines vertical writing from top to bottom. Other values
        // of for WMode are reserved." (Adobe Technical Note #5014)
        match i {
            0 => Ok(Self::Horizontal),
            1 => Ok(Self::Vertical),
            _ => Err("Unrecognized integer for WMode in CMap file."
                .to_owned()
                .into()),
        }
    }
}

pub type CMap = BTreeMap<CharCode, Cid>;

// "It is equivalent to the concept of an encoding in simple fonts. Whereas a
// simple font allows a maximum of 256 glyphs to be encoded and accessible at
// one time, a CMap can describe a mapping from multiple-byte codes to
// thousands of glyphs in a large CID-keyed font." (PDF 9.7.2)
#[derive(Debug)]
pub struct CMapFile<'a> {
    pub name: &'a Name,
    pub cid_system_info: BTreeMap<&'a Name, &'a Object>,
    // "Writing mode is specified as part of the CMap because, in some cases,
    // different shapes are used when writing horizontally and vertically.
    // In such cases, the horizontal and vertical variants of a CMap specify
    // different CIDs for a given character code." (PDF 9.7.5.1)
    pub writing_mode: CMapWritingMode,
    pub cmap: CMap,
}

impl<'a> CMapFile<'a> {
    pub fn new(
        name: &'a Name,
        cid_system_info: BTreeMap<&'a Name, &'a Object>,
        writing_mode: CMapWritingMode,
        cmap: CMap,
    ) -> Self {
        Self {
            name,
            cid_system_info,
            writing_mode,
            cmap,
        }
    }
}
