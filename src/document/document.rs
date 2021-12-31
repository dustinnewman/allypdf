use std::{collections::BTreeMap, convert::TryFrom, path::PathBuf};

use crate::{
    error::{PdfError, Result},
    parser::{parser::{Dictionary, IndirectReference, Name, Object, Stream, Trailer, XrefSection, Parser}, lexer::Lexer},
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

#[derive(Debug)]
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

impl TryFrom<PathBuf> for PDFDocument {
    type Error = PdfError;
    fn try_from(pathbuf: PathBuf) -> Result<PDFDocument> {
        let file = std::fs::read(pathbuf).unwrap();
        let tokens = Lexer::new(&file).lex();
        let mut parser = Parser::new(&tokens);
        let tokens = parser.parse();
        println!("Tokens: {:?}", tokens);
        PDFDocument::try_from(tokens)
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
    use crate::{array, name, indirect_reference, integer, dict};
    use std::{path::PathBuf};

    macro_rules! inner {
        ($outer:expr, $inner:path, $msg:literal) => {
            match $outer {
                $inner(i) => i,
                _ => panic!($msg),
            }
        };
    }

    macro_rules! get {
        ($dict:expr, $key:expr) => (
            $dict.get(&$key).unwrap()
        );
    }

    #[test]
    fn test_hello() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("test_data/hello.pdf");
        let doc = PDFDocument::try_from(file).unwrap();
        assert_eq!(doc.version, (1, 4));
        assert_eq!(doc.start_xref, 491);
        let catalog = inner!(get!(doc, doc.trailer.root), Object::Dictionary, "Catalog is not a dictionary");
        assert!(
            matches!(get!(catalog, b"Type".to_vec()), Object::Name(x) if *x == b"Catalog".to_vec())
        );
        let pages = inner!(get!(catalog, b"Pages".to_vec()), Object::IndirectReference, "Catalog's pages is not indirect reference.");
        let pages = get!(doc, pages);
        let pages_rhs = dict!(
            b"Type" => name!("Pages"),
            b"Kids" => array!(indirect_reference!(3)),
            b"Count" => integer!(1)
        );
        assert_eq!(pages, &pages_rhs);
        let pages = inner!(pages, Object::Dictionary, "Pages is not a dictionary.");
        let kids = inner!(get!(pages, b"Kids".to_vec()), Object::Array, "Pages' kids is not array.");
        let kids = inner!(kids[0], Object::IndirectReference, "Kids is not indirect reference.");
        let kids = get!(doc, kids);
        let kids_rhs = dict!(
            b"Type" => name!("Page"),
            b"MediaBox" => array!(integer!(0), integer!(0), integer!(612), integer!(792)),
            b"Parent" => indirect_reference!(2),
            b"Resources" => indirect_reference!(4),
            b"Contents" => indirect_reference!(5)
        );
        assert_eq!(kids, &kids_rhs);
        let kids = inner!(kids, Object::Dictionary, "Kids is not dictionary.");
    }

    #[test]
    fn test_mostly_postscript_contents() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("test_data/heinz.pdf");
        let doc = PDFDocument::try_from(file).unwrap();
        println!("{:#?}", doc);
        assert!(false);
    }
}
