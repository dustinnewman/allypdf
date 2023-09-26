use std::borrow::Cow;
use std::{collections::BTreeMap, convert::TryFrom};

use ttf_parser::Face;

use super::Font;
use crate::document::{ObjectMap, ReferenceResolver};
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
                    font_dictionary
                        .0
                        .insert(std::borrow::Cow::Owned(name), font);
                }
            }
        }
        Ok(font_dictionary)
    }
}
