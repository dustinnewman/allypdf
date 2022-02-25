use std::convert::TryFrom;

use crate::{
    error::PdfError,
    parser::parser::{Object, ObjectKind},
};

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
