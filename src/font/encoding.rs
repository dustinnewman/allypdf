use std::{
    convert::TryFrom,
    ops::{Deref, DerefMut},
};

use super::adobe_glyph_list::adobe_glyph_list;
use super::base_encodings::{MAC_ROMAN_ENCODING, STANDARD_ENCODING, WIN_ANSI_ENCODING};
use crate::cmaps::cid::{CharCode, CharCodeToGlyphName, GlyphNameToUnicode};
use crate::error::{PdfError, Result};
use crate::parser::object::{Dictionary, Name, Object, ObjectKind};

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
            .and_then(|entry| entry.and_then(Some))
    }
}

impl<'a> GlyphNameToUnicode<'a, 'static> for Encoding<'a> {
    fn get_unicode(&self, glyph_name: &'a [u8]) -> Option<&'static str> {
        adobe_glyph_list(glyph_name)
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

impl TryFrom<&Name> for Encoding<'_> {
    type Error = PdfError;

    fn try_from(name: &Name) -> Result<Self> {
        if name == &MAC_ROMAN {
            Ok(MAC_ROMAN_ENCODING)
        } else if name == &WIN_ANSI {
            Ok(WIN_ANSI_ENCODING)
        } else if name == &MAC_EXPERT {
            todo!()
        } else {
            Err(PdfError::InvalidDefaultEncodingName)
        }
    }
}

impl<'a> TryFrom<&'a Dictionary> for Encoding<'a> {
    type Error = PdfError;

    fn try_from(dict: &'a Dictionary) -> Result<Self> {
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
                        encoding[array_index] = Some(&name.0);
                        array_index += 1;
                    }
                    _ => continue,
                }
            }
        }
        Ok(encoding)
    }
}

impl<'a> TryFrom<&'a Object> for Encoding<'a> {
    type Error = PdfError;

    fn try_from(object: &'a Object) -> Result<Self> {
        // If the Encoding entry is a dictionary, the table shall be
        // initialised with the entries from the dictionary’s
        // BaseEncoding entry. Any entries in the Differences array
        // shall be used to update the table. Finally, any undefined
        // entries in the table shall be filled using StandardEncoding.
        // - PDF 9.6.5.4 paragraph 6 item 2
        // If the Encoding entry is one of the names MacRomanEncoding
        // or WinAnsiEncoding, the table shall be initialised with the
        // mappings described in Annex D, "Character Sets and Encodings".
        // - PDF 9.6.5.4 paragraph 6 item 1
        match &object.kind {
            ObjectKind::Dictionary(dict) => Self::try_from(dict),
            ObjectKind::Name(name) => Self::try_from(name),
            _ => Err(PdfError::InvalidDefaultEncodingName),
        }
    }
}
