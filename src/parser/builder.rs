use std::collections::BTreeMap;
use crate::document::document::{ObjectMap, PDFDocument, Page, Rectangle, Version};
use super::parser::{Dictionary, IndirectReference, Object, Trailer, XrefSection};

const TYPE: &[u8] = b"Type";
const PAGE_ROOT: &[u8] = b"Pages";
const KIDS: &[u8] = b"Kids";
const PAGE: &[u8] = b"Page";
const MEDIA_BOX: &[u8] = b"MediaBox";

pub struct Builder {
    version: Version,
    trailer: Trailer,
    xref_table: XrefSection,
    start_xref: u64,
    object_map: ObjectMap,
}

macro_rules! get_or_none {
    ($obj:expr, $variant:path) => {
        match $obj? {
            $variant(x) => x,
            _ => return None,
        }
    };
}

macro_rules! get_or_continue {
    ($obj:expr, $variant:path) => {
        match $obj {
            Some($variant(x)) => x,
            _ => continue,
        }
    };
}

macro_rules! get_f32_or_none {
    ($obj:expr) => {
        match $obj {
            Object::Integer(x) => *x as f32,
            Object::Real(x) => *x as f32,
            _ => return None,
        }
    };
}

impl Builder {
    pub fn new(version: Version, trailer: Trailer, xref_table: XrefSection, start_xref: u64, object_map: ObjectMap) -> Self {
        Self {
            version,
            trailer,
            xref_table,
            start_xref,
            object_map
        }
    }

    pub fn build(&mut self) -> Option<PDFDocument<'_>> {
        let catalog = get_or_none!(self.get_root(), Object::Dictionary);
        let page_root_key = get_or_none!(catalog.get(PAGE_ROOT), Object::IndirectReference);
        let page_root = get_or_none!(self.get(page_root_key), Object::Dictionary);
        let kid_references = get_or_none!(page_root.get(KIDS), Object::Array);
        let kids = self.build_pages(kid_references);
        None
    }

    fn build_pages(&self, root_references: &Vec<Object>) -> Vec<Page> {
        let mut pages = vec![];
        for reference in root_references {
            let reference = match reference {
                Object::IndirectReference(x) => x,
                _ => continue,
            };
            let object = get_or_continue!(self.get(reference), Object::Dictionary);
            let object_type = get_or_continue!(object.get(TYPE), Object::Name);
            if object_type == PAGE_ROOT {
                let kid_references = get_or_continue!(object.get(KIDS), Object::Array);
                let mut kids = self.build_pages(kid_references);
                pages.append(&mut kids);
            } else if object_type == PAGE {
                let page = self.build_page(object);
                if let Some(page) = page {
                    pages.push(page);
                }
            } else {
                continue
            }
        }
        pages
    }

    fn build_page(&self, dict: &Dictionary) -> Option<Page> {
        let media_box = get_or_none!(dict.get(MEDIA_BOX), Object::Array);
        let media_box = self.rectangle(media_box)?;
        None
    }

    fn rectangle(&self, arr: &Vec<Object>) -> Option<Rectangle> {
        let lower_left_x: f32 = get_f32_or_none!(arr.get(0)?);
        let lower_left_y: f32 = get_f32_or_none!(arr.get(1)?);
        let upper_right_x: f32 = get_f32_or_none!(arr.get(2)?);
        let upper_right_y: f32 = get_f32_or_none!(arr.get(3)?);
        Some(Rectangle {
            lower_left_x,
            lower_left_y,
            upper_right_x,
            upper_right_y
        })
    }

    fn get_root(&self) -> Option<&Object> {
        let root_key = self.trailer.root;
        self.object_map.get(&root_key)
    }

    fn get(&self, key: &IndirectReference) -> Option<&Object> {
        self.object_map.get(key)
    }
}

pub fn new_builder(objects: Vec<Object>) -> Option<Builder> {
    let mut version: Option<Version> = None;
    let mut trailer: Option<Trailer> = None;
    let mut xref_section: Option<XrefSection> = None;
    let mut start_xref = None;
    let mut object_map: ObjectMap = BTreeMap::new();
    for obj in objects {
        match obj {
            Object::Header(m, n) => version = Some((m, n)),
            Object::Trailer(t) => trailer = Some(t),
            Object::StartXref(s) => start_xref = Some(s),
            Object::IndirectObject(ind) => {
                let key = IndirectReference {
                    object_number: ind.object_number,
                    generation_number: ind.generation_number,
                };
                object_map.insert(key, *ind.object);
            },
            Object::Xref(x) => xref_section = Some(x),
            _ => continue,
        }
    }
    Some(Builder {
        version: version?,
        trailer: trailer?,
        xref_table: xref_section?,
        start_xref: start_xref?,
        object_map,
    })
}