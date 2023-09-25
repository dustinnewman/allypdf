use std::convert::{TryFrom, TryInto};

use crate::cmaps::cid::{CharCode, CharCodeToGlyphName, CharCodeToUnicode};
use crate::cmaps::cmap::CMap;
use crate::document::document::{ObjectMap, ReferenceResolver};
use crate::error::{PdfError, Result};
use crate::parser::object::{Name, Dictionary, Object, Stream};

use super::encoding::{ENCODING_SIZE, Encoding};
use super::font_descriptor::FontDescriptor;

const NAME: &[u8] = b"Name";
const BASE_FONT: &[u8] = b"BaseFont";
const FIRST_CHAR: &[u8] = b"FirstChar";
const LAST_CHAR: &[u8] = b"LastChar";
const WIDTHS: &[u8] = b"Widths";
const FONT_DESCRIPTOR: &[u8] = b"FontDescriptor";
const ENCODING: &[u8] = b"Encoding";
const TO_UNICODE: &[u8] = b"ToUnicode";

// PDF 9.6.3
#[derive(Debug)]
pub struct TrueTypeFont<'a> {
    pub name: Option<&'a Name>,
    pub base_font: &'a Name,
    pub first_char: Option<CharCode>,
    pub last_char: Option<CharCode>,
    pub widths: Option<[f64; ENCODING_SIZE]>,
    pub font_descriptor: Option<FontDescriptor<'a>>,
    pub encoding: Option<Encoding<'a>>,
    pub to_unicode: Option<CMap<'a>>,
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

impl<'a> TryFrom<(&'a Dictionary, &'a ObjectMap)> for TrueTypeFont<'a> {
    type Error = PdfError;

    fn try_from((dict, object_map): (&'a Dictionary, &'a ObjectMap)) -> Result<Self> {
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
        let to_unicode = object_map
            .follow_till(dict.get(TO_UNICODE))
            .and_then(|stream: &Stream| TryInto::<CMap>::try_into(stream).ok());
        Ok(TrueTypeFont {
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
