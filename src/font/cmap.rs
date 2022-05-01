use std::convert::TryInto;
use std::{borrow::Cow, convert::TryFrom, ops::RangeInclusive};

use super::font::{CharCode, Cid, CidSystemInfo};
use crate::parser::parser::{Object, ObjectKind};
use crate::{error::PdfError, parser::parser::Name};

pub const MAX_CODE_SPACE_LENGTH: usize = 4;

// beginrearrangedfont, endrearrangedfont, beginusematrix, endusematrix are
// not used in PDF - PDF 9.7.5.4.e
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
}

pub trait CharCodeToCid {
    fn get_cid(&self, char_code: CharCode) -> Option<Cid>;
}

pub trait CharCodeToGlyphName {
    fn get_glyph_name(&self, char_code: CharCode) -> Option<&[u8]>;
}

#[derive(Debug)]
pub enum GlyphWidth {
    Consecutive(Cid, Vec<f64>),
    Range(Cid, Cid, f64),
}

// This is perhaps the ugliest function I have ever written
pub fn object_array_to_glyph_widths(array: &Vec<Object>) -> Vec<GlyphWidth> {
    let mut glyph_widths = vec![];
    let mut iter = array.iter();
    while let Some(object) = iter.next() {
        let first = match object.kind {
            ObjectKind::Integer(i) => i as Cid,
            ObjectKind::Real(r) => r as Cid,
            _ => break,
        };
        match iter.next() {
            Some(Object {
                kind: ObjectKind::Integer(i),
                ..
            }) => {
                let end = *i as Cid;
                let width = match iter.next() {
                    Some(Object {
                        kind: ObjectKind::Integer(i),
                        ..
                    }) => *i as f64,
                    Some(Object {
                        kind: ObjectKind::Real(r),
                        ..
                    }) => *r as f64,
                    _ => break,
                };
                let glyph_width = GlyphWidth::Range(first, end, width);
                glyph_widths.push(glyph_width);
            }
            Some(Object {
                kind: ObjectKind::Real(r),
                ..
            }) => {
                let end = *r as Cid;
                let width = match iter.next() {
                    Some(Object {
                        kind: ObjectKind::Integer(i),
                        ..
                    }) => *i as f64,
                    Some(Object {
                        kind: ObjectKind::Real(r),
                        ..
                    }) => *r as f64,
                    _ => break,
                };
                let glyph_width = GlyphWidth::Range(first, end, width);
                glyph_widths.push(glyph_width);
            }
            Some(Object {
                kind: ObjectKind::Array(widths),
                ..
            }) => {
                let widths = widths
                    .iter()
                    .filter_map(|obj| obj.try_into().ok())
                    .collect::<Vec<f64>>();
                let glyph_width = GlyphWidth::Consecutive(first, widths);
                glyph_widths.push(glyph_width);
            }
            _ => break,
        };
    }
    glyph_widths
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

#[derive(Debug, Clone)]
pub struct CidChar {
    pub char: CharCode,
    pub cid: Cid,
}

impl CidChar {
    pub const fn new(char_code: CharCode, cid: Cid) -> Self {
        Self {
            char: char_code,
            cid,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CidRange {
    pub start: CharCode,
    pub end: CharCode,
    pub cid: Cid,
}

impl CidRange {
    pub const fn new(start: CharCode, end: CharCode, cid: Cid) -> Self {
        Self { start, end, cid }
    }

    pub fn to_cid(&self, char_code: CharCode) -> Option<Cid> {
        if self.start <= char_code && char_code <= self.end {
            Some(self.cid + (char_code - self.start))
        } else {
            None
        }
    }
}

// "The code length shall not be greater than 4." - PDF 9.7.6.2
// For efficiency, we use this 4 as a universal size instead of supporting a
// hash map implementation. For code lengths less than 4, we just left pad
// with zeros.
pub type CodespaceRange = [RangeInclusive<u8>; MAX_CODE_SPACE_LENGTH];
pub const DEFAULT_CODE_SPACE_RANGE: RangeInclusive<u8> = RangeInclusive::new(0, 0);

#[derive(Debug, PartialEq, Default)]
pub struct Codespace<'a> {
    // We use Cow here because predefined CMaps reference const slices of
    // codespace ranges whereas user-defined CMaps will need to use vectors
    // allocated at runtime.
    pub ranges: Cow<'a, [CodespaceRange]>,
}

impl<'a> Codespace<'a> {
    pub fn new() -> Self {
        Self {
            ranges: vec![].into(),
        }
    }

    pub const fn from(ranges: Cow<'a, [CodespaceRange]>) -> Self {
        Self { ranges }
    }

    // Take a list of bytes and see if the codespace contains these bytes.
    // For the code space to contain a list of bytes, then each byte must
    // be contained within its corresponding range.
    pub fn contains(&self, code: u32) -> bool {
        let bytes = code.to_be_bytes();
        for range in self.ranges.iter() {
            if range
                .iter()
                .zip(bytes)
                .all(|(dim, byte)| dim.contains(&byte))
            {
                return true;
            }
        }
        false
    }
}

// "It is equivalent to the concept of an encoding in simple fonts. Whereas a
// simple font allows a maximum of 256 glyphs to be encoded and accessible at
// one time, a CMap can describe a mapping from multiple-byte codes to
// thousands of glyphs in a large CID-keyed font." (PDF 9.7.2)
#[derive(Debug)]
pub struct CMap<'a> {
    pub name: &'a [u8],
    pub cid_system_info: CidSystemInfo<'a>,
    // "Writing mode is specified as part of the CMap because, in some cases,
    // different shapes are used when writing horizontally and vertically.
    // In such cases, the horizontal and vertical variants of a CMap specify
    // different CIDs for a given character code." (PDF 9.7.5.1)
    pub writing_mode: CMapWritingMode,
    pub codespace: Codespace<'a>,
    pub cid_chars: Cow<'a, [CidChar]>,
    // We use Cow here because for predefined CMaps, we use borrowed slices of
    // const ranges, but for CMap files specified in the PDF file itself we
    // use owned vectors allocated at runtime on the heap
    pub cid_range: Cow<'a, [CidRange]>,
}

impl<'a> CharCodeToCid for CMap<'a> {
    fn get_cid(&self, char_code: CharCode) -> Option<Cid> {
        // TODO: Handle codespace somehow
        // We use rev() because "succeeding maps supercede previous maps"
        // - (CID 5.2 pg 52)
        self.cid_range
            .iter()
            .rev()
            .find_map(|range| range.to_cid(char_code))
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
        let codespace = Codespace::from(ranges.into());
        assert!(codespace.contains(0x79));
        assert!(codespace.contains(0x86A9));
        assert!(!codespace.contains(0x8010));
        assert!(!codespace.contains(0x8210));
    }

    #[test]
    fn test_cmap_cid_range() {
        let cid_range = CidRange::new(0x20, 0x7e, 1);
        assert_eq!(cid_range.to_cid(0x20).unwrap(), 1);
        assert_eq!(cid_range.to_cid(0x21).unwrap(), 2);
        assert_eq!(cid_range.to_cid(0x22).unwrap(), 3);
        // SPEC_BREAK? CID says this should be 94, not sure
        assert_eq!(cid_range.to_cid(0x7e).unwrap(), 95);
    }
}
