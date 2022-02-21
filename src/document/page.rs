use std::convert::TryFrom;

use crate::{
    error::PdfError,
    parser::parser::{Dictionary, IndirectReference, Name, Object, ObjectKind, Stream},
};

use super::document::PDFDocument;

// Default page size is letter: 8.5" x 11" = 612pt x 792pt
const LETTER_PAGE: Rectangle = Rectangle {
    lower_left_x: 0.0,
    lower_left_y: 0.0,
    upper_right_x: 612.0,
    upper_right_y: 792.0,
};

const A4_PAGE: Rectangle = Rectangle {
    lower_left_x: 0.0,
    lower_left_y: 0.0,
    upper_right_x: 595.0,
    upper_right_y: 842.0,
};

pub struct Annotation<'a> {
    pub subtype: &'a Name,
    pub rect: Rectangle,
    pub flags: u32,
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
    pub lower_left_x: f64,
    pub lower_left_y: f64,
    pub upper_right_x: f64,
    pub upper_right_y: f64,
}

impl Default for Rectangle {
    fn default() -> Self {
        LETTER_PAGE
    }
}

impl TryFrom<&Object> for Rectangle {
    type Error = PdfError;

    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        let error = Err(PdfError::Other {
            msg: "Could not convert object to rectangle".to_string(),
        });
        match &object.kind {
            ObjectKind::Array(array) => {
                let lower_left_x = match array.get(0) {
                    Some(Object {
                        kind: ObjectKind::Integer(x),
                        ..
                    }) => *x as f64,
                    Some(Object {
                        kind: ObjectKind::Real(x),
                        ..
                    }) => *x,
                    _ => return error,
                };
                let lower_left_y = match array.get(1) {
                    Some(Object {
                        kind: ObjectKind::Integer(x),
                        ..
                    }) => *x as f64,
                    Some(Object {
                        kind: ObjectKind::Real(x),
                        ..
                    }) => *x,
                    _ => return error,
                };
                let upper_right_x = match array.get(2) {
                    Some(Object {
                        kind: ObjectKind::Integer(x),
                        ..
                    }) => *x as f64,
                    Some(Object {
                        kind: ObjectKind::Real(x),
                        ..
                    }) => *x,
                    _ => return error,
                };
                let upper_right_y = match array.get(3) {
                    Some(Object {
                        kind: ObjectKind::Integer(x),
                        ..
                    }) => *x as f64,
                    Some(Object {
                        kind: ObjectKind::Real(x),
                        ..
                    }) => *x,
                    _ => return error,
                };
                Ok(Self {
                    lower_left_x,
                    lower_left_y,
                    upper_right_x,
                    upper_right_y,
                })
            }
            _ => error,
        }
    }
}

pub struct Page<'a> {
    pub r#ref: IndirectReference,
    parent: IndirectReference,
    media_box: Rectangle,
    crop_box: Rectangle,
    bleed_box: Rectangle,
    trim_box: Rectangle,
    art_box: Rectangle,
    contents: Option<Vec<&'a Stream>>,
    annotations: Option<Vec<Annotation<'a>>>,
    resources: &'a Dictionary,
    rotate: u32,
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
        r#ref: IndirectReference,
        parent: IndirectReference,
        media_box: Rectangle,
        crop_box: Rectangle,
        bleed_box: Rectangle,
        trim_box: Rectangle,
        art_box: Rectangle,
        resources: &'a Dictionary,
        contents: Option<Vec<&'a Stream>>,
        annotations: Option<Vec<Annotation<'a>>>,
        rotate: u32,
    ) -> Self {
        Self {
            r#ref,
            parent,
            media_box,
            crop_box,
            bleed_box,
            trim_box,
            art_box,
            contents,
            annotations,
            resources,
            rotate,
        }
    }
}

impl<'a> TryFrom<(&'a PDFDocument, IndirectReference)> for Page<'a> {
    type Error = PdfError;

    fn try_from(value: (&'a PDFDocument, IndirectReference)) -> Result<Self, Self::Error> {
        todo!()
    }
}
