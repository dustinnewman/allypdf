use std::collections::BTreeMap;

use crate::parser::parser::{IndirectReference, Name, Object, Trailer, XrefSection};

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

struct Page {
    parent: IndirectReference,
    media_box: Rectangle,
    contents: Vec<Object>,
    annotations: Vec<Annotation>
    // resources: ResourcePool,
}

struct PagesRoot {
    kids: Vec<IndirectReference>,
    count: u64,
}

struct Catalog {
    pages: PagesRoot,
}

type ObjectMap = BTreeMap<IndirectReference, Object>;

pub struct PDFDocument {
    // (major, minor)
    version: (u8, u8),
    trailer: Trailer,
    xref_table: XrefSection,
    object_map: ObjectMap,
}
