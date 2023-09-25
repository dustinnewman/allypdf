use std::convert::{TryFrom, TryInto};

use crate::cmaps::cid::CharCode;
use crate::document::document::{ObjectMap, ReferenceResolver};
use crate::error::{PdfError, Result};
use crate::parser::object::{Dictionary, Name, Object, Stream};

use super::encoding::{Encoding, ENCODING_SIZE};
use super::font_descriptor::FontDescriptor;

const SUB_TYPE: &[u8] = b"Subtype";
const TYPE_1: &[u8] = b"Type1";
const TYPE_1_MM: &[u8] = b"MMType1";
const NAME: &[u8] = b"Name";
const BASE_FONT: &[u8] = b"BaseFont";
const FIRST_CHAR: &[u8] = b"FirstChar";
const LAST_CHAR: &[u8] = b"LastChar";
const WIDTHS: &[u8] = b"Widths";
const FONT_DESCRIPTOR: &[u8] = b"FontDescriptor";
const ENCODING: &[u8] = b"Encoding";
const TO_UNICODE: &[u8] = b"ToUnicode";

// PDF 9.5 Table 108
#[derive(Debug)]
pub enum Type1SubtypeKind {
    Type1,
    MMType1,
}

impl TryFrom<&Name> for Type1SubtypeKind {
    type Error = PdfError;

    fn try_from(name: &Name) -> Result<Self> {
        match name.0.as_ref() {
            TYPE_1 => Ok(Self::Type1),
            TYPE_1_MM => Ok(Self::MMType1),
            _ => Err(PdfError::InvalidFontName),
        }
    }
}

// PDF 9.6.2.1 Table 109
#[derive(Debug)]
pub struct Type1Font<'a> {
    pub subtype: Type1SubtypeKind,
    pub name: Option<&'a Name>,
    pub base_font: &'a Name,
    pub first_char: Option<CharCode>,
    pub last_char: Option<CharCode>,
    pub widths: Option<[f64; ENCODING_SIZE]>,
    pub font_descriptor: Option<FontDescriptor<'a>>,
    pub encoding: Option<Encoding<'a>>,
    pub to_unicode: Option<&'a Stream>,
}

impl<'a> TryFrom<(&'a Dictionary, &'a ObjectMap)> for Type1Font<'a> {
    type Error = PdfError;

    fn try_from((dict, object_map): (&'a Dictionary, &'a ObjectMap)) -> Result<Self> {
        let subtype: &Name = object_map
            .follow_till(dict.get(SUB_TYPE))
            .ok_or(PdfError::InvalidFontName)?;
        let subtype = subtype.try_into()?;
        let name = object_map.follow_till(dict.get(NAME));
        let base_font = object_map
            .follow_till(dict.get(BASE_FONT))
            .ok_or(PdfError::ObjectNotName)?;
        let first_char = object_map.follow_till(dict.get(FIRST_CHAR));
        let last_char = object_map.follow_till(dict.get(LAST_CHAR));
        let src_widths: Option<Vec<f64>> = object_map
            .follow_till(dict.get(WIDTHS))
            .map(|a: &Vec<Object>| a.iter().filter_map(|o| o.try_into().ok()).collect());
        let font_descriptor = object_map
            .follow_till(dict.get(FONT_DESCRIPTOR))
            .and_then(|obj| FontDescriptor::try_from((obj, object_map)).ok());
        let widths = if let (Some(first_char), Some(last_char), Some(src_widths)) =
            (first_char, last_char, src_widths)
        {
            let mut dst_widths = [font_descriptor.as_ref().map_or(0., |fd| fd.missing_width); 256];
            dst_widths[(first_char as usize)..(last_char as usize + 1)]
                .clone_from_slice(&src_widths);
            Some(dst_widths)
        } else {
            None
        };
        let encoding = object_map
            .follow_till(dict.get(ENCODING))
            .and_then(|obj: &Object| Encoding::try_from(obj).ok());
        let to_unicode = object_map.follow_till(dict.get(TO_UNICODE));
        Ok(Type1Font {
            subtype,
            name,
            base_font,
            first_char,
            last_char,
            widths,
            font_descriptor,
            encoding,
            to_unicode,
        })
    }
}
