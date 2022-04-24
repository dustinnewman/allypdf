use std::io;

pub type Result<T> = std::result::Result<T, PdfError>;

#[derive(Debug)]
pub enum PdfError {
    EOF,
    NoHeader,
    NoTrailer,
    NoCrossReferences,
    NoStartXref,
    // Beginning of File
    BOF,
    CMapParsingError,
    InvalidFilterName,
    InvalidProcSet,
    InvalidFontWeight,
    InvalidFontStretch,
    InvalidDefaultEncodingName,
    InvalidCIDFontSubtypeKind,
    RectangleParsingError,
    ParseF64Error,
    IO { source: io::Error },
    Other { msg: String },
}

impl From<io::Error> for PdfError {
    fn from(source: io::Error) -> PdfError {
        PdfError::IO { source }
    }
}
impl From<String> for PdfError {
    fn from(msg: String) -> PdfError {
        PdfError::Other { msg }
    }
}
