use std::io;

pub type Result<T> = std::result::Result<T, PdfError>;

#[derive(Debug)]
pub enum PdfError {
    Eof,
    FileStart,
    NoHeader,
    NoTrailer,
    NoCrossReferences,
    NoStartXref,
    ObjectNotDictionary,
    ObjectNotInteger,
    ObjectNotStream,
    ObjectNotName,
    ObjectNotArray,
    ObjectNotIndirectReference,
    CMapParsingError,
    InvalidFilterName,
    InvalidProcSet,
    InvalidFontWeight,
    InvalidFontStretch,
    InvalidDefaultEncodingName,
    InvalidCIDFontSubtypeKind,
    InvalidCIDToGIDMap,
    InvalidType0EncodingName,
    RectangleParsingError,
    ParseF64Error,
    ParseU32Error,
    NoFontDescriptor,
    FontDescriptorMissingFontName,
    FontDescriptorMissingFlags,
    FontDescriptorMissingItalicAngle,
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
