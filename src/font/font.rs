use std::borrow::Cow;
use std::{collections::BTreeMap, convert::TryFrom};

use ttf_parser::Face;

use super::true_type_font::TrueTypeFont;
use super::type_0_font::Type0Font;
use super::type_1_font::Type1Font;
use super::type_3_font::Type3Font;
use crate::document::document::{ObjectMap, ReferenceResolver};
use crate::error::{PdfError, Result};
use crate::parser::object::{Dictionary, Name, Stream};

const IDENTITY: &[u8] = b"Identity";
const SUB_TYPE: &[u8] = b"Subtype";
const TYPE_0: &[u8] = b"Type0";
const TYPE_1: &[u8] = b"Type1";
const ENCODING: &[u8] = b"Encoding";
const TYPE_1_MM: &[u8] = b"MMType1";
const TYPE_3: &[u8] = b"Type3";
const TRUE_TYPE: &[u8] = b"TrueType";

pub struct Type1FontProgram<'a> {
    pub clear_portion: &'a [u8],
    pub program_portion: &'a [u8],
}

// PDF 9.9.1 Table 124 - Not technically accurate, Type1C, CID Font, and OpenType
// are subtypes rather than types themselves.
#[derive(Debug)]
pub enum FontProgramKind<'a> {
    Type1(&'a Stream),
    TrueType(Box<Face<'a>>),
    Type1C,
    CIDFontType0C,
    OpenType(&'a Stream),
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

impl<'a> TryFrom<(&'a Dictionary, &'a ObjectMap)> for Font<'a> {
    type Error = PdfError;

    fn try_from((dict, object_map): (&'a Dictionary, &'a ObjectMap)) -> Result<Self> {
        let name: &Name = object_map.follow_till(dict.get(SUB_TYPE)).ok_or(PdfError::ObjectNotName)?;
        match name.0.as_ref() {
            TYPE_0 => Ok(Font::Type0(Type0Font::try_from((dict, object_map))?)),
            TYPE_1 | TYPE_1_MM => Ok(Font::Type1(Type1Font::try_from((dict, object_map))?)),
            TYPE_3 => Ok(Font::Type3(Type3Font::try_from((dict, object_map))?)),
            TRUE_TYPE => Ok(Font::TrueType(TrueTypeFont::try_from((dict, object_map))?)),
            _ => Err(PdfError::InvalidFontName)
        }
    }
}

#[derive(Debug)]
pub struct FontDictionary<'a>(pub BTreeMap<Cow<'a, Name>, Font<'a>>);

impl<'a> FontDictionary<'a> {
    pub const fn new() -> Self {
        Self(BTreeMap::new())
    }
}

impl<'a> TryFrom<(&'a Dictionary, &'a ObjectMap)> for FontDictionary<'a> {
    type Error = PdfError;

    fn try_from((dict, object_map): (&'a Dictionary, &'a ObjectMap)) -> Result<Self> {
        let mut font_dictionary = FontDictionary::new();
        for (name, object) in &dict.0 {
            let name = Name(name.clone());
            if let Some(dict) = object_map.follow_till(Some(object)) {
                if let Ok(font) = Font::try_from((dict, object_map)) {
                    font_dictionary.0.insert(std::borrow::Cow::Owned(name), font);
                }
            }
        }
        Ok(font_dictionary)
    }
}

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
