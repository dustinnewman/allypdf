use std::{
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
    fs,
    path::PathBuf,
};

use super::annotation::{Annotation, AnnotationFlags};
use super::page::Page;
use super::resources::Resources;
use crate::error::{PdfError, Result};
use crate::inner;
use crate::operators::rect::Rectangle;
use crate::parser::lexer::{Header, Lexer};
use crate::parser::object::{
    Dictionary, IndirectReference, Name, Object, ObjectKind, Stream, Trailer, XrefSection,
};
use crate::parser::parser::Parser;
use crate::render::canvas::Canvas;

const TYPE: &[u8] = b"Type";
const NAME: &[u8] = b"Name";
const PAGE_ROOT: &[u8] = b"Pages";
const KIDS: &[u8] = b"Kids";
const PAGE: &[u8] = b"Page";
const PARENT: &[u8] = b"Parent";
const MEDIA_BOX: &[u8] = b"MediaBox";
const CROP_BOX: &[u8] = b"CropBox";
const BLEED_BOX: &[u8] = b"BleedBox";
const TRIM_BOX: &[u8] = b"TrimBox";
const ART_BOX: &[u8] = b"ArtBox";
const RESOURCES: &[u8] = b"Resources";
const FONT_DESCRIPTOR: &[u8] = b"FontDescriptor";
const FONT_NAME: &[u8] = b"FontName";
const FONT_FAMILY: &[u8] = b"FontFamily";
const FONT_STRETCH: &[u8] = b"FontStretch";
const FONT_WEIGHT: &[u8] = b"FontWeight";
const FLAGS: &[u8] = b"Flags";
const FONT_B_BOX: &[u8] = b"FontBBox";
const ITALIC_ANGLE: &[u8] = b"ItalicAngle";
const ASCENT: &[u8] = b"Ascent";
const DESCENT: &[u8] = b"Descent";
const LEADING: &[u8] = b"Leading";
const CAP_HEIGHT: &[u8] = b"CapHeight";
const X_HEIGHT: &[u8] = b"XHeight";
const STEM_V: &[u8] = b"StemV";
const STEM_H: &[u8] = b"StemH";
const AVG_WIDTH: &[u8] = b"AvgWidth";
const MAX_WIDTH: &[u8] = b"MaxWidth";
const MISSING_WIDTH: &[u8] = b"MissingWidth";
const FONT_FILE: &[u8] = b"FontFile";
const FONT_FILE_2: &[u8] = b"FontFile2";
const FONT_FILE_3: &[u8] = b"FontFile3";
const CHAR_SET: &[u8] = b"CharSet";
const CHAR_PROCS: &[u8] = b"CharProcs";
const TYPE_0: &[u8] = b"Type0";
const TYPE_1: &[u8] = b"Type1";
const ENCODING: &[u8] = b"Encoding";
const TO_UNICODE: &[u8] = b"ToUnicode";
const TYPE_1_MM: &[u8] = b"MMType1";
const TYPE_3: &[u8] = b"Type3";
const FONT_MATRIX: &[u8] = b"FontMatrix";
const TRUE_TYPE: &[u8] = b"TrueType";
const CID_FONT_TYPE_0: &[u8] = b"CIDFontType0";
const CID_FONT_TYPE_2: &[u8] = b"CIDFontType2";
const DESCENDANT_FONTS: &[u8] = b"DescendantFonts";
const BASE_FONT: &[u8] = b"BaseFont";
const FIRST_CHAR: &[u8] = b"FirstChar";
const LAST_CHAR: &[u8] = b"LastChar";
const WIDTHS: &[u8] = b"Widths";
const CONTENTS: &[u8] = b"Contents";
const ANNOTS: &[u8] = b"Annots";
const SUB_TYPE: &[u8] = b"Subtype";
const LENGTH: &[u8] = b"Length";
const LENGTH_1: &[u8] = b"Length1";
const LENGTH_2: &[u8] = b"Length2";
const LENGTH_3: &[u8] = b"Length3";
const RECTANGLE: &[u8] = b"Rect";
const CATALOG: &[u8] = b"Catalog";
const ANNOTS_FLAGS: &[u8] = b"F";
const ROTATE: &[u8] = b"Rotate";
const CID_SYSTEM_INFO: &[u8] = b"CIDSystemInfo";
const REGISTRY: &[u8] = b"Registry";
const ORDERING: &[u8] = b"Ordering";
const SUPPLEMENT: &[u8] = b"Supplement";
const DEFAULT_WIDTH: &[u8] = b"DW";
const DEFAULT_VERTICAL_WIDTH: &[u8] = b"DW2";
const GLYPH_WIDTHS: &[u8] = b"W";
const VERTICAL_GLYPH_WIDTHS: &[u8] = b"W2";
const CID_TO_GID_MAP: &[u8] = b"CIDToGIDMap";

