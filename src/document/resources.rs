use std::convert::{TryFrom, TryInto};

use crate::error::{PdfError, Result};
use crate::font::font_dictionary::FontDictionary;
use crate::parser::object::{Dictionary, Name, Object};

use super::{ObjectMap, ReferenceResolver};

const PDF: &[u8] = b"PDF";
const TEXT: &[u8] = b"Text";
const IMAGE_B: &[u8] = b"ImageB";
const IMAGE_C: &[u8] = b"ImageC";
const IMAGE_I: &[u8] = b"ImageI";
const EXT_G_STATE: &[u8] = b"ExtGState";
const COLOR_SPACE: &[u8] = b"ColorSpace";
const PATTERN: &[u8] = b"Pattern";
const SHADING: &[u8] = b"Shading";
const X_OBJECT: &[u8] = b"XObject";
pub const FONT: &[u8] = b"Font";
const PROC_SET: &[u8] = b"ProcSet";
const PROPERTIES: &[u8] = b"Properties";

#[derive(Debug)]
pub enum ProcSet {
    Pdf,
    Text,
    ImageBlack,
    ImageColor,
    ImageIndexed,
}

impl TryFrom<&Object> for ProcSet {
    type Error = PdfError;

    fn try_from(object: &Object) -> Result<Self> {
        let name: &Name = object.try_into()?;
        match name.0.as_ref() {
            PDF => Ok(Self::Pdf),
            TEXT => Ok(Self::Text),
            IMAGE_B => Ok(Self::ImageBlack),
            IMAGE_C => Ok(Self::ImageColor),
            IMAGE_I => Ok(Self::ImageIndexed),
            _ => Err(PdfError::InvalidProcSet),
        }
    }
}

#[derive(Debug)]
pub struct Resources<'a> {
    pub ext_g_state: Option<&'a Dictionary>,
    pub color_space: Option<&'a Dictionary>,
    pub pattern: Option<&'a Dictionary>,
    pub shading: Option<&'a Dictionary>,
    pub x_object: Option<&'a Dictionary>,
    pub font: Option<FontDictionary<'a>>,
    pub proc_set: Option<Vec<ProcSet>>,
    pub properties: Option<&'a Dictionary>,
}

impl<'a> TryFrom<(&'a Dictionary, &'a ObjectMap)> for Resources<'a> {
    type Error = PdfError;

    fn try_from((dict, object_map): (&'a Dictionary, &'a ObjectMap)) -> Result<Self> {
        let ext_g_state = object_map.follow_till(dict.get(EXT_G_STATE));
        let color_space = object_map.follow_till(dict.get(COLOR_SPACE));
        let pattern = object_map.follow_till(dict.get(PATTERN));
        let shading = object_map.follow_till(dict.get(SHADING));
        let x_object = object_map.follow_till(dict.get(X_OBJECT));
        let font = object_map
            .follow_till(dict.get(FONT))
            .and_then(|dict| FontDictionary::try_from((dict, object_map)).ok());
        let proc_set = object_map
            .follow_till(dict.get(PROC_SET))
            .map(|vec: &Vec<Object>| {
                vec.iter()
                    .filter_map(|obj| ProcSet::try_from(obj).ok())
                    .collect()
            });
        let properties = object_map.follow_till(dict.get(PROPERTIES));
        Ok(Self {
            ext_g_state,
            color_space,
            pattern,
            shading,
            x_object,
            font,
            proc_set,
            properties,
        })
    }
}
