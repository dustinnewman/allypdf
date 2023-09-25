use std::borrow::Cow;
use std::{collections::BTreeMap, convert::TryFrom};

use ttf_parser::Face;

use super::encoding::{Encoding, ENCODING_SIZE};
use super::font_descriptor::FontDescriptor;
use crate::cmaps::cid::{CharCode, CharCodeToCid, CharCodeToGlyphName, CharCodeToUnicode, Cid};
use crate::cmaps::cmap::CMap;
use crate::document::document::{ObjectMap, ReferenceResolver};
use crate::document::page::Resources;
use crate::error::{PdfError, Result};
use crate::font::glyph_width::GlyphWidth;
use crate::operators::{matrix::Matrix, rect::Rectangle};
use crate::parser::object::{Dictionary, Name, Object, ObjectKind, Stream};

const IDENTITY: &[u8] = b"Identity";
const IDENTITY_H: &[u8] = b"Identity-H";
const IDENTITY_V: &[u8] = b"Identity-V";

pub struct Type1FontProgram<'a> {
    clear_portion: &'a [u8],
    program_portion: &'a [u8],
}

// PDF 9.9.1 Table 124 - Not technically accurate, Type1C, CID Font, and OpenType
// are subtypes rather than types themselves.
#[derive(Debug)]
pub enum FontProgramKind<'a> {
    Type1(&'a Stream),
    TrueType(Face<'a>),
    Type1C,
    CIDFontType0C,
    OpenType(&'a Stream),
}

// PDF 9.5 Table 108
#[derive(Debug)]
pub enum Type1SubtypeKind {
    Type1,
    MMType1,
}

// PDF 9.6.2.1 Table 109
#[derive(Debug)]
pub struct Type1Font<'a> {
    subtype: Type1SubtypeKind,
    name: Option<&'a Name>,
    base_font: &'a Name,
    first_char: Option<CharCode>,
    last_char: Option<CharCode>,
    widths: Option<[f64; ENCODING_SIZE]>,
    font_descriptor: Option<FontDescriptor<'a>>,
    encoding: Option<Encoding<'a>>,
    to_unicode: Option<&'a Stream>,
}

impl<'a> Type1Font<'a> {
    pub fn new(
        subtype: Type1SubtypeKind,
        name: Option<&'a Name>,
        base_font: &'a Name,
        first_char: Option<CharCode>,
        last_char: Option<CharCode>,
        widths: Option<[f64; ENCODING_SIZE]>,
        font_descriptor: Option<FontDescriptor<'a>>,
        encoding: Option<Encoding<'a>>,
        to_unicode: Option<&'a Stream>,
    ) -> Self {
        Self {
            subtype,
            name,
            base_font,
            first_char,
            last_char,
            widths,
            font_descriptor,
            encoding,
            to_unicode,
        }
    }
}

// PDF 9.6.3
#[derive(Debug)]
pub struct TrueTypeFont<'a> {
    name: Option<&'a Name>,
    base_font: &'a Name,
    first_char: Option<CharCode>,
    last_char: Option<CharCode>,
    widths: Option<[f64; ENCODING_SIZE]>,
    font_descriptor: Option<FontDescriptor<'a>>,
    encoding: Option<Encoding<'a>>,
    to_unicode: Option<CMap<'a>>,
}

impl<'a> TrueTypeFont<'a> {
    pub fn new(
        name: Option<&'a Name>,
        base_font: &'a Name,
        first_char: Option<CharCode>,
        last_char: Option<CharCode>,
        widths: Option<[f64; ENCODING_SIZE]>,
        font_descriptor: Option<FontDescriptor<'a>>,
        encoding: Option<Encoding<'a>>,
        to_unicode: Option<CMap<'a>>,
    ) -> Self {
        Self {
            name,
            base_font,
            first_char,
            last_char,
            widths,
            font_descriptor,
            encoding,
            to_unicode,
        }
    }
}

