#[derive(Debug, PartialEq)]
pub enum Operator {
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
    DefineMarkedContentPoint, // MP
    DefineMarkedContentPointPropertyList, // DP
    BeginMarkedContentSequence, // BMC
    BeginMarkedContentSequencePropertyList, // BDC
    EndMarkedContentSequence, // EMC
    // Image operators
    BeginInlineImageObject, // BI
    BeginInlineImageData, // ID
    EndInlineImage, // EI
    // Text operators
    BeginText, // BT
    ShowText, // Tj
    ShowTextAdjusted, // TJ
    MoveNextLineShowText, // '
    SetSpacingMoveNextLineShowText, // "
    MoveTextPosition, // Td
    MoveTextPositionLeading, // TD
    SetTextMatrix, // Tm
    MoveStartNextLine, // T*
    SetCharSpacing, // Tc
    SelectFont, // Tf
    SetTextLeading, // TL
    SetTextRendering, // Tr
    SetTextRise, // Ts
    SetWordSpacing, // Tw
    SetHorizontalTextScaling, // Tz
    EndText, // ET
    BeginCompat, // BX
    // Type 3 font operators
    SetCharWidth, // d0
    SetCacheDevice, // d1
    InvokeXObject, // Do
    EndCompat, // EX
    // Path construction operators
    MoveTo, // m
    LineTo, // l
    AppendCurveThreePoints, // c
    AppendCurveInitialReplicated, // v
    AppendCurveFinalReplicated, // y
    AppendRectangle, // re
    CloseSubpath, // h
    SetMiterLimit, // M
    ConcatMatrix, // cm
    SetLineWidth, // w
    SetLineJoin, // j
    SetLineCap, // J
    SetDash, // d
    GSave, // q
    GRestore, // Q
    SetColorRenderingIntent, // ri
    SetFlat, // i
    SetGraphicsStateParams, // gs
    // Color operators
    SetCMYKColorStroke, // K
    SetCMYKColorNonstroke, // k
    SetColorStroke, // SC
    SetColorNonstroke, // sc
    SetColorSpecialStroke, // SCN
    SetColorSpecialNonstroke, // scn
    SetColorSpaceStroke, // CS
    SetColorSpaceNonstroke, // cs
    SetRGBColorStroke, // RG
    SetRGBColorNonstroke, // rg
    SetGrayStroke, // G
    SetGrayNonstroke, // g
    // Shading operators
    ShFill, // sh
}
