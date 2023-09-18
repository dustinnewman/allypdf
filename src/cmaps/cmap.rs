use std::borrow::Cow;
use std::convert::TryFrom;
use std::ops::RangeInclusive;

use crate::error::PdfError;
use crate::font::font::CidSystemInfo;
use crate::parser::cid_parser::CMapFileParser;
use crate::parser::parser::{Name, Stream};

use super::cid::{CharCode, CharCodeToCid, Cid};
use super::{cns_1, gb_1, japan_1, korea_1};

pub const MAX_CODE_SPACE_LENGTH: usize = 4;
pub const ADOBE_REGISTRY: &[u8] = b"Adobe";
pub const NO_CID_CHARS: Cow<[CidChar]> = Cow::Borrowed(&[]);
pub const NO_CID_RANGE: Cow<[CidRange]> = Cow::Borrowed(&[]);
pub const NO_BASE_FONT_CHARS: Cow<[BaseFontChar]> = Cow::Borrowed(&[]);

#[derive(Debug, Clone)]
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
#[repr(C)]
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
pub enum BaseFontCharDestination {
    Cid(Cid),
    CharName(Name),
}

#[derive(Debug, Clone)]
pub struct BaseFontChar {
    pub char: CharCode,
    pub dest: BaseFontCharDestination,
}

impl BaseFontChar {
    pub const fn new(char: CharCode, dest: BaseFontCharDestination) -> Self {
        Self { char, dest }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
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

#[derive(Debug, Clone)]
pub enum BaseFontRangeDestination {
    Cid(Cid),
    CharNames(Vec<Name>),
}

#[derive(Debug, Clone)]
pub struct BaseFontRange {
    pub start: CharCode,
    pub end: CharCode,
    pub dest: BaseFontRangeDestination,
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
#[derive(Debug, Default)]
pub struct CMap<'a> {
    pub name: Cow<'a, [u8]>,
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
    pub base_font_chars: Cow<'a, [BaseFontChar]>,
    // pub base_font_range: Cow<>,
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

impl TryFrom<&[u8]> for CMap<'static> {
    type Error = PdfError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let cmap = match value {
            b"GB-EUC-H" => gb_1::gb_euc::GB_EUC_H,
            b"GB-EUC-V" => gb_1::gb_euc::GB_EUC_V,
            b"GBpc-EUC-H" => gb_1::gbpc_euc::GB_1_GBPC_EUC_H,
            b"GBpc-EUC-V" => gb_1::gbpc_euc::GB_1_GBPC_EUC_V,
            b"GBK-EUC-H" => gb_1::gbk_euc::GBK_EUC_H,
            b"GBK-EUC-V" => gb_1::gbk_euc::GBK_EUC_V,
            b"GBKp-EUC-H" => gb_1::gbkp_euc::GBKP_EUC_H,
            b"GBKp-EUC-V" => gb_1::gbkp_euc::GBKP_EUC_V,
            b"GBK2K-H" => gb_1::gbk2k::GBK2K_H,
            b"GBK2K-V" => gb_1::gbk2k::GBK2K_V,
            b"UniGB-UCS2-H" => gb_1::unigb_ucs2::UNIGB_UCS2_H,
            b"UniGB-UCS2-V" => gb_1::unigb_ucs2::UNIGB_UCS2_V,
            b"UniGB-UTF16-H" => gb_1::unigb_utf16::UNIGB_UTF16_H,
            b"UniGB-UTF16-V" => gb_1::unigb_utf16::UNIGB_UTF16_V,
            b"B5pc-H" => cns_1::b5pc::B5PC_H,
            b"B5pc-V" => cns_1::b5pc::B5PC_V,
            b"HKscs-B5-H" => cns_1::hkscs_b5::HKSCS_B5_H,
            b"HKscs-B5-V" => cns_1::hkscs_b5::HKSCS_B5_V,
            b"ETen-B5-H" => cns_1::eten_b5::ETEN_B5_H,
            b"ETen-B5-V" => cns_1::eten_b5::ETEN_B5_V,
            b"ETenms-B5-H" => cns_1::etenms_b5::ETENMS_B5_H,
            b"ETenms-B5-V" => cns_1::etenms_b5::ETENMS_B5_V,
            b"CNS-EUC-H" => cns_1::cns_euc::CNS_EUC_H,
            b"CNS-EUC-V" => cns_1::cns_euc::CNS_EUC_V,
            b"UniCNS-UCS2-H" => cns_1::unicns_ucs2::UNICNS_UCS2_H,
            b"UniCNS-UCS2-V" => cns_1::unicns_ucs2::UNICNS_UCS2_V,
            b"UniCNS-UTF16-H" => cns_1::unicns_utf16::UNICNS_UTF16_H,
            b"UniCNS-UTF16-V" => cns_1::unicns_utf16::UNICNS_UTF16_V,
            b"83pv-RKSJ-H" => japan_1::jp_83pv_rksj::JAPAN_1_83PV_RKSJ_H,
            b"90ms-RKSJ-H" => japan_1::jp_90ms_rksj::JAPAN_1_90MS_RKSJ_H,
            b"90ms-RKSJ-V" => japan_1::jp_90ms_rksj::JAPAN_1_90MS_RKSJ_V,
            b"90msp-RKSJ-H" => japan_1::jp_90msp_rksj::JAPAN_1_90MSP_RKSJ_H,
            b"90msp-RKSJ-V" => japan_1::jp_90msp_rksj::JAPAN_1_90MSP_RKSJ_V,
            b"90pv-RKSJ-H" => japan_1::jp_90pv_rksj::JAPAN_1_90PV_RKSJ_H,
            b"Add-RKSJ-H" => japan_1::add_rksj::ADD_RKSJ_H,
            b"Add-RKSJ-V" => japan_1::add_rksj::ADD_RKSJ_V,
            b"EUC-H" => japan_1::euc::EUC_H,
            b"EUC-V" => japan_1::euc::EUC_V,
            b"Ext-RKSJ-H" => japan_1::ext_rksj::EXT_RKSJ_H,
            b"Ext-RKSJ-V" => japan_1::ext_rksj::EXT_RKSJ_V,
            b"H" => japan_1::h_v::H,
            b"V" => japan_1::h_v::V,
            b"UniJIS-UCS2-H" => japan_1::unijis_ucs2::JAPAN_1_UNIJIS_UCS2_H,
            b"UniJIS-UCS2-V" => japan_1::unijis_ucs2::JAPAN_1_UNIJIS_UCS2_V,
            b"UniJIS-UCS2-HW-H" => japan_1::unijis_ucs2_hw::UNIJIS_UCS2_HW_H,
            b"UniJIS-UCS2-HW-V" => japan_1::unijis_ucs2_hw::UNIJIS_UCS2_HW_V,
            b"UniJIS-UTF16-H" => japan_1::unijis_utf16::UNIJIS_UTF16_H,
            b"UniJIS-UTF16-V" => japan_1::unijis_utf16::UNIJIS_UTF16_V,
            b"KSC-EUC-H" => korea_1::ksc_euc::KSC_EUC_H,
            b"KSC-EUC-V" => korea_1::ksc_euc::KSC_EUC_V,
            b"KSCms-UHC-H" => korea_1::kscms_uhc::KSCMS_UHC_H,
            b"KSCms-UHC-V" => korea_1::kscms_uhc::KSCMS_UHC_V,
            b"KSCms-UHC-HW-H" => korea_1::kscms_uhc_hw::KSCMS_UHC_HW_H,
            b"KSCms-UHC-HW-V" => korea_1::kscms_uhc_hw::KSCMS_UHC_HW_V,
            b"KSCpc-EUC-H" => korea_1::kscpc_euc::KSCPC_EUC_H,
            b"UniKS-UCS2-H" => korea_1::uniks_ucs2::UNIKS_UCS2_H,
            b"UniKS-UCS2-V" => korea_1::uniks_ucs2::UNIKS_UCS2_V,
            b"UniKS-UTF16-H" => korea_1::uniks_utf16::UNIKS_UTF16_H,
            b"UniKS-UTF16-V" => korea_1::uniks_utf16::UNIKS_UTF16_V,
            _ => return Err(PdfError::InvalidType0EncodingName),
        };
        Ok(cmap)
    }
}

impl<'a> TryFrom<&'a Vec<u8>> for CMap<'a> {
    type Error = PdfError;