// PDF 9.6.4
#[derive(Debug)]
pub struct Type3Font<'a> {
    name: Option<&'a Name>,
    font_b_box: Rectangle,
    font_matrix: Matrix,
    char_procs: &'a Dictionary,
    encoding: Encoding<'a>,
    first_char: CharCode,
    last_char: CharCode,
    widths: [f64; ENCODING_SIZE],
    font_descriptor: FontDescriptor<'a>,
    resources: Option<Box<Resources<'a>>>,
    to_unicode: Option<&'a Stream>,
}

impl<'a> Type3Font<'a> {
    pub fn new(
        name: Option<&'a Name>,
        font_b_box: Rectangle,
        font_matrix: Matrix,
        char_procs: &'a Dictionary,
        encoding: Encoding<'a>,
        first_char: CharCode,
        last_char: CharCode,
        widths: [f64; ENCODING_SIZE],
        font_descriptor: FontDescriptor<'a>,
        resources: Option<Resources<'a>>,
        to_unicode: Option<&'a Stream>,
    ) -> Self {
        Self {
            name,
            font_b_box,
            font_matrix,
            char_procs,
            encoding,
            first_char,
            last_char,
            widths,
            font_descriptor,
            resources: resources.map(Box::new),
            to_unicode,
        }
    }
}

// PDF 9.5 Table 108
#[derive(Debug)]
pub enum CIDFontSubtypeKind {
    CIDFontType0,
    CIDFontType2,
}

impl TryFrom<&[u8]> for CIDFontSubtypeKind {
    type Error = PdfError;

    fn try_from(string: &[u8]) -> Result<Self> {
        match string {
            b"CIDFontType0" => Ok(Self::CIDFontType0),
            b"CIDFontType2" => Ok(Self::CIDFontType2),
            _ => Err(PdfError::InvalidCIDFontSubtypeKind),
        }
    }
}

// PDF 9.7.3 Table 114
#[derive(Debug, Default)]
pub struct CidSystemInfo<'a> {
    pub registry: Cow<'a, [u8]>,
    pub ordering: Cow<'a, [u8]>,
    pub supplement: u32,
}

// Map character identifiers to glyph identifiers.
#[derive(Debug)]
pub enum CIDToGIDMap<'a> {
    // Basically there is no mapping, just use the CID as the GID but we
    // needed a way to represent this formally
    Identity,
    // There is an actual mapping inside the stream, use it
    Stream(&'a Stream),
}

impl<'a> TryFrom<&'a Object> for CIDToGIDMap<'a> {
    type Error = PdfError;

    fn try_from(object: &'a Object) -> Result<Self> {
        match &object.kind {
            ObjectKind::Name(name) if name == &IDENTITY => Ok(CIDToGIDMap::Identity),
            ObjectKind::Stream(stream) => Ok(CIDToGIDMap::Stream(stream)),
            _ => Err(PdfError::InvalidCIDToGIDMap),
        }
    }
}

// PDF 9.7.4.1 Table 115
#[derive(Debug)]
pub struct CIDFont<'a> {
    pub subtype: CIDFontSubtypeKind,
    pub base_font: &'a Name,
    pub cid_system_info: CidSystemInfo<'a>,
    pub font_descriptor: FontDescriptor<'a>,
    pub default_width: f64,
    pub widths: Vec<GlyphWidth>,
    pub vertical_default_width: (f64, f64),
    pub vertical_widths: Option<Vec<GlyphWidth>>,
    pub cid_to_gid_map: CIDToGIDMap<'a>,
}

#[derive(Debug)]
pub enum Type0Encoding<'a> {
    IdentityH,
    IdentityV,
    CMap(CMap<'a>),
}

impl<'a> TryFrom<&'a Name> for Type0Encoding<'a> {
    type Error = PdfError;

    fn try_from(name: &'a Name) -> Result<Self> {
        match name.0.as_ref() {
            IDENTITY_H => Ok(Self::IdentityH),
            IDENTITY_V => Ok(Self::IdentityV),
            name => Ok(Self::CMap(CMap::try_from(name)?)),
        }
    }
}

