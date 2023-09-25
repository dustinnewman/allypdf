use crate::operators::rect::Rectangle;
use crate::parser::object::IndirectReference;

// PDF 12.4.3 Table 162
pub struct ThreadDictionary {
    first_bead: IndirectReference,
}

// PDF 12.4.3 Table 163
pub struct BeadDictionary {
    thread: Option<IndirectReference>,
    next: IndirectReference,
    prev: IndirectReference,
    page: IndirectReference,
    rect: Rectangle,
}
