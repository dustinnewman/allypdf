use std::{
    collections::BTreeMap,
    convert::{TryFrom, TryInto},
    fs,
    path::PathBuf,
};

use super::annotation::{Annotation, AnnotationFlags};
use super::page::{Page, ProcSet, Resources};
use crate::cmaps::cmap::CMap;
use crate::error::{PdfError, Result};
use crate::font::encoding::Encoding;
use crate::font::font::{
    CIDFont, CIDFontSubtypeKind, CidSystemInfo, Font, FontDictionary, TrueTypeFont, Type0Encoding,
    Type0Font, Type1Font, Type1SubtypeKind, Type3Font,
};
use crate::font::font_descriptor::FontDescriptor;
use crate::font::glyph_width::object_array_to_glyph_widths;
use crate::inner;
use crate::operators::{matrix::Matrix, rect::Rectangle};
use crate::parser::lexer::{Header, Lexer};
use crate::parser::object::{
    Dictionary, IndirectReference, Name, Object, ObjectKind, Stream, Trailer, XrefSection,
};
use crate::parser::parser::Parser;

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
const EXT_G_STATE: &[u8] = b"ExtGState";
const COLOR_SPACE: &[u8] = b"ColorSpace";
const PATTERN: &[u8] = b"Pattern";
const SHADING: &[u8] = b"Shading";
const X_OBJECT: &[u8] = b"XObject";
const FONT: &[u8] = b"Font";
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
const PROC_SET: &[u8] = b"ProcSet";
const PROPERTIES: &[u8] = b"Properties";
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

    fn cid_font(&'a self, dict: &'a Dictionary) -> Option<CIDFont<'a>> {
        let subtype = self
            .object_map
            .follow_till(dict.get(SUB_TYPE))
            .and_then(|name: &Name| CIDFontSubtypeKind::try_from(name.0.as_ref()).ok())?;
        let font_descriptor = self
            .object_map
            .follow_till(dict.get(FONT_DESCRIPTOR))
            .and_then(|obj| FontDescriptor::try_from((obj, &self.object_map)).ok())?;
        let base_font = inner!(dict.get(BASE_FONT)?, ObjectKind::Name)?;
        let cid_system_info: &Dictionary =
            self.object_map.follow_till(dict.get(CID_SYSTEM_INFO))?;
        let registry = inner!(cid_system_info.get(REGISTRY)?, ObjectKind::String)?;
        let ordering = inner!(cid_system_info.get(ORDERING)?, ObjectKind::String)?;
        let supplement = *inner!(cid_system_info.get(SUPPLEMENT)?, ObjectKind::Integer)? as u32;
        let cid_system_info = CidSystemInfo {
            registry: registry.into(),
            ordering: ordering.into(),
            supplement,
        };
        let default_width = match dict.get(DEFAULT_WIDTH) {
            Some(Object {
                kind: ObjectKind::Integer(i),
                ..
            }) => *i as f64,
            Some(Object {
                kind: ObjectKind::Real(r),
                ..
            }) => *r,
            _ => 1000.00,
        };
        let vertical_default_width: (f64, f64) = dict
            .get(DEFAULT_VERTICAL_WIDTH)
            .and_then(|array| self.object_array_to_array(array))
            .map_or((880.00, -1000.00), |array| (array[0], array[1]));
        let widths = dict
            .get(GLYPH_WIDTHS)
            .and_then(|x| inner!(x, ObjectKind::Array))
            .map_or(vec![], object_array_to_glyph_widths);
        let vertical_widths = dict
            .get(VERTICAL_GLYPH_WIDTHS)
            .and_then(|x| inner!(x, ObjectKind::Array))
            .map(object_array_to_glyph_widths);
        let cid_to_gid_map = dict.get(CID_TO_GID_MAP)?.try_into().ok()?;
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
        Some(cid_font)
    }

    fn composite_font(&'a self, dict: &'a Dictionary) -> Option<Type0Font<'a>> {
        let base_font = inner!(dict.get(BASE_FONT)?, ObjectKind::Name)?;
        let descendant_fonts = self.cid_font(
            self.object_map
                .follow_till(inner!(dict.get(DESCENDANT_FONTS)?, ObjectKind::Array)?.get(0))?,
        )?;
        let encoding: Type0Encoding =
            Type0Encoding::try_from((dict.get(ENCODING), &self.object_map)).ok()?;
        let to_unicode = self
            .object_map
            .follow_till(dict.get(TO_UNICODE))
            .and_then(|stream: &Stream| TryInto::<CMap>::try_into(stream).ok());
        Some(Type0Font::new(
            base_font,
            encoding,
            descendant_fonts,
            to_unicode,
        ))
    }

    fn type_1_font(
        &'a self,
        dict: &'a Dictionary,
        subtype: Type1SubtypeKind,
    ) -> Option<Type1Font<'a>> {
        let name = self.object_map.follow_till(dict.get(NAME));
        let base_font = self.object_map.follow_till(dict.get(BASE_FONT))?;
        let first_char = self.object_map.follow_till(dict.get(FIRST_CHAR));
        let last_char = self.object_map.follow_till(dict.get(LAST_CHAR));
        let src_widths: Option<Vec<f64>> = dict
            .get(WIDTHS)
            .and_then(|obj| self.object_array_to_array::<f64>(obj));
        let font_descriptor = self
            .object_map
            .follow_till(dict.get(FONT_DESCRIPTOR))
            .and_then(|obj| FontDescriptor::try_from((obj, &self.object_map)).ok());
        let widths = if let (Some(first_char), Some(last_char), Some(src_widths)) =
            (first_char, last_char, src_widths)
        {
            let mut dst_widths = [font_descriptor.as_ref().map_or(0., |fd| fd.missing_width); 256];
            dst_widths[(first_char as usize)..(last_char as usize + 1)]
                .clone_from_slice(&src_widths);
            Some(dst_widths)
        } else {
            None
        };
        let encoding = self
            .object_map
            .follow_till(dict.get(ENCODING))
            .and_then(|obj: &Object| Encoding::try_from(obj).ok());
        let to_unicode = dict
            .get(TO_UNICODE)
            .and_then(|obj| inner!(obj, ObjectKind::Stream));
        Some(Type1Font::new(
            subtype,
            name,
            base_font,
            first_char,
            last_char,
            widths,
            font_descriptor,
            encoding,
            to_unicode,
        ))
    }

    fn type_3_font(&'a self, dict: &'a Dictionary) -> Option<Type3Font<'a>> {
        let name = self.object_map.follow_till(dict.get(NAME));
        let resources = self
            .object_map
            .follow_till(dict.get(RESOURCES))
            .map(|dict| self.resources(dict));
        let font_b_box = Rectangle::try_from(dict.get(FONT_B_BOX)?).ok()?;
        let font_matrix = inner!(dict.get(FONT_MATRIX)?, ObjectKind::Array)?
            .iter()
            .filter_map(|obj| f64::try_from(obj).ok())
            .collect::<Vec<f64>>();
        let font_matrix: [f64; 6] = font_matrix.try_into().ok()?;
        let font_matrix = Matrix::new(font_matrix);
        let char_procs = self.object_map.follow_till(dict.get(CHAR_PROCS))?;
        let first_char = self.object_map.follow_till(dict.get(FIRST_CHAR))?;
        let last_char = self.object_map.follow_till(dict.get(LAST_CHAR))?;
        let src_widths: Vec<f64> = self.object_array_to_array::<f64>(dict.get(WIDTHS)?)?;
        let src_widths: &[f64] = &src_widths[0..((last_char - first_char) as usize)];
        let font_descriptor = self
            .object_map
            .follow_till(dict.get(FONT_DESCRIPTOR))
            .and_then(|obj| FontDescriptor::try_from((obj, &self.object_map)).ok())?;
        let mut widths = [font_descriptor.missing_width; 256];
        widths[(first_char as usize)..(last_char as usize + 1)].clone_from_slice(src_widths);
        let encoding = self
            .object_map
            .follow_till(dict.get(ENCODING))
            .and_then(|obj: &Object| Encoding::try_from(obj).ok())?;
        let to_unicode = dict
            .get(TO_UNICODE)
            .and_then(|obj| inner!(obj, ObjectKind::Stream));
        Some(Type3Font::new(
            name,
            font_b_box,
            font_matrix,
            char_procs,
            encoding,
            first_char,
            last_char,
            widths,
            font_descriptor,
            resources,
            to_unicode,
        ))
    }

    fn true_type_font(&'a self, dict: &'a Dictionary) -> Option<TrueTypeFont<'a>> {
        let name = self.object_map.follow_till(dict.get(NAME));
        let base_font = self.object_map.follow_till(dict.get(BASE_FONT))?;
        let first_char = self.object_map.follow_till(dict.get(FIRST_CHAR));
        let last_char = self.object_map.follow_till(dict.get(LAST_CHAR));
        let src_widths: Option<Vec<f64>> = dict
            .get(WIDTHS)
            .and_then(|obj| self.object_array_to_array::<f64>(obj));
        let font_descriptor = self
            .object_map
            .follow_till(dict.get(FONT_DESCRIPTOR))
            .and_then(|obj| FontDescriptor::try_from((obj, &self.object_map)).ok());
        let widths = if let (Some(first_char), Some(last_char), Some(src_widths)) =
            (first_char, last_char, src_widths)
        {
            let mut dst_widths = [font_descriptor.as_ref().map_or(0., |fd| fd.missing_width); 256];
            dst_widths[(first_char as usize)..(last_char as usize + 1)]
                .clone_from_slice(&src_widths);
            Some(dst_widths)
        } else {
            None
        };
        // A font that is used to display glyphs that do not use
        // MacRomanEncoding or WinAnsiEncoding should not specify an
        // Encoding entry. - PDF 9.6.5.4 paragraph 4 item 3
        let encoding = self
            .object_map
            .follow_till(dict.get(ENCODING))
            .and_then(|obj: &Object| Encoding::try_from(obj).ok());
        let to_unicode = self
            .object_map
            .follow_till(dict.get(TO_UNICODE))
            .and_then(|stream: &Stream| CMap::try_from(stream).ok());
        Some(TrueTypeFont::new(
            name,
            base_font,
            first_char,
            last_char,
            widths,
            font_descriptor,
            encoding,
            to_unicode,
        ))
    }

    fn font(&'a self, dict: &'a Dictionary) -> Option<Font<'a>> {
        let name = inner!(dict.get(SUB_TYPE)?, ObjectKind::Name)?;
        if name == &TYPE_0 {
            Some(Font::Type0(self.composite_font(dict)?))
        } else if name == &TYPE_1 {
            Some(Font::Type1(
                self.type_1_font(dict, Type1SubtypeKind::Type1)?,
            ))
        } else if name == &TYPE_1_MM {
            Some(Font::Type1(
                self.type_1_font(dict, Type1SubtypeKind::MMType1)?,
            ))
        } else if name == &TYPE_3 {
            Some(Font::Type3(self.type_3_font(dict)?))
        } else if name == &TRUE_TYPE {
            Some(Font::TrueType(self.true_type_font(dict)?))
        } else {
            None
        }
    }

    fn font_dictionary(&'a self, dict: &'a Dictionary) -> FontDictionary<'a> {
        let mut font_dictionary = FontDictionary::new();
        for (name, object) in &dict.0 {
            let name = Name(name.clone());
            if let Some(dict) = self.object_map.follow_till(Some(object)) {
                if let Some(font) = self.font(dict) {
                    font_dictionary.insert(std::borrow::Cow::Owned(name), font);
                }
            }
        }
        font_dictionary
    }

    fn resources(&'a self, dict: &'a Dictionary) -> Resources<'a> {
        let ext_g_state = self.object_map.follow_till(dict.get(EXT_G_STATE));
        let color_space = self.object_map.follow_till(dict.get(COLOR_SPACE));
        let pattern = self.object_map.follow_till(dict.get(PATTERN));
        let shading = self.object_map.follow_till(dict.get(SHADING));
        let x_object = self.object_map.follow_till(dict.get(X_OBJECT));
        let font = self
            .object_map
            .follow_till(dict.get(FONT))
            .map(|dict| self.font_dictionary(dict));
        let proc_set = dict
            .get(PROC_SET)
            .and_then(|obj| inner!(obj, ObjectKind::Array))
            .map(|vec| {
                vec.iter()
                    .filter_map(|obj| ProcSet::try_from(obj).ok())
                    .collect()
            });
        let properties = self.object_map.follow_till(dict.get(PROPERTIES));
        Resources {
            ext_g_state,
            color_space,
            pattern,
            shading,
            x_object,
            font,
            proc_set,
            properties,
        }
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
            .follow_till(self.get_inherited_page_key(&r#ref, RESOURCES))?;
        let resources = self.resources(resources);
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
        let page = Page::new(
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
        );
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
