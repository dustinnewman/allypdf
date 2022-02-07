use std::{collections::BTreeMap, convert::TryFrom, path::PathBuf};

use crate::{
    error::{PdfError, Result},
    parser::{
        lexer::Lexer,
        parser::{
            Dictionary, IndirectReference, Name, Object, ObjectKind, Parser, Stream, Trailer,
            XrefSection,
        },
    },
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

    fn catalog(&self) -> Option<&Dictionary> {
        match self.get(&self.trailer.root)? {
            Object { kind: ObjectKind::Dictionary(dict), .. } => Some(dict),
            _ => None
        }
    }

    fn traverse_pages(&self, cur: &IndirectReference, refs: &mut Vec<IndirectReference>) {
        let dict = match self.get(cur) {
            Some(Object { kind: ObjectKind::Dictionary(dict), .. }) => dict,
            _ => return
        };
        let r#type = match dict.get(TYPE) {
            Some(Object { kind: ObjectKind::Name(name), .. }) => name,
            _ => return
        };
        if r#type == PAGE {
            refs.push(*cur);
        } else if r#type == PAGE_ROOT {
            let kids = match dict.get(KIDS) {
                Some(Object { kind: ObjectKind::Array(arr), .. }) => arr,
                _ => return
            };
            for kid in kids {
                let r#ref = match kid {
                    Object { kind: ObjectKind::IndirectReference(r#ref), .. } => r#ref,
                    _ => continue
                };
                self.traverse_pages(r#ref, refs);
            }
        }
    }

    fn pages(&self) -> Option<Vec<IndirectReference>> {
        let catalog = self.catalog()?;
        let page_root = match catalog.get(PAGE_ROOT)? {
            Object { kind: ObjectKind::IndirectReference(r#ref), .. } => r#ref,
            _ => return None
        };
        let mut refs = vec![];
        self.traverse_pages(page_root, &mut refs);
        Some(refs)
    }
}

impl TryFrom<PathBuf> for PDFDocument {
    type Error = PdfError;
    fn try_from(pathbuf: PathBuf) -> Result<PDFDocument> {
        let file = std::fs::read(pathbuf).unwrap();
        let tokens = Lexer::new(&file).lex();
        let mut parser = Parser::new(&tokens);
        let objects = parser.parse();
        println!("{:?}", objects);
        PDFDocument::try_from(objects)
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
            match obj.kind {
                ObjectKind::Header(m, n) => version = Ok((m, n)),
                ObjectKind::Trailer(t) => trailer = Ok(t),
                ObjectKind::StartXref(s) => start_xref = Ok(s),
                ObjectKind::IndirectObject(ind) => {
                    let key = IndirectReference {
                        object_number: ind.object_number,
                        generation_number: ind.generation_number,
                    };
                    object_map.insert(key, *ind.object);
                }
                ObjectKind::Xref(x) => xref_section = Ok(x),
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
    use crate::{array, dict, indirect_reference, inner, integer, name, offset, stream};
    use std::path::PathBuf;

    macro_rules! get {
        ($dict:expr, $key:expr) => {
            $dict.get(&$key).unwrap()
        };
    }

    #[test]
    fn test_document_hello_catalog() {

    }

    #[test]
    fn test_document_hello_pages() {
        
    }

    #[test]
    fn test_hello() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("test_data/hello.pdf");
        let doc = PDFDocument::try_from(file).unwrap();
        assert_eq!(doc.version, (1, 4));
        assert_eq!(doc.start_xref, 491);
        assert_eq!(
            doc.trailer.root,
            IndirectReference {
                object_number: 1,
                generation_number: 0
            }
        );
        let expected_id_string = vec![1, 35, 69, 103, 137, 10, 188, 222, 240];
        let expected_trailer = dict!(
            b"Root" => indirect_reference!(1),
            b"Size" => integer!(8),
            b"ID" => array![offset!(ObjectKind::String(expected_id_string.clone())), offset!(ObjectKind::String(expected_id_string))]
        );
        assert_eq!(
            &doc.trailer.dictionary,
            inner!(
                &expected_trailer.kind,
                ObjectKind::Dictionary,
                "Expected trailer is not a dictionary."
            )
        );
        assert_eq!(doc.trailer.size, 8);
        let catalog = inner!(
            &get!(doc, doc.trailer.root).kind,
            ObjectKind::Dictionary,
            "Catalog is not a dictionary"
        );
        assert!(
            matches!(&get!(catalog, b"Type".to_vec()).kind, ObjectKind::Name(x) if *x == b"Catalog".to_vec())
        );
        let pages = inner!(
            get!(catalog, b"Pages".to_vec()).kind,
            ObjectKind::IndirectReference,
            "Catalog's pages is not indirect reference."
        );
        let pages = get!(doc, pages);
        let expected_pages = dict!(
            b"Type" => name!("Pages"),
            b"Kids" => array!(indirect_reference!(3)),
            b"Count" => integer!(1)
        );
        assert_eq!(pages, &expected_pages);
        let pages = inner!(
            &pages.kind,
            ObjectKind::Dictionary,
            "Pages is not a dictionary."
        );
        let kids = inner!(
            &get!(pages, b"Kids".to_vec()).kind,
            ObjectKind::Array,
            "Pages' kids is not array."
        );
        let kids = inner!(
            kids[0].kind,
            ObjectKind::IndirectReference,
            "Kids is not indirect reference."
        );
        let kids = get!(doc, kids);
        let expected_kids = dict!(
            b"Type" => name!("Page"),
            b"MediaBox" => array!(integer!(0), integer!(0), integer!(612), integer!(792)),
            b"Parent" => indirect_reference!(2),
            b"Resources" => indirect_reference!(4),
            b"Contents" => indirect_reference!(5)
        );
        assert_eq!(kids, &expected_kids);
        let kids = inner!(
            &kids.kind,
            ObjectKind::Dictionary,
            "Kids is not dictionary."
        );
        let resources_ref = get!(kids, b"Resources".to_vec());
        assert_eq!(resources_ref, &indirect_reference!(4));
        let resources_ref = inner!(
            resources_ref.kind,
            ObjectKind::IndirectReference,
            "Kids' resources field is not indirect reference."
        );
        let resources = get!(doc, resources_ref);
        let expected_resources = dict!(
            b"ProcSet" => array!(name!("PDF")),
            b"Font" => indirect_reference!(6)
        );
        assert_eq!(resources, &expected_resources);
        let resources = inner!(
            &resources.kind,
            ObjectKind::Dictionary,
            "Resources is not a dictionary"
        );
        let font_ref = get!(resources, b"Font".to_vec());
        assert_eq!(font_ref, &indirect_reference!(6));
        let font_ref = inner!(
            font_ref.kind,
            ObjectKind::IndirectReference,
            "Resources' font field is not indirect reference."
        );
        let font = get!(doc, font_ref);
        let expected_font = dict!(
            b"F0" => indirect_reference!(8)
        );
        assert_eq!(font, &expected_font);
        let font = inner!(
            &font.kind,
            ObjectKind::Dictionary,
            "Font is not a dictionary."
        );
        let helvetica_ref = get!(font, b"F0".to_vec());
        assert_eq!(helvetica_ref, &indirect_reference!(8));
        let helvetica_ref = inner!(
            helvetica_ref.kind,
            ObjectKind::IndirectReference,
            "F0 is not indirect reference."
        );
        let helvetica = get!(doc, helvetica_ref);
        let expected_helvetica = dict!(
            b"Type" => name!("Font"),
            b"Subtype" => name!("Type1"),
            b"BaseFont" => name!("Helvetica")
        );
        assert_eq!(helvetica, &expected_helvetica);
        let contents = get!(kids, b"Contents".to_vec());
        assert_eq!(contents, &indirect_reference!(5));
        let contents = inner!(
            contents.kind,
            ObjectKind::IndirectReference,
            "Kids' contents field is not indirect reference."
        );
        let contents = get!(doc, contents);
        let expected_content = b"BT\n/F0 12 Tf\n100 700 Td\n(Hello, World) Tj\nET\n";
        let expected_contents = stream!(
            expected_content.to_vec(),
            b"Length" => indirect_reference!(7)
        );
        assert_eq!(contents, &expected_contents);
        let contents = inner!(
            &contents.kind,
            ObjectKind::Stream,
            "Contents is not a stream."
        );
        let length = get!(contents.dict, b"Length".to_vec());
        assert_eq!(length, &indirect_reference!(7));
        let length = inner!(
            length.kind,
            ObjectKind::IndirectReference,
            "Contents' length is not indirect reference."
        );
        let length_object = get!(doc, length);
        let expected_length_object = integer!(51);
        assert_eq!(length_object, &expected_length_object);
    }

    #[test]
    fn test_mostly_postscript_contents() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("test_data/heinz.pdf");
        let doc = PDFDocument::try_from(file).unwrap();
        println!("{:?}", doc);
        assert!(false);
    }

    #[test]
    fn test_pdf_2_spec() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("test_data/pdf.pdf");
        let doc = PDFDocument::try_from(file).unwrap();
        println!("{:?}", doc);
        assert!(false);
    }
}
