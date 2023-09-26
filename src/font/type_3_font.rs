use std::convert::{TryFrom, TryInto};

use crate::cmap::cid::CharCode;
use crate::document::resources::Resources;
use crate::document::{ObjectMap, ReferenceResolver};
use crate::error::{PdfError, Result};
use crate::operators::matrix::Matrix;
use crate::operators::rect::Rectangle;
use crate::parser::object::{Dictionary, Name, Object, Stream};

use super::encoding::{Encoding, ENCODING_SIZE};
use super::font_descriptor::FontDescriptor;

const NAME: &[u8] = b"Name";
const RESOURCES: &[u8] = b"Resources";
const FONT_DESCRIPTOR: &[u8] = b"FontDescriptor";
const FONT_B_BOX: &[u8] = b"FontBBox";
const FONT_MATRIX: &[u8] = b"FontMatrix";
const CHAR_PROCS: &[u8] = b"CharProcs";
const FIRST_CHAR: &[u8] = b"FirstChar";
const LAST_CHAR: &[u8] = b"LastChar";
const WIDTHS: &[u8] = b"Widths";
const ENCODING: &[u8] = b"Encoding";
const TO_UNICODE: &[u8] = b"ToUnicode";

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

impl<'a> TryFrom<(&'a Dictionary, &'a ObjectMap)> for Type3Font<'a> {
    type Error = PdfError;

    fn try_from((dict, object_map): (&'a Dictionary, &'a ObjectMap)) -> Result<Self> {
        let name = object_map.follow_till(dict.get(NAME));
        let resources = object_map
            .follow_till(dict.get(RESOURCES))
            .and_then(|dict: &Dictionary| Resources::try_from((dict, object_map)).ok());
        let font_b_box = object_map
            .follow_till(dict.get(FONT_B_BOX))
            .and_then(|object: &Object| Rectangle::try_from(object).ok())
            .ok_or(PdfError::ObjectNotArray)?;
        let font_matrix: Vec<f64> = object_map
            .follow_till(dict.get(FONT_MATRIX))
            .map(|a: &Vec<Object>| a.iter().filter_map(|o| o.try_into().ok()).collect())
            .ok_or(PdfError::ObjectNotArray)?;
        let font_matrix: [f64; 6] = font_matrix
            .try_into()
            .map_err(|_| PdfError::ParseF64Error)?;
        let font_matrix = Matrix::new(font_matrix);
        let char_procs = object_map
            .follow_till(dict.get(CHAR_PROCS))
            .ok_or(PdfError::ObjectNotDictionary)?;
        let first_char = object_map
            .follow_till(dict.get(FIRST_CHAR))
            .ok_or(PdfError::ParseU32Error)?;
        let last_char = object_map
            .follow_till(dict.get(LAST_CHAR))
            .ok_or(PdfError::ParseU32Error)?;
        let src_widths: Vec<f64> = object_map
            .follow_till(dict.get(WIDTHS))
            .ok_or(PdfError::ObjectNotArray)
            .map(|a: &Vec<Object>| a.iter().filter_map(|o| o.try_into().ok()).collect())?;
        let src_widths: &[f64] = &src_widths[0..((last_char - first_char) as usize)];
        let font_descriptor = object_map
            .follow_till(dict.get(FONT_DESCRIPTOR))
            .ok_or(PdfError::ObjectNotDictionary)
            .and_then(|obj| FontDescriptor::try_from((obj, object_map)))?;
        let mut widths = [font_descriptor.missing_width; 256];
        widths[(first_char as usize)..(last_char as usize + 1)].clone_from_slice(src_widths);
        let encoding = object_map
            .follow_till(dict.get(ENCODING))
            .ok_or(PdfError::InvalidDefaultEncodingName)
            .and_then(|obj: &Object| Encoding::try_from(obj))?;
        let to_unicode = object_map.follow_till(dict.get(TO_UNICODE));
        Ok(Type3Font {
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
        })
    }
}
