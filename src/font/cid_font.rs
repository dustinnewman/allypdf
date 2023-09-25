use std::borrow::Cow;
use std::convert::{TryFrom, TryInto};

use crate::document::document::{ObjectMap, ReferenceResolver};
use crate::error::{PdfError, Result};
use crate::parser::object::{Dictionary, Name, Object, ObjectKind, Stream};

use super::font_descriptor::FontDescriptor;
use super::glyph_width::{object_array_to_glyph_widths, GlyphWidth};

const IDENTITY: &[u8] = b"Identity";
const BASE_FONT: &[u8] = b"BaseFont";
const SUB_TYPE: &[u8] = b"Subtype";
const FONT_DESCRIPTOR: &[u8] = b"FontDescriptor";
const CID_SYSTEM_INFO: &[u8] = b"CIDSystemInfo";
const DEFAULT_WIDTH: &[u8] = b"DW";
const REGISTRY: &[u8] = b"Registry";
const ORDERING: &[u8] = b"Ordering";
const SUPPLEMENT: &[u8] = b"Supplement";
const DEFAULT_VERTICAL_WIDTH: &[u8] = b"DW2";
const GLYPH_WIDTHS: &[u8] = b"W";
const VERTICAL_GLYPH_WIDTHS: &[u8] = b"W2";
const CID_TO_GID_MAP: &[u8] = b"CIDToGIDMap";
const CID_FONT_TYPE_0: &[u8] = b"CIDFontType0";
const CID_FONT_TYPE_2: &[u8] = b"CIDFontType2";

// PDF 9.5 Table 108
#[derive(Debug)]
pub enum CIDFontSubtypeKind {
    CIDFontType0,
    CIDFontType2,
}

impl TryFrom<&Name> for CIDFontSubtypeKind {
    type Error = PdfError;

    fn try_from(name: &Name) -> Result<Self> {
        match name.0.as_ref() {
            CID_FONT_TYPE_0 => Ok(Self::CIDFontType0),
            CID_FONT_TYPE_2 => Ok(Self::CIDFontType2),
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

impl<'a> TryFrom<(&'a Dictionary, &'a ObjectMap)> for CIDFont<'a> {
    type Error = PdfError;

    fn try_from((dict, object_map): (&'a Dictionary, &'a ObjectMap)) -> Result<Self> {
        let subtype = object_map
            .follow_till(dict.get(SUB_TYPE))
            .ok_or(PdfError::ObjectNotName)
            .and_then(|name: &Name| CIDFontSubtypeKind::try_from(name))?;
        let font_descriptor = object_map
            .follow_till(dict.get(FONT_DESCRIPTOR))
            .ok_or(PdfError::ObjectNotDictionary)
            .and_then(|obj| FontDescriptor::try_from((obj, object_map)))?;
        let base_font = object_map
            .follow_till(dict.get(BASE_FONT))
            .ok_or(PdfError::ObjectNotName)?;
        let cid_system_info: &Dictionary = object_map
            .follow_till(dict.get(CID_SYSTEM_INFO))
            .ok_or(PdfError::ObjectNotDictionary)?;
        let registry: &Vec<u8> = object_map
            .follow_till(cid_system_info.get(REGISTRY))
            .ok_or(PdfError::ObjectNotString)?;
        let ordering: &Vec<u8> = object_map
            .follow_till(cid_system_info.get(ORDERING))
            .ok_or(PdfError::ObjectNotString)?;
        let supplement = object_map
            .follow_till(cid_system_info.get(SUPPLEMENT))
            .ok_or(PdfError::ObjectNotInteger)?;
        let cid_system_info = CidSystemInfo {
            registry: registry.into(),
            ordering: ordering.into(),
            supplement,
        };
        let default_width = object_map
            .follow_till(dict.get(DEFAULT_WIDTH))
            .unwrap_or(1000.00);
        let vertical_default_width = object_map
            .follow_till(dict.get(DEFAULT_VERTICAL_WIDTH))
            .map_or((880.0, 1000.0), |array: &Vec<Object>| {
                (
                    array
                        .get(0)
                        .and_then(|o| f64::try_from(o).ok())
                        .unwrap_or(880.0),
                    array
                        .get(1)
                        .and_then(|o| f64::try_from(o).ok())
                        .unwrap_or(1000.0),
                )
            });
        let widths = object_map
            .follow_till(dict.get(GLYPH_WIDTHS))
            .map_or(vec![], |a: &Vec<Object>| {
                object_array_to_glyph_widths(a.as_ref())
            });
        let vertical_widths = object_map
            .follow_till(dict.get(VERTICAL_GLYPH_WIDTHS))
            .and_then(|a: &Vec<Object>| Some(object_array_to_glyph_widths(a.as_ref())));
        let cid_to_gid_map: &Object = object_map
            .follow_till(dict.get(CID_TO_GID_MAP))
            .ok_or(PdfError::InvalidCIDToGIDMap)?;
        let cid_to_gid_map = cid_to_gid_map.try_into()?;
        let cid_font = CIDFont {
            subtype,
            base_font,
            cid_system_info,
            font_descriptor,
            default_width,
            widths,
            vertical_default_width,
            vertical_widths,
            cid_to_gid_map,
        };
        Ok(cid_font)
    }
}
