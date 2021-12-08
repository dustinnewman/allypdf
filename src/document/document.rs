use std::{collections::BTreeMap, convert::TryFrom};

use crate::{
    error::{PdfError, Result},
    parser::parser::{Dictionary, IndirectReference, Name, Object, Stream, Trailer, XrefSection},
};

const TYPE: &[u8] = b"Type";
const PAGE_ROOT: &[u8] = b"Pages";
const KIDS: &[u8] = b"Kids";
const PAGE: &[u8] = b"Page";
const PARENT: &[u8] = b"Parent";
const MEDIA_BOX: &[u8] = b"MediaBox";
const CROP_BOX: &[u8] = b"CropBox";
const BLEED_BOX: &[u8] = b"BleedBox";
const TRIM_BOX: &[u8] = b"TrimBox";
const ART_BOX: &[u8] = b"ArtBox";
const CONTENTS: &[u8] = b"Contents";

struct Annotation {
    subtype: Name,
    rect: Rectangle,
    flags: u32,
}

enum ProcSet {
    PDF,
    Text,
    ImageBlack,
    ImageColor,
    ImageIndexed,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rectangle {
    pub lower_left_x: f32,
    pub lower_left_y: f32,
    pub upper_right_x: f32,
    pub upper_right_y: f32,
}

pub struct Page<'a> {
    parent: IndirectReference,
    media_box: Rectangle,
    crop_box: Rectangle,
    bleed_box: Rectangle,
    trim_box: Rectangle,
    art_box: Rectangle,
    contents: Vec<&'a Stream>,
    annotations: Vec<Annotation>,
    resources: &'a Dictionary,
}

pub struct PagesRoot<'a> {
    kids: Vec<Page<'a>>,
    count: u64,
}

pub struct Catalog<'a> {
    pages: PagesRoot<'a>,
}

pub type Version = (u8, u8);
pub type ObjectMap = BTreeMap<IndirectReference, Object>;

pub struct PDFDocument {
    // (major, minor)
    version: Version,
    trailer: Trailer,
    xref_table: XrefSection,
    start_xref: u64,
    object_map: ObjectMap,
}

impl PDFDocument {
    pub fn get(&self, key: &IndirectReference) -> Option<&Object> {
        self.object_map.get(key)
    }
}

impl TryFrom<Vec<Object>> for PDFDocument {
    type Error = PdfError;
    fn try_from(objects: Vec<Object>) -> Result<PDFDocument> {
        let mut version = Err(PdfError::NoHeader);
        let mut trailer = Err(PdfError::NoTrailer);
        let mut xref_section = Err(PdfError::NoCrossReferences);
        let mut start_xref = Err(PdfError::NoStartXref);
        let mut object_map: ObjectMap = BTreeMap::new();
        for obj in objects {
            match obj {
                Object::Header(m, n) => version = Ok((m, n)),
                Object::Trailer(t) => trailer = Ok(t),
                Object::StartXref(s) => start_xref = Ok(s),
                Object::IndirectObject(ind) => {
                    let key = IndirectReference {
                        object_number: ind.object_number,
                        generation_number: ind.generation_number,
                    };
                    object_map.insert(key, *ind.object);
                }
                Object::Xref(x) => xref_section = Ok(x),
                _ => continue,
            }
        }
        Ok(PDFDocument {
            version: version?,
            trailer: trailer?,
            xref_table: xref_section?,
            start_xref: start_xref?,
            object_map,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::{lexer::Lexer, parser::Parser};
    use std::{fs, path::PathBuf};

    #[test]
    fn test_hello() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("test_data/hello.pdf");
        let file = fs::read(file).unwrap();
        let tokens = Lexer::new(&file).lex();
        let mut parser = Parser::new(&tokens);
        let doc = PDFDocument::try_from(parser.parse()).unwrap();
        assert_eq!(doc.version, (1, 4));
        assert_eq!(doc.start_xref, 491);
        let catalog = match doc.get(&doc.trailer.root).unwrap() {
            Object::Dictionary(d) => d,
            _ => panic!("Catalog is not a dictionary"),
        };
        assert!(
            matches!(catalog.get(&b"Type".to_vec()).unwrap(), Object::Name(x) if *x == b"Catalog".to_vec())
        );
        let pages = match catalog.get(&b"Pages".to_vec()) {
            Some(Object::IndirectReference(
                r
                @
                IndirectReference {
                    object_number,
                    generation_number,
                },
            )) if *object_number == 2 && *generation_number == 0 => r,
            _ => panic!("Catalog's pages is not indirect reference."),
        };
    }
}
