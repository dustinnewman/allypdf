use std::convert::{TryFrom, TryInto};

use crate::document::{ObjectMap, ReferenceResolver};
use crate::error::PdfError;
use crate::operators::rect::Rectangle;
use crate::parser::object::{Dictionary, Name, Object, Stream};

use super::FontProgramKind;

const FONT_NAME: &[u8] = b"FontName";
const FONT_FAMILY: &[u8] = b"FontFamily";
const FONT_STRETCH: &[u8] = b"FontStretch";
const FONT_WEIGHT: &[u8] = b"FontWeight";
const FLAGS: &[u8] = b"Flags";
const FONT_B_BOX: &[u8] = b"FontBBox";
const ITALIC_ANGLE: &[u8] = b"ItalicAngle";
const ASCENT: &[u8] = b"Ascent";
const DESCENT: &[u8] = b"Descent";
const LEADING: &[u8] = b"Leading";
const CAP_HEIGHT: &[u8] = b"CapHeight";
const X_HEIGHT: &[u8] = b"XHeight";
const STEM_V: &[u8] = b"StemV";
const STEM_H: &[u8] = b"StemH";
const AVG_WIDTH: &[u8] = b"AvgWidth";
const MAX_WIDTH: &[u8] = b"MaxWidth";
const MISSING_WIDTH: &[u8] = b"MissingWidth";
const FONT_FILE: &[u8] = b"FontFile";
const FONT_FILE_2: &[u8] = b"FontFile2";
const FONT_FILE_3: &[u8] = b"FontFile3";
const CHAR_SET: &[u8] = b"CharSet";
const LENGTH_1: &[u8] = b"Length1";
const LENGTH_2: &[u8] = b"Length2";
const LENGTH_3: &[u8] = b"Length3";

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
    font_file: Option<FontProgramKind<'a>>,
    char_set: Option<&'a Name>,
    // CID only
    style: Option<&'a Dictionary>,
    lang: Option<Name>,
    fd: Option<&'a Dictionary>,
    cid_set: Option<&'a Dictionary>,
}

impl<'a> TryFrom<(Option<&'a Object>, &'a ObjectMap)> for FontDescriptor<'a>
where
    Object: TryInto<&'a Dictionary>,
{
    type Error = PdfError;

    fn try_from(
        (object, object_map): (Option<&'a Object>, &'a ObjectMap),
    ) -> Result<Self, Self::Error> {
        object
            .ok_or(Self::Error::NoFontDescriptor)
            .and_then(|object| Self::try_from((object, object_map)))
    }
}

impl<'a> TryFrom<(&'a Object, &'a ObjectMap)> for FontDescriptor<'a>
where
    Object: TryInto<&'a Dictionary>,
{
    type Error = PdfError;

    fn try_from((object, object_map): (&'a Object, &'a ObjectMap)) -> Result<Self, Self::Error> {
        let dict: &'a Dictionary = object_map
            .follow_till(Some(object))
            .ok_or(Self::Error::ObjectNotIndirectReference)?;
        Self::try_from((dict, object_map))
    }
}

impl<'a> TryFrom<(&'a Dictionary, &'a ObjectMap)> for FontDescriptor<'a> {
    type Error = PdfError;

    fn try_from((dict, object_map): (&'a Dictionary, &'a ObjectMap)) -> Result<Self, Self::Error> {
        let font_name = dict
            .get(FONT_NAME)
            .ok_or(Self::Error::FontDescriptorMissingFontName)?
            .try_into()?;
        let font_family = dict
            .get(FONT_FAMILY)
            .and_then(|object| object.try_into().ok());
        let font_stretch = dict
            .get(FONT_STRETCH)
            .and_then(|object| object.try_into().ok())
            .and_then(|name: &Name| FontStretch::try_from(name).ok());
        let font_weight = dict
            .get(FONT_WEIGHT)
            .and_then(|object| object.try_into().ok())
            .and_then(|name: &Name| FontWeight::try_from(name).ok());
        let flags = dict
            .get(FLAGS)
            .ok_or(Self::Error::FontDescriptorMissingFlags)?
            .try_into()?;
        let flags = FontDescriptorFlags::new(flags);
        let font_b_box = dict
            .get(FONT_B_BOX)
            .and_then(|object| Rectangle::try_from(object).ok());
        let italic_angle = dict
            .get(ITALIC_ANGLE)
            .ok_or(Self::Error::FontDescriptorMissingItalicAngle)?;
        let italic_angle = f64::try_from(italic_angle)?;
        let ascent = dict.get(ASCENT).and_then(|obj| f64::try_from(obj).ok());
        let descent = dict.get(DESCENT).and_then(|obj| f64::try_from(obj).ok());
        let leading = dict.get(LEADING).and_then(|obj| f64::try_from(obj).ok());
        let cap_height = dict.get(CAP_HEIGHT).and_then(|obj| f64::try_from(obj).ok());
        let x_height = dict.get(X_HEIGHT).and_then(|obj| f64::try_from(obj).ok());
        let stem_v = dict.get(STEM_V).and_then(|obj| f64::try_from(obj).ok());
        let stem_h = dict.get(STEM_H).and_then(|obj| f64::try_from(obj).ok());
        let avg_width = dict.get(AVG_WIDTH).and_then(|obj| f64::try_from(obj).ok());
        let max_width = dict.get(MAX_WIDTH).and_then(|obj| f64::try_from(obj).ok());
        let missing_width = dict
            .get(MISSING_WIDTH)
            .and_then(|obj| f64::try_from(obj).ok());
        let font_file = if let Some(stream) = object_map.follow_till(dict.get(FONT_FILE)) {
            // PDF 9.9.1 Table 125
            let clear_text_length: Option<i64> = object_map.follow_till(dict.get(LENGTH_1));
            let cypher_text_length: Option<i64> = object_map.follow_till(dict.get(LENGTH_2));
            let fixed_content_length: Option<i64> = object_map.follow_till(dict.get(LENGTH_3));
            Some(FontProgramKind::Type1(stream))
        } else if let Some(stream) = <ObjectMap as ReferenceResolver<&Object, &Stream>>::follow_till(
            object_map,
            dict.get(FONT_FILE_2),
        ) {
            // PDF 9.9.1 Table 125
            let font_program_length: Option<i64> = object_map.follow_till(dict.get(LENGTH_1));
            let font_program_length = if let Some(x) = font_program_length {
                x as usize
            } else {
                stream.content.len()
            };
            let content = &stream.content[..font_program_length];
            ttf_parser::Face::parse(content, 0)
                .ok()
                .map(Box::new)
                .map(FontProgramKind::TrueType)
        } else {
            object_map
                .follow_till(dict.get(FONT_FILE_3))
                .map(FontProgramKind::OpenType)
        };
        let char_set = dict.get(CHAR_SET).and_then(|obj| obj.try_into().ok());
        // TODO: Parse char_set list of names into Vec<Name> using helper functions
        Ok(FontDescriptor {
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
            style: None,
            lang: None,
            fd: None,
            cid_set: None,
        })
    }
}
