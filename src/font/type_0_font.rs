use std::convert::{TryFrom, TryInto};

use crate::cmaps::cid::{CharCode, CharCodeToCid, Cid};
use crate::cmaps::cmap::CMap;
use crate::document::document::{ObjectMap, ReferenceResolver};
use crate::error::{PdfError, Result};
use crate::parser::object::{Dictionary, Name, Object, ObjectKind, Stream};

use crate::font::cid_font::CIDFont;

const IDENTITY_H: &[u8] = b"Identity-H";
const IDENTITY_V: &[u8] = b"Identity-V";
const ENCODING: &[u8] = b"Encoding";
const BASE_FONT: &[u8] = b"BaseFont";
const DESCENDANT_FONTS: &[u8] = b"DescendantFonts";
const TO_UNICODE: &[u8] = b"ToUnicode";

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
    pub base_font: &'a Name,
    pub encoding: Type0Encoding<'a>,
    pub descendant_fonts: CIDFont<'a>,
    pub to_unicode: Option<CMap<'a>>,
}

impl<'a> TryFrom<(&'a Dictionary, &'a ObjectMap)> for Type0Font<'a> {
    type Error = PdfError;

    fn try_from((dict, object_map): (&'a Dictionary, &'a ObjectMap)) -> Result<Self> {
        let base_font = object_map
            .follow_till(dict.get(BASE_FONT))
            .ok_or(PdfError::ObjectNotName)?;
        let descendant_fonts: &Vec<Object> = object_map
            .follow_till(dict.get(DESCENDANT_FONTS))
            .ok_or(PdfError::ObjectNotName)?;
        let descendant_fonts = descendant_fonts.get(0);
        let descendant_fonts = object_map.follow_till(descendant_fonts).ok_or(PdfError::ObjectNotDictionary)?;
        let descendant_fonts = CIDFont::try_from((descendant_fonts, object_map))?;
        let encoding: Type0Encoding = Type0Encoding::try_from((dict.get(ENCODING), object_map))?;
        let to_unicode = object_map
            .follow_till(dict.get(TO_UNICODE))
            .and_then(|stream: &Stream| TryInto::<CMap>::try_into(stream).ok());
        Ok(Type0Font {
            base_font,
            encoding,
            descendant_fonts,
            to_unicode,
        })
    }
}
