use std::convert::TryInto;

use crate::cmaps::cid::Cid;
use crate::parser::parser::{Object, ObjectKind};

#[derive(Debug)]
pub enum GlyphWidth {
    Consecutive(Cid, Vec<f64>),
    Range(Cid, Cid, f64),
}

// This is perhaps the ugliest function I have ever written
pub fn object_array_to_glyph_widths(array: &Vec<Object>) -> Vec<GlyphWidth> {
    let mut glyph_widths = vec![];
    let mut iter = array.iter();
    while let Some(object) = iter.next() {
        let first = match object.kind {
            ObjectKind::Integer(i) => i as Cid,
            ObjectKind::Real(r) => r as Cid,
            _ => break,
        };
        match iter.next() {
            Some(Object {
                kind: ObjectKind::Integer(i),
                ..
            }) => {
                let end = *i as Cid;
                let width = match iter.next() {
                    Some(Object {
                        kind: ObjectKind::Integer(i),
                        ..
                    }) => *i as f64,
                    Some(Object {
                        kind: ObjectKind::Real(r),
                        ..
                    }) => *r,
                    _ => break,
                };
                let glyph_width = GlyphWidth::Range(first, end, width);
                glyph_widths.push(glyph_width);
            }
            Some(Object {
                kind: ObjectKind::Real(r),
                ..
            }) => {
                let end = *r as Cid;
                let width = match iter.next() {
                    Some(Object {
                        kind: ObjectKind::Integer(i),
                        ..
                    }) => *i as f64,
                    Some(Object {
                        kind: ObjectKind::Real(r),
                        ..
                    }) => *r,
                    _ => break,
                };
                let glyph_width = GlyphWidth::Range(first, end, width);
                glyph_widths.push(glyph_width);
            }
            Some(Object {
                kind: ObjectKind::Array(widths),
                ..
            }) => {
                let widths = widths
                    .iter()
                    .filter_map(|obj| obj.try_into().ok())
                    .collect::<Vec<f64>>();
                let glyph_width = GlyphWidth::Consecutive(first, widths);
                glyph_widths.push(glyph_width);
            }
            _ => break,
        };
    }
    glyph_widths
}