impl<'a> TryFrom<&'a Stream> for Type0Encoding<'a> {
    type Error = PdfError;

    fn try_from(stream: &'a Stream) -> Result<Self> {
        let cmap: CMap<'a> = CMap::try_from(stream)?;
        Ok(Self::CMap(cmap))
    }
}

impl<'a> TryFrom<(Option<&'a Object>, &'a ObjectMap)> for Type0Encoding<'a> {
    type Error = PdfError;

    fn try_from((object, object_map): (Option<&'a Object>, &'a ObjectMap)) -> Result<Self> {
        let Some(object) = object else {
            return Err(PdfError::CMapParsingError)
        };
        match &object.kind {
            ObjectKind::Stream(stream) => Self::try_from(stream),
            ObjectKind::Name(name) => Self::try_from(name),
            ObjectKind::IndirectReference(ind_ref) => {
                Self::try_from((object_map.get(ind_ref), object_map))
            }
            _ => Err(PdfError::CMapParsingError),
        }
    }
}

impl CharCodeToCid for Type0Encoding<'_> {
    fn get_cid(&self, char_code: CharCode) -> Option<Cid> {
        match self {
            Type0Encoding::IdentityH | Type0Encoding::IdentityV => Some(char_code),
            Type0Encoding::CMap(cmap) => cmap.get_cid(char_code),
        }
    }
}

// PDF 9.6.2.1 Table 109
#[derive(Debug)]
pub struct Type0Font<'a> {
    base_font: &'a Name,
    encoding: Type0Encoding<'a>,
    descendant_fonts: CIDFont<'a>,
    to_unicode: Option<CMap<'a>>,
}

impl<'a> Type0Font<'a> {
    pub fn new(
        base_font: &'a Name,
        encoding: Type0Encoding<'a>,
        descendant_fonts: CIDFont<'a>,
        to_unicode: Option<CMap<'a>>,
    ) -> Self {
        Self {
            base_font,
            encoding,
            descendant_fonts,
            to_unicode,
        }
    }
}

// PDF 9.5 Table 108
#[derive(Debug)]
pub enum Font<'a> {
    // Type0 fonts are also called "composite fonts." All others are "simple"
    Type0(Type0Font<'a>),
    Type1(Type1Font<'a>),
    Type3(Type3Font<'a>),
    TrueType(TrueTypeFont<'a>),
    // CID fonts cannot be used directly but only as children of Type0 fonts
}

impl CharCodeToGlyphName for TrueTypeFont<'_> {
    fn get_glyph_name(&self, char_code: CharCode) -> Option<&[u8]> {
        if let Some(encoding) = &self.encoding {
            encoding.get_glyph_name(char_code)
        } else {
            // PDF 9.6.5.4 Encodings for TrueType fonts
            // "When the font has no Encoding entry, or the font descriptor's
            // Symbolic flag is set (in which case the Encoding entry is
            // ignored), this shall occur:
            // If the font contains a (1, 0) subtable, single bytes
            // from the string shall be used to look up the associated glyph
            // descriptions from the subtable
            todo!()
        }
    }
}

impl CharCodeToUnicode for TrueTypeFont<'_> {
    fn get_unicode(&self, char_code: CharCode) -> Option<String> {
        if let Some(to_unicode_cmap) = &self.to_unicode {
            Some(String::from("A"))
        } else {
            None
        }
    }
}

pub type FontDictionary<'a> = BTreeMap<Cow<'a, Name>, Font<'a>>;

#[cfg(test)]
mod test {
    use std::fs;
    use std::path::Path;

    use ttf_parser::fonts_in_collection;

    #[test]
    fn test_true_type_font_parser() {
        let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("true_type_font.ttf");
        let font = fs::read(file_path).unwrap();
        let index = fonts_in_collection(&font).unwrap_or(0);
        let font = ttf_parser::Face::parse(&font, index).unwrap();
        let post = font.tables().post.unwrap();
        for name in post.names() {
            println!("{:?}", name);
        }
    }
}