pub type ObjectMap = BTreeMap<IndirectReference, Object>;

pub trait ReferenceResolver<'a, W, K>
where
    W: TryInto<K>,
{
    fn follow_till(&'a self, object: Option<W>) -> Option<K>;
}

impl<'a, K> ReferenceResolver<'a, &'a Object, K> for ObjectMap
where
    K: TryFrom<&'a Object>,
{
    fn follow_till(&'a self, object: Option<&'a Object>) -> Option<K> {
        if let Ok(x) = object?.try_into() {
            Some(x)
        } else if let ObjectKind::IndirectReference(r) = &object?.kind {
            self.follow_till(self.get(r))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct PDFDocument {
    version: Header,
    trailer: Trailer,
    xref_table: XrefSection,
    start_xref: u64,
    object_map: ObjectMap,
}

impl<'a> PDFDocument {
    pub fn get(&'a self, key: &IndirectReference) -> Option<&'a Object> {
        self.object_map.get(key)
    }

    fn catalog(&'a self) -> Option<&'a Dictionary> {
        inner!(self.get(&self.trailer.root)?, ObjectKind::Dictionary)
    }

    fn page_refs(&'a self, cur: &'a IndirectReference, refs: &'a mut Vec<IndirectReference>) {
        let Some(dict): Option<&'a Dictionary> = self.get(cur).and_then(|obj| obj.try_into().ok()) else {
            return;
        };
        let Some(name): Option<&'a Name> = dict.get(TYPE).and_then(|obj| obj.try_into().ok()) else {
            return;
        };
        if name == &PAGE {
            refs.push(*cur);
        } else if name == &PAGE_ROOT {
            let Some(Object {kind: ObjectKind::Array(kids), ..}) = dict.get(KIDS) else {
                return;
            };
            for kid in kids {
                if let Some(r#ref) = inner!(kid, ObjectKind::IndirectReference) {
                    self.page_refs(r#ref, refs);
                }
            }
        }
    }

    fn get_inherited_page_key(
        &'a self,
        r#ref: &IndirectReference,
        key: &'a [u8],
    ) -> Option<&'a Object> {
        let page_object = inner!(self.get(r#ref)?, ObjectKind::Dictionary)?;
        let value = page_object.get(key);
        if value.is_some() {
            return value;
        }
        if let Object {
            kind: ObjectKind::IndirectReference(parent),
            ..
        } = page_object.get(PARENT)?
        {
            self.get_inherited_page_key(parent, key)
        } else {
            None
        }
    }

    fn content_streams(&'a self, object: &'a Object, streams: &mut Vec<&'a Stream>) {
        match object {
            Object {
                kind: ObjectKind::IndirectReference(r#ref),
                ..
            } => {
                if let Some(object) = self.get(r#ref) {
                    self.content_streams(object, streams)
                }
            }
            Object {
                kind: ObjectKind::Stream(stream),
                ..
            } => streams.push(stream),
            Object {
                kind: ObjectKind::Array(array),
                ..
            } => {
                for object in array {
                    self.content_streams(object, streams)
                }
            }
            _ => (),
        }
    }

    fn annotation(&'a self, object: &'a Object) -> Option<Annotation<'a>> {
        let annotation_dict: &Dictionary = self.object_map.follow_till(Some(object))?;
        let subtype = inner!(annotation_dict.get(SUB_TYPE)?, ObjectKind::Name)?;
        let rect = Rectangle::try_from(annotation_dict.get(RECTANGLE)?).ok()?;
        let flags = match annotation_dict.get(ANNOTS_FLAGS) {
            Some(Object {
                kind: ObjectKind::Integer(i),
                ..
            }) => AnnotationFlags::new(*i as u32),
            _ => AnnotationFlags::default(),
        };
        Some(Annotation {
            subtype,
            rect,
            flags,
        })
    }

    fn object_array_to_array<T>(&'a self, object: &'a Object) -> Option<Vec<T>>
    where
        T: TryFrom<&'a Object>,
    {
        let array = match object {
            Object {
                kind: ObjectKind::IndirectReference(r#ref),
                ..
            } => inner!(self.get(r#ref)?, ObjectKind::Array)?,
            Object {
                kind: ObjectKind::Array(array),
                ..
            } => array,
            _ => return None,
        };
        Some(
            array
                .iter()
                .filter_map(|obj| obj.try_into().ok())
                .collect::<Vec<T>>(),
        )
    }

    fn page(&'a self, r#ref: IndirectReference) -> Option<Page<'a>> {
        let page_object = inner!(self.get(&r#ref)?, ObjectKind::Dictionary)?;
        let parent = *inner!(page_object.get(PARENT)?, ObjectKind::IndirectReference)?;
        let media_box = self.get_inherited_page_key(&r#ref, MEDIA_BOX)?;
        let media_box = Rectangle::try_from(media_box).ok()?;
        // TODO: Clarify this behavior here. Crop box is inheritable and optional
        // so do we inherit the value of crop box from parent if present or do we
        // simply copy the value of media box if crop box is not set on this page
        // itself?
        let crop_box = page_object.get(CROP_BOX).map_or_else(
            || media_box,
            |x| Rectangle::try_from(x).unwrap_or(media_box),
        );
        let bleed_box = page_object
            .get(BLEED_BOX)
            .map_or_else(|| crop_box, |x| Rectangle::try_from(x).unwrap_or(crop_box));
        let trim_box = page_object
            .get(TRIM_BOX)
            .map_or_else(|| crop_box, |x| Rectangle::try_from(x).unwrap_or(crop_box));
        let art_box = page_object
            .get(ART_BOX)
            .map_or_else(|| crop_box, |x| Rectangle::try_from(x).unwrap_or(crop_box));
        let resources = self
            .object_map
            .follow_till(self.get_inherited_page_key(&r#ref, RESOURCES))
            .and_then(|dict: &Dictionary| Resources::try_from((dict, &self.object_map)).ok())?;
        let contents = match page_object.get(CONTENTS) {
            Some(object) => {
                let mut streams = vec![];
                self.content_streams(object, &mut streams);
                Some(streams)
            }
            _ => None,
        };
        let annotations = match page_object.get(ANNOTS) {
            Some(Object {
                kind: ObjectKind::Array(array),
                ..
            }) => Some(
                array
                    .iter()
                    .filter_map(|obj| self.annotation(obj))
                    .collect(),
            ),
            _ => None,
        };
        let rotate = match page_object.get(ROTATE) {
            Some(Object {
                kind: ObjectKind::Integer(i),
                ..
            }) if *i % 90 == 0 => i.unsigned_abs() as u32 % 360,
            _ => 0,
        };
        let page = Page {
            r#ref,
            parent,
            media_box,
            crop_box,
            bleed_box,
            trim_box,
            art_box,
            resources,
            contents,
            annotations,
            rotate,
            canvas: Canvas::new(),
        };
        Some(page)
    }

    pub(crate) fn pages(&'a self) -> Option<Vec<Page<'a>>> {
        let catalog = self.catalog()?;
        let page_root = catalog.get(PAGE_ROOT)?.try_into().ok()?;
        let mut refs = vec![];
        self.page_refs(page_root, &mut refs);
        let pages = refs.into_iter().filter_map(|r| self.page(r)).collect();
        Some(pages)
    }
}

impl TryFrom<PathBuf> for PDFDocument {
    type Error = PdfError;
    fn try_from(pathbuf: PathBuf) -> Result<PDFDocument> {
        let file = fs::read(pathbuf).unwrap();
        let tokens = Lexer::new(&file).lex();
        let mut parser = Parser::new(&tokens);
        let objects = parser.parse();
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
                ObjectKind::Header(header) => version = Ok(header),
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
mod tests {
    use super::*;
    use crate::document::resources::FONT;
    use crate::parser::object::Stream;
    use crate::{array, dict, indirect_reference, inner, integer, name, offset, stream};
    use std::path::PathBuf;

    const F0: &[u8] = b"F0";

    macro_rules! get {
        ($dict:expr, $key:expr) => {
            $dict.get(&$key).unwrap()
        };
    }

    #[test]
    fn test_document_hello_catalog() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("test_data/hello.pdf");
        let doc = PDFDocument::try_from(file).unwrap();
        let catalog = doc.catalog().unwrap();
        assert_eq!(*get!(catalog, TYPE.to_vec()), name!("Catalog"));
        assert_eq!(*get!(catalog, PAGE_ROOT.to_vec()), indirect_reference!(2));
    }

    #[test]
    fn test_document_hello_pages() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("test_data/hello.pdf");
        let doc = PDFDocument::try_from(file).unwrap();
        let pages = doc.pages().unwrap();
        let expected = vec![IndirectReference {
            object_number: 3,
            generation_number: 0,
        }];
        assert_eq!(
            pages
                .iter()
                .map(|page| page.r#ref)
                .collect::<Vec<IndirectReference>>(),
            expected
        );
    }

    #[test]
    fn test_document_curtiss_page_refs() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("test_data/curtiss.pdf");
        let doc = PDFDocument::try_from(file).unwrap();
        let pages = doc
            .pages()
            .unwrap()
            .iter()
            .map(|page| page.r#ref.object_number)
            .collect::<Vec<u32>>();
        let expected = vec![
            2, 15, 20, 24, 28, 32, 36, 41, 46, 51, 55, 59, 63, 67, 71, 75, 79, 84, 88, 92, 96, 100,
            105, 109, 113, 118, 122, 126, 131, 135, 139, 143,
        ];
        assert_eq!(pages, expected);
    }

    #[test]
    fn test_document_test() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("test_data/pages_salam.pdf");
        let doc = PDFDocument::try_from(file).unwrap();
        let pages = doc.pages().unwrap();
        for mut page in pages {
            page.process_operations();
        }
        assert!(false);
    }

    #[test]
    fn test_hello() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("test_data/hello.pdf");
        let doc = PDFDocument::try_from(file).unwrap();
        assert_eq!(doc.version, Header { major: 1, minor: 4 });
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
            inner!(&expected_trailer, ObjectKind::Dictionary)
                .expect("Expected trailer is not a dictionary.")
        );
        assert_eq!(doc.trailer.size, 8);
        let catalog = inner!(&get!(doc, doc.trailer.root), ObjectKind::Dictionary)
            .expect("Catalog is not a dictionary");
        assert!(
            matches!(&get!(catalog, TYPE.to_vec()).kind, ObjectKind::Name(x) if *x == CATALOG.to_vec())
        );
        let pages: &IndirectReference = catalog
            .get(PAGE_ROOT)
            .unwrap()
            .try_into()
            .expect("Catalog's pages is not indirect reference.");
        let pages = get!(doc, pages);
        let expected_pages = dict!(
            TYPE => name!("Pages"),
            KIDS => array!(indirect_reference!(3)),
            b"Count" => integer!(1)
        );
        assert_eq!(pages, &expected_pages);
        let pages = inner!(&pages, ObjectKind::Dictionary).expect("Pages is not a dictionary.");
        let kids: &Vec<Object> = pages
            .get(KIDS)
            .unwrap()
            .try_into()
            .expect("Pages' kids is not array.");
        let kids = inner!(kids[0], ObjectKind::IndirectReference)
            .expect("Kids is not indirect reference.");
        let kids = get!(doc, kids);
        let expected_kids = dict!(
            TYPE => name!("Page"),
            MEDIA_BOX => array!(integer!(0), integer!(0), integer!(612), integer!(792)),
            PARENT => indirect_reference!(2),
            b"Resources" => indirect_reference!(4),
            CONTENTS => indirect_reference!(5)
        );
        assert_eq!(kids, &expected_kids);
        let kids = inner!(&kids, ObjectKind::Dictionary).expect("Kids is not dictionary.");
        let resources_ref = kids.get(RESOURCES).unwrap();
        assert_eq!(resources_ref, &indirect_reference!(4));
        let resources_ref = inner!(resources_ref, ObjectKind::IndirectReference)
            .expect("Kids' resources field is not indirect reference.");
        let resources = get!(doc, resources_ref);
        let expected_resources = dict!(
            b"ProcSet" => array!(name!("PDF")),
            b"Font" => indirect_reference!(6)
        );
        assert_eq!(resources, &expected_resources);
        let resources =
            inner!(&resources, ObjectKind::Dictionary).expect("Resources is not a dictionary");
        let font_ref = resources.get(FONT).unwrap();
        assert_eq!(font_ref, &indirect_reference!(6));
        let font_ref = inner!(font_ref, ObjectKind::IndirectReference)
            .expect("Resources' font field is not indirect reference.");
        let font = get!(doc, font_ref);
        let expected_font = dict!(
            b"F0" => indirect_reference!(8)
        );
        assert_eq!(font, &expected_font);
        let font = inner!(&font, ObjectKind::Dictionary).expect("Font is not a dictionary.");
        let helvetica_ref = font.get(F0).unwrap();
        assert_eq!(helvetica_ref, &indirect_reference!(8));
        let helvetica_ref = inner!(helvetica_ref, ObjectKind::IndirectReference)
            .expect("F0 is not indirect reference.");
        let helvetica = get!(doc, helvetica_ref);
        let expected_helvetica = dict!(
            TYPE => name!("Font"),
            b"Subtype" => name!("Type1"),
            b"BaseFont" => name!("Helvetica")
        );
        assert_eq!(helvetica, &expected_helvetica);
        let contents = kids.get(CONTENTS).unwrap();
        assert_eq!(contents, &indirect_reference!(5));
        let contents = inner!(contents, ObjectKind::IndirectReference)
            .expect("Kids' contents field is not indirect reference.");
        let contents = get!(doc, contents);
        let expected_content = b"BT\n/F0 12 Tf\n100 700 Td\n(Hello, World) Tj\nET\n";
        let expected_contents = stream!(
            expected_content.to_vec(),
            LENGTH => indirect_reference!(7)
        );
        assert_eq!(contents, &expected_contents);
        let contents = inner!(&contents, ObjectKind::Stream).expect("Contents is not a stream.");
        let length = contents.dict.get(LENGTH).unwrap();
        assert_eq!(length, &indirect_reference!(7));
        let length = inner!(length, ObjectKind::IndirectReference)
            .expect("Contents' length is not indirect reference.");
        let length_object = get!(doc, length);
        let expected_length_object = integer!(51);
        assert_eq!(length_object, &expected_length_object);
    }
}