    fn try_from(text: &'a Vec<u8>) -> Result<Self, Self::Error> {
        let mut lexer = crate::parser::lexer::Lexer::new(text);
        let tokens = lexer.lex();
        let mut parser = crate::parser::parser::Parser::new(&tokens);
        let objects = parser.parse();
        let cmap_parser = CMapFileParser::new(objects);
        let cmap = cmap_parser.parse();
        cmap.ok_or(PdfError::CMapParsingError)
    }
}

impl<'a, 'b: 'a> TryFrom<&'b Stream> for CMap<'a> {
    type Error = PdfError;

    fn try_from(stream: &'b Stream) -> Result<Self, Self::Error> {
        Self::try_from(&stream.content)
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

    #[test]
    fn test_cmap_bf_char() {
        let text = b"/CIDInit /ProcSet findresource begin
        12 dict begin
        begincmap
        /CIDSystemInfo <<
          /Registry (Adobe)
          /Ordering (UCS)
          /Supplement 0
        >> def
        /CMapName /Adobe-Identity-UCS def
        /CMapType 2 def
        1 begincodespacerange
        <00><FF>
        endcodespacerange
        1 beginbfchar
        <22><0644 0627>
        endbfchar
        2 beginbfrange
        <21><21><0645>
        <23><23><0633>
        endbfrange
        endcmap
        CMapName currentdict /CMap defineresource pop
        end
        end"
        .to_vec();
        let cmap = CMap::try_from(&text);
        assert!(cmap.is_ok());
        let cmap = cmap.unwrap();
        println!("{:?}", cmap);
        assert!(false);
    }
}
