use std::{
    convert::TryFrom,
    ops::{Deref, DerefMut},
};

use super::base_encodings::{MAC_ROMAN_ENCODING, STANDARD_ENCODING, WIN_ANSI_ENCODING};
use super::cmap::CharCodeToGlyphName;
use super::font::CharCode;
use crate::error::PdfError;
use crate::parser::parser::{Dictionary, Name, Object, ObjectKind};

pub const ENCODING_SIZE: usize = 256;
const BASE_ENCODING: &[u8] = b"BaseEncoding";
const DIFFERENCES: &[u8] = b"Differences";
const MAC_ROMAN: &[u8] = b"MacRomanEncoding";
const WIN_ANSI: &[u8] = b"WinAnsiEncoding";
const MAC_EXPERT: &[u8] = b"MacExpertEncoding";

pub enum DefaultBaseEncoding {
    MacRoman,
    MacExpert,
    WinAnsi,
}

#[derive(Debug)]
pub struct Encoding<'a>(pub [Option<&'a [u8]>; ENCODING_SIZE]);

impl<'a> Encoding<'a> {
    pub fn new(inner: [Option<&'a [u8]>; ENCODING_SIZE]) -> Self {
        Self(inner)
    }
}

impl CharCodeToGlyphName for Encoding<'_> {
    fn get_glyph_name(&self, char_code: CharCode) -> Option<&[u8]> {
        self.get(char_code as usize)
            .and_then(|entry| entry.and_then(|name| Some(name)))
    }
}

impl<'a> Default for Encoding<'a> {
    fn default() -> Self {
        STANDARD_ENCODING
    }
}

impl<'a> Deref for Encoding<'a> {
    type Target = [Option<&'a [u8]>; ENCODING_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Encoding<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> TryFrom<&'a Name> for Encoding<'a> {
    type Error = PdfError;

    fn try_from(name: &'a Name) -> Result<Self, Self::Error> {
        if name == MAC_ROMAN {
            Ok(MAC_ROMAN_ENCODING)
        } else if name == WIN_ANSI {
            Ok(WIN_ANSI_ENCODING)
        } else if name == MAC_EXPERT {
            todo!()
        } else {
            Err(PdfError::Other {
                msg: "Could not convert dictionary to encoding.".to_string(),
            })
        }
    }
}

impl<'a> TryFrom<&'a Dictionary> for Encoding<'a> {
    type Error = PdfError;

    fn try_from(dict: &'a Dictionary) -> Result<Self, Self::Error> {
        let mut encoding = if let Some(Object {
            kind: ObjectKind::Name(name),
            ..
        }) = dict.get(BASE_ENCODING)
        {
            Self::try_from(name)?
        } else {
            Self::default()
        };
        if let Some(Object {
            kind: ObjectKind::Array(differences),
            ..
        }) = dict.get(DIFFERENCES)
        {
            let mut array_index = 0;
            for object in differences {
                match object {
                    Object {
                        kind: ObjectKind::Integer(i),
                        ..
                    } => array_index = *i as usize,
                    Object {
                        kind: ObjectKind::Name(name),
                        ..
                    } => {
                        encoding[array_index] = Some(name);
                        array_index += 1;
                    }
                    _ => continue,
                }
            }
        }
        Ok(encoding)
    }
}
