use std::{collections::BTreeMap, convert::TryFrom};

use super::cmap::{CMap, GlyphWidth, CharCodeToCid};
use super::encoding::{Encoding, ENCODING_SIZE};
use crate::document::page::Resources;
use crate::error::PdfError;
use crate::operators::{matrix::Matrix, rect::Rectangle};
use crate::parser::parser::{Dictionary, Name, Object, ObjectKind, Stream};

const IDENTITY: &[u8] = b"Identity";
const IDENTITY_H: &[u8] = b"Identity-H";
const IDENTITY_V: &[u8] = b"Identity-V";
// In simple fonts, character codes are only 8-bit and can thus only address
// 256 glyphs. In composite fonts, however, we can have multi-byte character
// codes from 2 to 4 bytes.
pub type CharCode = u32;
// "In previous versions of PDF, it was recommended that the maximum value of
// a CID (character identifier) was limited to 65,535." (PDF Annex C.2)
// However, we will use u32 to be safe. If a PDF works for CID 65,535, why
// shouldn't it work for 65,536?
pub type Cid = u32;

#[derive(Debug)]
pub struct FontDescriptorFlags(u32);

impl FontDescriptorFlags {
    const FLAG_FIXED_PITCH: u32 = 0;
    const FLAG_SERIF: u32 = 1;
    const FLAG_SYMBOLIC: u32 = 2;
    const FLAG_SCRIPT: u32 = 3;
    const FLAG_NON_SYMBOLIC: u32 = 5;
    const FLAG_ITALIC: u32 = 6;
    const FLAG_ALL_CAP: u32 = 16;
    const FLAG_SMALL_CAP: u32 = 17;
    const FLAG_FORCE_BOLD: u32 = 18;

    pub fn new(value: u32) -> Self {
        Self(value)
    }

    fn n(&self, n: u32) -> bool {
        (self.0 >> n) & 1 == 1
    }

    pub fn fixed_pitch(&self) -> bool {
        self.n(Self::FLAG_FIXED_PITCH)
    }

    pub fn serif(&self) -> bool {
        self.n(Self::FLAG_FIXED_PITCH)
    }

    pub fn symbolic(&self) -> bool {
        self.n(Self::FLAG_SYMBOLIC)
    }

    pub fn script(&self) -> bool {
        self.n(Self::FLAG_SCRIPT)
    }

    pub fn non_symbolic(&self) -> bool {
        self.n(Self::FLAG_NON_SYMBOLIC)
    }

    pub fn italic(&self) -> bool {
        self.n(Self::FLAG_ITALIC)
    }

    pub fn all_cap(&self) -> bool {
        self.n(Self::FLAG_ALL_CAP)
    }

    pub fn small_cap(&self) -> bool {
        self.n(Self::FLAG_SMALL_CAP)
    }

    pub fn force_bold(&self) -> bool {
        self.n(Self::FLAG_FORCE_BOLD)
    }
}

#[derive(Debug)]
pub enum FontStretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl TryFrom<&Name> for FontStretch {
    type Error = PdfError;

    fn try_from(name: &Name) -> Result<Self, Self::Error> {
        match name {
            x if x == b"UltraCondensed" => Ok(Self::UltraCondensed),
            x if x == b"ExtraCondensed" => Ok(Self::ExtraCondensed),
            x if x == b"Condensed" => Ok(Self::Condensed),
            x if x == b"SemiCondensed" => Ok(Self::SemiCondensed),
            x if x == b"Normal" => Ok(Self::Normal),
            x if x == b"SemiExpanded" => Ok(Self::SemiExpanded),
            x if x == b"Expanded" => Ok(Self::Expanded),
            x if x == b"ExtraExpanded" => Ok(Self::ExtraExpanded),
            x if x == b"UltraExpanded" => Ok(Self::UltraExpanded),
            _ => Err(PdfError::InvalidFontStretch),
        }
    }
}

#[derive(Debug)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

impl TryFrom<&Name> for FontWeight {
    type Error = PdfError;

    fn try_from(name: &Name) -> Result<Self, Self::Error> {
        match name {
            x if x == b"Thin" => Ok(Self::Thin),
            x if x == b"ExtraLight" => Ok(Self::ExtraLight),
            x if x == b"Light" => Ok(Self::Light),
            x if x == b"Normal" => Ok(Self::Normal),
            x if x == b"Medium" => Ok(Self::Medium),
            x if x == b"SemiBold" => Ok(Self::SemiBold),
            x if x == b"Bold" => Ok(Self::Bold),
            x if x == b"ExtraBold" => Ok(Self::ExtraBold),
            x if x == b"Black" => Ok(Self::Black),
            _ => Err(PdfError::InvalidFontWeight),
        }
    }
}

pub struct Type1FontProgram<'a> {
    clear_portion: &'a [u8],
    program_portion: &'a [u8],
}

