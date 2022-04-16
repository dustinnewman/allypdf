use std::{
    collections::BTreeMap,
    convert::TryFrom,
    ops::{Deref, DerefMut, RangeInclusive},
};

use crate::{
    error::PdfError,
    parser::parser::{Name, Object},
};

use super::font::{CharCode, Cid};

pub const MAX_CODE_SPACE_LENGTH: usize = 4;

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

pub trait CharCodeToCid {
    fn get_cid(&self, char_code: CharCode) -> Option<Cid>;
}

pub trait CharCodeToGlyphName {
    fn get_glyph_name(&self, char_code: CharCode) -> Option<&[u8]>;
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

#[derive(Debug)]
pub struct CidRange {
    start: CharCode,
    end: CharCode,
    cid: Cid,
}

impl CidRange {
    pub const fn new(start: CharCode, end: CharCode, cid: Cid) -> Self {
        Self { start, end, cid }
    }
}

pub type CMap = BTreeMap<CharCode, Cid>;

impl CharCodeToCid for CMap {
    fn get_cid(&self, char_code: CharCode) -> Option<Cid> {
        self.get(&char_code).and_then(|&cid| Some(cid))
    }
}

// "The code length shall not be greater than 4." - PDF 9.7.6.2
// For efficiency, we use this 4 as a universal size instead of supporting a
// hash map implementation. For code lengths less than 4, we just left pad
// with zeros.
pub type CodespaceRange = [RangeInclusive<u8>; MAX_CODE_SPACE_LENGTH];

#[derive(Debug, PartialEq)]
pub struct Codespace(Vec<CodespaceRange>);

impl Codespace {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn from(ranges: Vec<CodespaceRange>) -> Self {
        Self(ranges)
    }

    // Take a list of bytes and see if the codespace contains these bytes.
    // For the code space to contain a list of bytes, then each byte must
    // be contained within its corresponding range.
    pub fn contains(&self, bytes: &[u8; MAX_CODE_SPACE_LENGTH]) -> bool {
        for range in &self.0 {
            if range
                .iter()
                .zip(bytes)
                .all(|(dim, byte)| dim.contains(byte))
            {
                return true;
            }
        }
        false
    }
}

impl Deref for Codespace {
    type Target = Vec<CodespaceRange>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Codespace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
    pub codespace: Codespace,
}

impl<'a> CMapFile<'a> {
    pub fn new(
        name: &'a Name,
        cid_system_info: BTreeMap<&'a Name, &'a Object>,
        writing_mode: CMapWritingMode,
        cmap: CMap,
        codespace: Codespace,
    ) -> Self {
        Self {
            name,
            cid_system_info,
            writing_mode,
            cmap,
            codespace,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmap_codespace_contains() {
        let first_range = [
            RangeInclusive::new(0x00, 0x00),
            RangeInclusive::new(0x00, 0x00),
            RangeInclusive::new(0x00, 0x00),
            RangeInclusive::new(0x00, 0x80),
        ];
        let second_range = [
            RangeInclusive::new(0x00, 0x00),
            RangeInclusive::new(0x00, 0x00),
            RangeInclusive::new(0x00, 0x00),
            RangeInclusive::new(0xA0, 0xDF),
        ];
        let third_range = [
            RangeInclusive::new(0x00, 0x00),
            RangeInclusive::new(0x00, 0x00),
            RangeInclusive::new(0x81, 0x9f),
            RangeInclusive::new(0x40, 0xfc),
        ];
        let fourth_range = [
            RangeInclusive::new(0x00, 0x00),
            RangeInclusive::new(0x00, 0x00),
            RangeInclusive::new(0xe0, 0xfb),
            RangeInclusive::new(0x40, 0xec),
        ];
        let ranges = vec![first_range, second_range, third_range, fourth_range];
        let codespace = Codespace::from(ranges);
        assert!(codespace.contains(&[0, 0, 0, 0x79]));
        assert!(codespace.contains(&[0, 0, 0x86, 0xA9]));
        assert!(!codespace.contains(&[0, 0, 0x80, 0x10]));
        assert!(!codespace.contains(&[0, 0, 0x82, 0x10]));
    }
}
