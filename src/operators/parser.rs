use crate::operators::operations::{CMYK, Color, LineCap, LineJoin, RGB, StringOrNumber, TextRendering};
use crate::parser::parser::Object;
use super::operations::Operation;
use super::operators::Operator;

macro_rules! coerce_f64 {
    ($obj:expr) => {
        match $obj {
            Object::Real(n) => n,
            Object::Integer(n) => n as f64,
            _ => return None,
        }
    };
}

macro_rules! coerce_f32 {
    ($obj:expr) => {
        match $obj {
            Object::Real(n) => n as f32,
            Object::Integer(n) => n as f32,
            _ => return None,
        }
    };
}

macro_rules! coerce_string {
    ($obj:expr) => {
        match $obj {
            Object::String(s) => s,
            _ => return None,
        }
    };
}

pub struct OperatorParser {
    operators: Vec<Object>,
    pos: usize,
}

impl OperatorParser {
    pub fn new(operators: Vec<Object>) -> Self {
        Self {
            operators,
            pos: operators.len() - 1,
        }
    }

    pub fn parse(&mut self) -> Vec<Operation> {
        let mut vec = vec![];
        loop {
            if let Some(op) = self.next() {
                vec.push(op);
            } else {
                break;
            }
        }
        vec
    }

    fn next(&mut self) -> Option<Operation> {
        macro_rules! op {
            ($op:expr) => {
                {
                    self.advance();
                    $op
                }
            };
            ($op:expr, $i:expr) => {
                {
                    self.seek($i);
                    $op
                }
            };
        }

        let operand = match self.pop()? {
            Object::Operator(Operator::CloseStrokePath) => Operation::CloseStrokePath,
            Object::Operator(Operator::StrokePath) => Operation::StrokePath,
            Object::Operator(Operator::FillPath) => Operation::FillPath,
            Object::Operator(Operator::FillPathEvenOdd) => Operation::FillPathEvenOdd,
            Object::Operator(Operator::CloseFillStrokePath) => Operation::CloseFillStrokePath,
            Object::Operator(Operator::FillStrokePath) => Operation::FillStrokePath,
            Object::Operator(Operator::CloseFillStrokePathEvenOdd) => Operation::CloseFillStrokePathEvenOdd,
            Object::Operator(Operator::FillStrokePathEvenOdd) => Operation::FillStrokePathEvenOdd,
            Object::Operator(Operator::EndPathNoFill) => Operation::EndPathNoFill,
            Object::Operator(Operator::SetClippingPath) => Operation::SetClippingPath,
            Object::Operator(Operator::SetClippingPathEvenOdd) => Operation::SetClippingPathEvenOdd,
            Object::Operator(Operator::DefineMarkedContentPoint) => match self.peek()? {
                Object::Name(name) => op!(Operation::DefineMarkedContentPoint(name)),
                _ => return None,
            },
            Object::Operator(Operator::DefineMarkedContentPointPropertyList) => match (self.peek()?, self.nth(1)?) {
                (Object::Dictionary(dict), Object::Name(name)) => op!(Operation::DefineMarkedContentPointPropertyList(name, dict), 2),
                _ => return None,
            },
            Object::Operator(Operator::BeginMarkedContentSequence) => match self.peek()? {
                Object::Name(name) => op!(Operation::BeginMarkedContentSequence(name)),
                _ => return None,
            },
            Object::Operator(Operator::BeginMarkedContentSequencePropertyList) => match (self.peek()?, self.nth(1)?) {
                (Object::Dictionary(dict), Object::Name(name)) => op!(Operation::BeginMarkedContentSequencePropertyList(name, dict), 2),
                _ => return None,
            },
            Object::Operator(Operator::EndMarkedContentSequence) => Operation::EndMarkedContentSequence,
            Object::Operator(Operator::BeginInlineImageObject) => Operation::BeginInlineImageObject,
            Object::Operator(Operator::BeginInlineImageData) => Operation::BeginInlineImageData,
            Object::Operator(Operator::EndInlineImage) => Operation::EndInlineImage,
            Object::Operator(Operator::BeginText) => Operation::BeginText,
            Object::Operator(Operator::ShowText) => match self.peek()? {
                Object::String(string) => op!(Operation::ShowText(string)),
                _ => return None,
            },
            Object::Operator(Operator::ShowTextAdjusted) => match self.peek()? {
                Object::Array(arr) => {
                    let mut vec = vec![];
                    for obj in arr {
                        match obj {
                            Object::String(string) => vec.push(StringOrNumber::String(string)),
                            Object::Real(n) => vec.push(StringOrNumber::Number(n)),
                            Object::Integer(n) => vec.push(StringOrNumber::Number(n as f64)),
                            _ => (),
                        }
                    }
                    op!(Operation::ShowTextAdjusted(vec))
                },
                _ => return None,
            },
            Object::Operator(Operator::MoveNextLineShowText) => match self.peek()? {
                Object::String(string) => op!(Operation::MoveNextLineShowText(string)),
                _ => return None,
            },
            Object::Operator(Operator::SetSpacingMoveNextLineShowText) => {
                let text = self.peek()?;
                let ac = self.nth(1)?;
                let aw = self.nth(2)?;
                let aw = coerce_f64!(aw);
                let ac = coerce_f64!(ac);
                let text = coerce_string!(text);
                op!(Operation::SetSpacingMoveNextLineShowText(aw, ac, text), 3)
            },
            Object::Operator(Operator::MoveTextPosition) => {
                let ty = coerce_f64!(self.peek()?);
                let tx = coerce_f64!(self.nth(1)?);
                op!(Operation::MoveTextPosition(tx, ty), 2)
            },
            Object::Operator(Operator::MoveTextPositionLeading) => {
                let ty = coerce_f64!(self.peek()?);
                let tx = coerce_f64!(self.nth(1)?);
                op!(Operation::MoveTextPositionLeading(tx, ty), 2)
            },
            Object::Operator(Operator::SetTextMatrix) => {
                let f = coerce_f64!(self.peek()?);
                let e = coerce_f64!(self.nth(1)?);
                let d = coerce_f64!(self.nth(2)?);
                let c = coerce_f64!(self.nth(3)?);
                let b = coerce_f64!(self.nth(4)?);
                let a = coerce_f64!(self.nth(5)?);
                let matrix = [a, b, c, d, e, f];
                op!(Operation::SetTextMatrix(matrix), 6)
            },
            Object::Operator(Operator::MoveStartNextLine) => Operation::MoveStartNextLine,
            Object::Operator(Operator::SetCharSpacing) => {
                let char_space = coerce_f64!(self.peek()?);
                op!(Operation::SetCharSpacing(char_space))
            },
            Object::Operator(Operator::SelectFont) => {
                let size = coerce_f64!(self.peek()?);
                let text = coerce_string!(self.nth(1)?);
                op!(Operation::SelectFont(text, size), 2)
            },
            Object::Operator(Operator::SetTextLeading) => {
                let leading = coerce_f64!(self.peek()?);
                op!(Operation::SetTextLeading(leading))
            },
            Object::Operator(Operator::SetTextRendering) => match self.peek()? {
                Object::Integer(i) => {
                    let render = TextRendering::from_i64(i)?;
                    op!(Operation::SetTextRendering(render))
                },
                _ => return None,
            },
            Object::Operator(Operator::SetTextRise) => {
                let rise = coerce_f64!(self.peek()?);
                op!(Operation::SetTextRise(rise))
            },
            Object::Operator(Operator::SetWordSpacing) => {
                let word_space = coerce_f64!(self.peek()?);
                op!(Operation::SetWordSpacing(word_space))
            },
            Object::Operator(Operator::SetHorizontalTextScaling) => {
                let scale = coerce_f64!(self.peek()?);
                op!(Operation::SetHorizontalTextScaling(scale))
            },
            Object::Operator(Operator::EndText) => Operation::EndText,
            Object::Operator(Operator::BeginCompat) => Operation::BeginCompat,
            Object::Operator(Operator::SetCharWidth) => {
                let wy = coerce_f64!(self.peek()?);
                let wx = coerce_f64!(self.nth(1)?);
                op!(Operation::SetCharWidth(wx, wy), 2)
            },
            Object::Operator(Operator::SetCacheDevice) => {
                let ur_y = coerce_f64!(self.peek()?);
                let ur_x = coerce_f64!(self.nth(1)?);
                let ll_y = coerce_f64!(self.nth(2)?);
                let ll_x = coerce_f64!(self.nth(3)?);
                let wy = coerce_f64!(self.nth(4)?);
                let wx = coerce_f64!(self.nth(5)?);
                op!(Operation::SetCacheDevice((wx, wy), (ll_x, ll_y), (ur_x, ur_y)), 6)
            },
            Object::Operator(Operator::InvokeXObject) => match self.peek()? {
                Object::Name(name) => op!(Operation::InvokeXObject(name)),
                _ => return None,
            },
            Object::Operator(Operator::EndCompat) => Operation::EndCompat,
            Object::Operator(Operator::MoveTo) => {
                let y = coerce_f64!(self.peek()?);
                let x = coerce_f64!(self.nth(1)?);
                op!(Operation::MoveTo(x, y), 2)
            },
            Object::Operator(Operator::LineTo) => {
                let y = coerce_f64!(self.peek()?);
                let x = coerce_f64!(self.nth(1)?);
                op!(Operation::LineTo(x, y), 2)
            },
            Object::Operator(Operator::AppendCurveThreePoints) => {
                let y3 = coerce_f64!(self.peek()?);
                let x3 = coerce_f64!(self.nth(1)?);
                let y2 = coerce_f64!(self.nth(2)?);
                let x2 = coerce_f64!(self.nth(3)?);
                let y1 = coerce_f64!(self.nth(4)?);
                let x1 = coerce_f64!(self.nth(5)?);
                op!(Operation::AppendCurveThreePoints((x1, y1), (x2, y2), (x3, y3)), 6)
            },
            Object::Operator(Operator::AppendCurveInitialReplicated) => {
                let y3 = coerce_f64!(self.peek()?);
                let x3 = coerce_f64!(self.nth(1)?);
                let y2 = coerce_f64!(self.nth(2)?);
                let x2 = coerce_f64!(self.nth(3)?);
                op!(Operation::AppendCurveInitialReplicated((x2, y2), (x3, y3)), 4)
            },
            Object::Operator(Operator::AppendCurveFinalReplicated) => {
                let y3 = coerce_f64!(self.peek()?);
                let x3 = coerce_f64!(self.nth(1)?);
                let y1 = coerce_f64!(self.nth(2)?);
                let x1 = coerce_f64!(self.nth(3)?);
                op!(Operation::AppendCurveInitialReplicated((x1, y1), (x3, y3)), 4)
            },
            Object::Operator(Operator::AppendRectangle) => {
                let height = coerce_f64!(self.peek()?);
                let width = coerce_f64!(self.nth(1)?);
                let y = coerce_f64!(self.nth(2)?);
                let x = coerce_f64!(self.nth(3)?);
                op!(Operation::AppendRectangle(x, y, width, height), 4)
            },
            Object::Operator(Operator::CloseSubpath) => Operation::CloseSubpath,
            Object::Operator(Operator::SetMiterLimit) => {
                let miter_limit = coerce_f64!(self.peek()?);
                op!(Operation::SetMiterLimit(miter_limit))
            },
            Object::Operator(Operator::ConcatMatrix) => {
                let f = coerce_f64!(self.peek()?);
                let e = coerce_f64!(self.nth(1)?);
                let d = coerce_f64!(self.nth(2)?);
                let c = coerce_f64!(self.nth(3)?);
                let b = coerce_f64!(self.nth(4)?);
                let a = coerce_f64!(self.nth(5)?);
                let matrix = [a, b, c, d, e, f];
                op!(Operation::ConcatMatrix(matrix), 6)
            },
            Object::Operator(Operator::SetLineWidth) => {
                let width = coerce_f64!(self.peek()?);
                op!(Operation::SetLineWidth(width))
            },
            Object::Operator(Operator::SetLineJoin) => match self.peek()? {
                Object::Integer(i) => {
                    let join = LineJoin::from_i64(i)?;
                    op!(Operation::SetLineJoin(join))
                },
                _ => return None,
            },
            Object::Operator(Operator::SetLineCap) => match self.peek()? {
                Object::Integer(i) => {
                    let cap = LineCap::from_i64(i)?;
                    op!(Operation::SetLineCap(cap))
                },
                _ => return None,
            },
            Object::Operator(Operator::SetDash) => match (self.peek()?, self.nth(1)?) {
                (Object::Integer(i), Object::Array(arr)) => {
                    let vec = vec![];
                    for obj in arr {
                        if let Object::Integer(i) = obj {
                            vec.push(i)
                        }
                    }
                    op!(Operation::SetDash(vec, i), 2)
                },
                _ => return None,
            },
            Object::Operator(Operator::GSave) => Operation::GSave,
            Object::Operator(Operator::GRestore) => Operation::GRestore,
            Object::Operator(Operator::SetColorRenderingIntent) => match self.peek()? {
                Object::Name(name) => op!(Operation::SetColorRenderingIntent(name)),
                _ => return None,
            },
            Object::Operator(Operator::SetFlat) => {
                let flatness = coerce_f32!(self.peek()?);
                op!(Operation::SetFlat(flatness))
            },
            Object::Operator(Operator::SetGraphicsStateParams) => match self.peek()? {
                Object::Name(name) => op!(Operation::SetGraphicsStateParams(name)),
                _ => return None,
            },
            Object::Operator(Operator::SetCMYKColorStroke) => {
                let black = coerce_f32!(self.peek()?);
                let yellow = coerce_f32!(self.nth(1)?);
                let magenta = coerce_f32!(self.nth(2)?);
                let cyan = coerce_f32!(self.nth(3)?);
                let cmyk = CMYK {
                    cyan,
                    magenta,
                    yellow,
                    black
                };
                op!(Operation::SetCMYKColorStroke(cmyk), 4)
            },
            Object::Operator(Operator::SetCMYKColorNonstroke) => {
                let black = coerce_f32!(self.peek()?);
                let yellow = coerce_f32!(self.nth(1)?);
                let magenta = coerce_f32!(self.nth(2)?);
                let cyan = coerce_f32!(self.nth(3)?);
                let cmyk = CMYK {
                    cyan,
                    magenta,
                    yellow,
                    black
                };
                op!(Operation::SetCMYKColorNonstroke(cmyk), 4)
            },
            Object::Operator(Operator::SetColorStroke) => {
                let first = coerce_f32!(self.peek()?);
                if matches!(self.nth(1), Some(Object::Integer(_)) | Some(Object::Real(_))) {
                    let second = coerce_f32!(self.nth(1)?);
                    let third = match self.nth(2) {
                        Some(Object::Real(r)) => r as f32,
                        Some(Object::Integer(i)) => i as f32,
                        _ => {
                            self.seek(2);
                            return Some(Operation::SetColorStroke(Color::Gray(first)));
                        },
                    };
                    let fourth = match self.nth(3) {
                        Some(Object::Real(r)) => r as f32,
                        Some(Object::Integer(i)) => i as f32,
                        _ => {
                            self.seek(3);
                            let rgb = RGB {
                                red: third,
                                green: second,
                                blue: first,
                            };
                            return Some(Operation::SetColorStroke(Color::RGB(rgb)));
                        },
                    };
                    let cmyk = CMYK {
                        cyan: fourth,
                        magenta: third,
                        yellow: second,
                        black: first,
                    };
                    op!(Operation::SetColorStroke(Color::CMYK(cmyk)), 4)
                } else {
                    op!(Operation::SetColorStroke(Color::Gray(first)))
                }
            },
            Object::Operator(Operator::SetColorNonstroke) => {
                let first = coerce_f32!(self.peek()?);
                if matches!(self.nth(1), Some(Object::Integer(_)) | Some(Object::Real(_))) {
                    let second = coerce_f32!(self.nth(1)?);
                    let third = match self.nth(2) {
                        Some(Object::Real(r)) => r as f32,
                        Some(Object::Integer(i)) => i as f32,
                        _ => {
                            self.seek(2);
                            return Some(Operation::SetColorNonstroke(Color::Gray(first)));
                        },
                    };
                    let fourth = match self.nth(3) {
                        Some(Object::Real(r)) => r as f32,
                        Some(Object::Integer(i)) => i as f32,
                        _ => {
                            self.seek(3);
                            let rgb = RGB {
                                red: third,
                                green: second,
                                blue: first,
                            };
                            return Some(Operation::SetColorNonstroke(Color::RGB(rgb)));
                        },
                    };
                    let cmyk = CMYK {
                        cyan: fourth,
                        magenta: third,
                        yellow: second,
                        black: first,
                    };
                    op!(Operation::SetColorNonstroke(Color::CMYK(cmyk)), 4)
                } else {
                    op!(Operation::SetColorNonstroke(Color::Gray(first)))
                }
            },
            Object::Operator(Operator::SetColorSpecialStroke),
            Object::Operator(Operator::SetColorSpecialNonstroke),
            Object::Operator(Operator::SetColorSpaceStroke),
            Object::Operator(Operator::SetColorSpaceNonstroke),
            Object::Operator(Operator::SetRGBColorStroke),
            Object::Operator(Operator::SetRGBColorNonstroke),
            Object::Operator(Operator::SetGrayStroke),
            Object::Operator(Operator::SetGrayNonstroke),
            Object::Operator(Operator::ShFill),
            _ => return None,
        };
        Some(operand)
    }

    fn pop(&mut self) -> Option<Object> {
        // We need special handling or else we will never return the first
        // element since self.pos + 1 will always be 1 if pos is 0
        let is_first = self.pos == 0;
        self.advance();
        if is_first {
            Some(self.operators[0])
        } else {
            Some(self.operators[self.pos + 1])
        }
    }

    fn advance(&mut self) {
        self.seek(1)
    }

    fn seek(&mut self, n: usize) {
        if n <= self.pos {
            self.pos -= n;
        }
    }

    fn peek(&self) -> Option<Object> {
        self.nth(0)
    }

    fn nth(&self, n: usize) -> Option<Object> {
        if n <= self.pos {
            Some(self.operators[self.pos - n])
        } else {
            None
        }
    }
}