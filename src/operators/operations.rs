use crate::parser::parser::{Dictionary, Name};

// A real number between 0 and 1 (inclusive)
pub type UnitInterval = f32;
// A real number between 0 and 100 (inclusive)
pub type Percent = f32;
// A real number
pub type UnscaledTextSpaceUnit = f64;

#[derive(Debug, PartialEq)]
pub struct RGB {
    pub red: UnitInterval,
    pub green: UnitInterval,
    pub blue: UnitInterval,
}

#[derive(Debug, PartialEq)]
pub struct CMYK {
    pub cyan: UnitInterval,
    pub magenta: UnitInterval,
    pub yellow: UnitInterval,
    pub black: UnitInterval,
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Gray(UnitInterval),
    RGB(RGB),
    CMYK(CMYK),
}

#[derive(Debug, PartialEq)]
pub enum TextRendering {
    Fill,
    Stroke,
    FillStroke,
    Invisible,
    FillAddToPath,
    StrokeAddToPath,
    FillStrokeAddToPath,
    AddTextToPath,
}

impl TextRendering {
    pub fn from_i64(value: i64) -> Option<Self> {
        let render = match value {
            0 => Self::Fill,
            1 => Self::Stroke,
            2 => Self::FillStroke,
            3 => Self::Invisible,
            4 => Self::FillAddToPath,
            5 => Self::StrokeAddToPath,
            6 => Self::FillStrokeAddToPath,
            7 => Self::AddTextToPath,
            _ => return None,
        };
        Some(render)
    }
}

#[derive(Debug, PartialEq)]
pub enum LineCap {
    Butt,
    Round,
    ProjectingSquare,
}

impl LineCap {
    pub fn from_i64(value: i64) -> Option<Self> {
        let render = match value {
            0 => Self::Butt,
            1 => Self::Round,
            2 => Self::ProjectingSquare,
            _ => return None,
        };
        Some(render)
    }
}

#[derive(Debug, PartialEq)]
pub enum LineJoin {
    MiterJoin,
    RoundJoin,
    BevelJoin,
}

impl LineJoin {
    pub fn from_i64(value: i64) -> Option<Self> {
        let render = match value {
            0 => Self::MiterJoin,
            1 => Self::RoundJoin,
            2 => Self::BevelJoin,
            _ => return None,
        };
        Some(render)
    }
}

#[derive(Debug, PartialEq)]
pub enum RenderingIntent {
    AbsoluteColorimetric,
    RelativeColorimetric,
    Saturation,
    Perceptual,
}

#[derive(Debug, PartialEq)]
pub enum StringOrNumber<'a> {
    String(&'a Vec<u8>),
    Number(f64),
}

#[derive(Debug, PartialEq)]
pub enum Operation<'a> {
    // Path operators
    CloseStrokePath, // s
    StrokePath, // S
    FillPath, // f, F (obsolete)
    FillPathEvenOdd, // f*
    CloseFillStrokePath, // b
    FillStrokePath, // B
    CloseFillStrokePathEvenOdd, // b*
    FillStrokePathEvenOdd, // B*
    EndPathNoFill, // n
    SetClippingPath, // W
    SetClippingPathEvenOdd, // W*
    // Marked content operators
    DefineMarkedContentPoint(&'a Name), // MP
    DefineMarkedContentPointPropertyList(&'a Name, &'a Dictionary), // DP
    BeginMarkedContentSequence(&'a Name), // BMC
    BeginMarkedContentSequencePropertyList(&'a Name, &'a Dictionary), // BDC
    EndMarkedContentSequence, // EMC
    // Image operators
    BeginInlineImageObject, // BI
    BeginInlineImageData, // ID
    EndInlineImage, // EI
    // Text operators
    BeginText, // BT
    ShowText(&'a Vec<u8>), // Tj
    ShowTextAdjusted(Vec<StringOrNumber<'a>>), // TJ
    MoveNextLineShowText(&'a Vec<u8>), // '
    SetSpacingMoveNextLineShowText(UnscaledTextSpaceUnit, UnscaledTextSpaceUnit, &'a Vec<u8>), // "
    MoveTextPosition(UnscaledTextSpaceUnit, UnscaledTextSpaceUnit), // Td
    MoveTextPositionLeading(UnscaledTextSpaceUnit, UnscaledTextSpaceUnit), // TD
    SetTextMatrix([f64; 6]), // Tm
    MoveStartNextLine, // T*
    SetCharSpacing(UnscaledTextSpaceUnit), // Tc
    SelectFont(&'a Name, f64), // Tf
    SetTextLeading(UnscaledTextSpaceUnit), // TL
    SetTextRendering(TextRendering), // Tr
    SetTextRise(UnscaledTextSpaceUnit), // Ts
    SetWordSpacing(UnscaledTextSpaceUnit), // Tw
    SetHorizontalTextScaling(f64), // Tz
    EndText, // ET
    BeginCompat, // BX
    // Type 3 font operators
    SetCharWidth(f64, f64), // d0
    SetCacheDevice((f64, f64), (f64, f64), (f64, f64)), // d1
    InvokeXObject(&'a Name), // Do
    EndCompat, // EX
    // Path construction operators
    MoveTo(f64, f64), // m
    LineTo(f64, f64), // l
    AppendCurveThreePoints((f64, f64), (f64, f64), (f64, f64)), // c
    AppendCurveInitialReplicated((f64, f64), (f64, f64)), // v
    AppendCurveFinalReplicated((f64, f64), (f64, f64)), // y
    AppendRectangle(f64, f64, f64, f64), // re
    CloseSubpath, // h
    SetMiterLimit(f64), // M
    ConcatMatrix([f64; 6]), // cm
    SetLineWidth(f64), // w
    SetLineJoin(LineJoin), // j
    SetLineCap(LineCap), // J
    SetDash(Vec<i64>, i64), // d
    GSave, // q
    GRestore, // Q
    SetColorRenderingIntent(&'a Name), // ri
    SetFlat(Percent), // i
    SetGraphicsStateParams(&'a Name), // gs
    // Color operators
    SetCMYKColorStroke(CMYK), // K
    SetCMYKColorNonstroke(CMYK), // k
    SetColorStroke(Color), // SC
    SetColorNonstroke(Color), // sc
    SetColorSpecialStroke(Color, Option<&'a Name>), // SCN
    SetColorSpecialNonstroke(Color, Option<&'a Name>), // scn
    SetColorSpaceStroke(&'a Name), // CS
    SetColorSpaceNonstroke(&'a Name), // cs
    SetRGBColorStroke(RGB), // RG
    SetRGBColorNonstroke(RGB), // rg
    SetGrayStroke(UnitInterval), // G
    SetGrayNonstroke(UnitInterval), // g
    // Shading operators
    ShFill(&'a Name), // sh
}
