use crate::parser::parser::Dictionary;

// PDF 9.5 Table 108
pub enum Type1SubtypeKind {
    Type1,
    MMType1
}

// PDF 9.5 Table 108
pub enum CIDFontSubtypeKind {
    CIDFontType0,
    CIDFontType2
}

// PDF 9.5 Table 108
pub enum FontKind {
    // Type0 fonts are also called "composite fonts." All others are "simple"
    Type0,
    Type1(Type1SubtypeKind),
    Type3,
    TrueType,
    CIDFont(CIDFontSubtypeKind)
}

// PDF 9.5
pub struct Font<'a> {
    kind: FontKind,
    descriptor: Option<&'a Dictionary>
}