use std::convert::TryFrom;

use crate::{
    error::PdfError,
    parser::parser::{Dictionary, IndirectReference, Name, Stream},
};

use super::document::PDFDocument;

// Default page size is letter: 8.5" x 11" = 612px x 792px
const DEFAULT_PAGE_SIZE: Rectangle = Rectangle {
    lower_left_x: 0.0,
    lower_left_y: 0.0,
    upper_right_x: 612.0,
    upper_right_y: 792.0,
};

pub struct Annotation {
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

impl Default for Rectangle {
    fn default() -> Self {
        DEFAULT_PAGE_SIZE
    }
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

impl<'a> Page<'a> {
    pub fn new(
        parent: IndirectReference,
        media_box: Rectangle,
        crop_box: Rectangle,
        bleed_box: Rectangle,
        trim_box: Rectangle,
        art_box: Rectangle,
        resources: &'a Dictionary,
        contents: Vec<&'a Stream>,
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            parent,
            media_box,
            crop_box,
            bleed_box,
            trim_box,
            art_box,
            contents,
            annotations,
            resources,
        }
    }
}

impl<'a> TryFrom<(&'a PDFDocument, IndirectReference)> for Page<'a> {
    type Error = PdfError;

    fn try_from(value: (&'a PDFDocument, IndirectReference)) -> Result<Self, Self::Error> {
        todo!()
    }
}
