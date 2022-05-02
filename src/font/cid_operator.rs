// beginrearrangedfont, endrearrangedfont, beginusematrix, endusematrix are
// not used in PDF - PDF 9.7.5.4.e
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CIDOperator {
    FindResource,
    Dict,
    Dup,
    Def,
    UseFont,
    UseCMap,
    Begin,
    End,
    BeginCMap,
    EndCMap,
    BeginCodeSpaceRange,
    EndCodeSpaceRange,
    BeginBfChar,
    EndBfChar,
    BeginBfRange,
    EndBfRange,
    BeginCIDChar,
    EndCIDChar,
    BeginCIDRange,
    EndCIDRange,
    BeginNotdefChar,
    EndNotdefChar,
    BeginNotdefRange,
    EndNotdefRange,
}
