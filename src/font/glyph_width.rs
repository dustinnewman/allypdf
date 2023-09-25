use std::convert::{TryFrom, TryInto};

use crate::cmaps::cid::Cid;
use crate::parser::object::{Object, ObjectKind};

#[derive(Debug)]
pub enum GlyphWidth {
    Consecutive(Cid, Vec<f64>),
    Range(Cid, Cid, f64),
}

// This is perhaps the ugliest function I have ever written
pub fn object_array_to_glyph_widths(array: &[Object]) -> Vec<GlyphWidth> {
    let mut glyph_widths = vec![];
    let mut iter = array.iter();
    while let Some(first) = iter.next() {
        let Ok(first) = u32::try_from(first) else {
            break
        };
        match iter.next() {
            Some(num) if u32::try_from(num).is_ok() => {
                let end = u32::try_from(num).unwrap();
                let Some(width) = iter.next() else {
                    break
                };
                let Ok(width) = f64::try_from(width) else {
                    break
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
