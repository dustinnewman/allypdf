use crate::{operators::rect::Rectangle, parser::parser::Name};

#[derive(Debug)]
pub struct AnnotationFlags(u32);

// Flags
const FLAG_INVISIBLE: u32 = 0;
const FLAG_HIDDEN: u32 = 1;
const FLAG_PRINTED: u32 = 2;
const FLAG_NO_ZOOM: u32 = 3;
const FLAG_NO_ROTATE: u32 = 4;
const FLAG_NO_VIEW: u32 = 5;
const FLAG_READ_ONLY: u32 = 6;
const FLAG_LOCKED: u32 = 7;
const FLAG_TOGGLE_NO_VIEW: u32 = 8;
const FLAG_LOCKED_CONTENTS: u32 = 9;

impl AnnotationFlags {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    fn n(&self, n: u32) -> bool {
        (self.0 >> n) & 1 == 1
    }

    pub fn invisible(&self) -> bool {
        self.n(FLAG_INVISIBLE)
    }

    pub fn hidden(&self) -> bool {
        self.n(FLAG_HIDDEN)
    }

    pub fn printed(&self) -> bool {
        self.n(FLAG_PRINTED)
    }

    pub fn no_zoom(&self) -> bool {
        self.n(FLAG_NO_ZOOM)
    }

    pub fn no_rotate(&self) -> bool {
        self.n(FLAG_NO_ROTATE)
    }

    pub fn no_view(&self) -> bool {
        self.n(FLAG_NO_VIEW)
    }

    pub fn read_only(&self) -> bool {
        self.n(FLAG_READ_ONLY)
    }

    pub fn locked(&self) -> bool {
        self.n(FLAG_LOCKED)
    }

    pub fn toggle_view(&self) -> bool {
        self.n(FLAG_TOGGLE_NO_VIEW)
    }

    pub fn locked_contents(&self) -> bool {
        self.n(FLAG_LOCKED_CONTENTS)
    }
}

impl Default for AnnotationFlags {
    // PDF 12.5.2 Table 166 - Default value is 0
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Debug)]
pub struct Annotation<'a> {
    pub subtype: &'a Name,
    pub rect: Rectangle,
    pub flags: AnnotationFlags,
}
