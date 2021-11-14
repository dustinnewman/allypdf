use std::collections::BTreeMap;

use crate::parser::parser::{Dictionary, IndirectReference, Name, Object, Trailer, XrefSection};

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

struct Rectangle {
    lower_left_x: u32,
    lower_left_y: u32,
    upper_right_x: u32,
    upper_right_y: u32,
}

pub struct Page<'a> {
    parent: IndirectReference,
    media_box: Rectangle,
    crop_box: Option<Rectangle>,
    bleed_box: Option<Rectangle>,
    trim_box: Option<Rectangle>,
    art_box: Option<Rectangle>,
    contents: Vec<Object>,
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

pub type Version = (u8,u8);
pub type ObjectMap = BTreeMap<IndirectReference, Object>;

pub struct PDFDocument<'a> {
    // (major, minor)
    version: (u8, u8),
    trailer: Trailer,
    xref_table: XrefSection,
    start_xref: u64,
    object_map: ObjectMap,
    catalog: Catalog<'a>,
}
