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

#[derive(Debug, PartialEq)]
pub struct Codespace(BTreeMap<u8, Vec<Vec<RangeInclusive<u8>>>>);

impl Codespace {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    // Take a list of bytes and see if the codespace contains these bytes.
    // For the code space to contain a list of bytes, then each byte must
    // be contained within its corresponding range. For example, for a one
    // byte value, we only need to check if there is any range which contains
    // this value. For two or more bytes, however, we must check if there is
    // any range containing these bytes in each dimension. A two-byte value
    // must have its first byte contained within the first range of the list
    // AND its second byte contained within the second range of the list.
    // There are lists of such lists, so we must iterate all of them (until we
    // find a match)
    pub fn contains(&self, bytes: &[u8]) -> bool {
        let len = bytes.len() as u8;
        if let Some(ranges) = self.0.get(&len) {
            for range in ranges {
                if range
                    .iter()
                    .zip(bytes)
                    .all(|(dimension, byte)| dimension.contains(byte))
                {
                    return true;
                }
            }
        }
        return false;
    }
}

impl Deref for Codespace {
    type Target = BTreeMap<u8, Vec<Vec<RangeInclusive<u8>>>>;

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
        let mut codespace = Codespace::new();
        codespace.insert(
            1,
            vec![
                vec![RangeInclusive::new(0x00, 0x80)],
                vec![RangeInclusive::new(0xA0, 0xDF)],
            ],
        );
        codespace.insert(
            2,
            vec![
                vec![
                    RangeInclusive::new(0x81, 0x9F),
                    RangeInclusive::new(0x40, 0xFC),
                ],
                vec![
                    RangeInclusive::new(0xE0, 0xFB),
                    RangeInclusive::new(0x40, 0xEC),
                ],
            ],
        );
        assert!(codespace.contains(&[0x79]));
        assert!(codespace.contains(&[0x86, 0xA9]));
        assert!(!codespace.contains(&[0x80, 0x10]));
        assert!(!codespace.contains(&[0x82, 0x10]));
    }
}