// PDF 9.9.1 Table 124 - Not technically accurate, Type1C, CID Font, and OpenType
// are subtypes rather than types themselves.
#[derive(Debug)]
pub enum FontProgramKind {
    Type1,
    TrueType,
    Type1C,
    CIDFontType0C,
    OpenType,
}

// PDF 9.8.1 Table 120
#[derive(Debug)]
pub struct FontDescriptor<'a> {
    font_name: &'a Name,
    font_family: Option<&'a Name>,
    font_stretch: Option<FontStretch>,
    font_weight: Option<FontWeight>,
    flags: FontDescriptorFlags,
    font_b_box: Option<Rectangle>,
    italic_angle: f64,
    ascent: Option<f64>,
    descent: Option<f64>,
    leading: Option<f64>,
    // Required if the font contains Latin characters (PDF 9.8.1 Table 108)
    // Since this is impossible to determine with the type system, let's just
    // say it is optional
    cap_height: Option<f64>,
    x_height: Option<f64>,
    stem_v: Option<f64>,
    stem_h: Option<f64>,
    avg_width: Option<f64>,
    max_width: Option<f64>,
    pub missing_width: f64,
    font_file: Option<(FontProgramKind, &'a Stream)>,
    char_set: Option<&'a Name>,
    // CID only
    style: Option<&'a Dictionary>,
    lang: Option<Name>,
    fd: Option<&'a Dictionary>,
    cid_set: Option<&'a Dictionary>,
}

impl<'a> FontDescriptor<'a> {
    pub fn new(
        font_name: &'a Name,
        font_family: Option<&'a Name>,
        font_stretch: Option<FontStretch>,
        font_weight: Option<FontWeight>,
        flags: FontDescriptorFlags,
        font_b_box: Option<Rectangle>,
        italic_angle: f64,
        ascent: Option<f64>,
        descent: Option<f64>,
        leading: Option<f64>,
        cap_height: Option<f64>,
        x_height: Option<f64>,
        stem_v: Option<f64>,
        stem_h: Option<f64>,
        avg_width: Option<f64>,
        max_width: Option<f64>,
        missing_width: Option<f64>,
        font_file: Option<(FontProgramKind, &'a Stream)>,
        char_set: Option<&'a Name>,
        style: Option<&'a Dictionary>,
        lang: Option<Name>,
        fd: Option<&'a Dictionary>,
        cid_set: Option<&'a Dictionary>,
    ) -> Self {
        Self {
            font_name,
            font_family,
            font_stretch,
            font_weight,
            flags,
            font_b_box,
            italic_angle,
            ascent,
            descent,
            leading,
            cap_height,
            x_height,
            stem_v,
            stem_h,
            avg_width,
            max_width,
            missing_width: missing_width.unwrap_or(0.),
            font_file,
            char_set,
            style,
            lang,
            fd,
            cid_set,
        }
    }
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
    to_unicode: Option<&'a Stream>,
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
        to_unicode: Option<&'a Stream>,
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

    fn try_from(string: &[u8]) -> Result<Self, Self::Error> {
        match string {
            b"CIDFontType0" => Ok(Self::CIDFontType0),
            b"CIDFontType2" => Ok(Self::CIDFontType2),
            _ => Err(PdfError::InvalidCIDFontSubtypeKind),
        }
    }
}

// PDF 9.7.3 Table 114
#[derive(Debug)]
pub struct CidSystemInfo<'a> {
    pub registry: &'a [u8],
    pub ordering: &'a [u8],
    pub supplement: u32,
}

#[derive(Debug)]
pub enum CIDToGIDMap<'a> {
    Identity,
    Stream(&'a Stream),
}

impl<'a> TryFrom<&'a Object> for CIDToGIDMap<'a> {
    type Error = PdfError;

    fn try_from(object: &'a Object) -> Result<Self, Self::Error> {
        match &object.kind {
            ObjectKind::Name(name) if name == IDENTITY => Ok(CIDToGIDMap::Identity),
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

impl<'a> TryFrom<&[u8]> for Type0Encoding<'a> {
    type Error = PdfError;

    fn try_from(name: &[u8]) -> Result<Self, Self::Error> {
        match name {
            IDENTITY_H => Ok(Self::IdentityH),
            IDENTITY_V => Ok(Self::IdentityV),
            _ => Err(PdfError::InvalidType0EncodingName),
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
    to_unicode: Option<&'a Stream>,
}

impl<'a> Type0Font<'a> {
    pub fn new(
        base_font: &'a Name,
        encoding: Type0Encoding<'a>,
        descendant_fonts: CIDFont<'a>,
        to_unicode: Option<&'a Stream>,
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

pub type FontDictionary<'a> = BTreeMap<&'a Name, Font<'a>>;
