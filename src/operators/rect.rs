use std::convert::TryFrom;

use crate::error::{PdfError, Result};
use crate::parser::object::{Object, ObjectKind};

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

    fn try_from(object: &Object) -> Result<Self> {
        match &object.kind {
            ObjectKind::Array(array) => {
                let lower_left_x =
                    f64::try_from(array.get(0).ok_or(PdfError::RectangleParsingError)?)?;
                let lower_left_y =
                    f64::try_from(array.get(1).ok_or(PdfError::RectangleParsingError)?)?;
                let upper_right_x =
                    f64::try_from(array.get(2).ok_or(PdfError::RectangleParsingError)?)?;
                let upper_right_y =
                    f64::try_from(array.get(3).ok_or(PdfError::RectangleParsingError)?)?;
                Ok(Self {
                    lower_left_x,
                    lower_left_y,
                    upper_right_x,
                    upper_right_y,
                })
            }
            _ => Err(PdfError::RectangleParsingError),
        }
    }
}
