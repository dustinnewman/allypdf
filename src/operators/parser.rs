use crate::operators::operations::{CMYK, Color, LineCap, LineJoin, RGB, StringOrNumber, TextRendering};
use crate::parser::parser::Object;
use super::operations::Operation;
use super::operators::Operator;

macro_rules! coerce_f64 {
    ($obj:expr) => {
        match $obj {
            Object::Real(n) => *n,
            Object::Integer(n) => *n as f64,
            _ => return None,
        }
    };
}

macro_rules! coerce_f32 {
    ($obj:expr) => {
        match $obj {
            Object::Real(n) => *n as f32,
            Object::Integer(n) => *n as f32,
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

pub struct OperatorParser<'a> {
    operators: &'a Vec<Object>,
    pos: usize,
    len: usize,
    done: bool,
}

impl<'a> OperatorParser<'a> {
    pub fn new(operators: &'a Vec<Object>) -> Self {
        let len = operators.len();
        Self {
            operators,
            pos: if len != 0 { len - 1 } else { 0 },
            len,
            done: false,
        }
    }

    pub fn parse(&mut self) -> Vec<Operation<'a>> {
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

    fn next(&mut self) -> Option<Operation<'a>> {
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
                            Object::Real(n) => vec.push(StringOrNumber::Number(*n)),
                            Object::Integer(n) => vec.push(StringOrNumber::Number(*n as f64)),
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
                    let render = TextRendering::from_i64(*i)?;
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
                    let join = LineJoin::from_i64(*i)?;
                    op!(Operation::SetLineJoin(join))
                },
                _ => return None,
            },
            Object::Operator(Operator::SetLineCap) => match self.peek()? {
                Object::Integer(i) => {
                    let cap = LineCap::from_i64(*i)?;
                    op!(Operation::SetLineCap(cap))
                },
                _ => return None,
            },
            Object::Operator(Operator::SetDash) => match (self.peek()?, self.nth(1)?) {
                (Object::Integer(i), Object::Array(arr)) => {
                    let mut vec = vec![];
                    for obj in arr {
                        if let Object::Integer(i) = obj {
                            vec.push(*i)
                        }
                    }
                    op!(Operation::SetDash(vec, *i), 2)
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
                let color = self.handle_color(0)?;
                Operation::SetColorStroke(color)
            },
            Object::Operator(Operator::SetColorNonstroke) => {
                let color = self.handle_color(0)?;
                Operation::SetColorNonstroke(color)
            },
            Object::Operator(Operator::SetColorSpecialStroke) => {
                let name = match self.peek()? {
                    Object::Name(n) => Some(n),
                    _ => None,
                };
                let start: usize = if name.is_some() { 1 } else { 0 };
                let color = self.handle_color(start)?;
                Operation::SetColorSpecialStroke(color, name)
            },
            Object::Operator(Operator::SetColorSpecialNonstroke) => {
                let first = self.peek()?;
                let name = match first {
                    Object::Name(n) => Some(n),
                    _ => None,
                };
                let start: usize = if name.is_some() { 1 } else { 0 };
                let color = self.handle_color(start)?;
                Operation::SetColorSpecialNonstroke(color, name)
            },
            Object::Operator(Operator::SetColorSpaceStroke) => match self.peek()? {
                Object::Name(name) => op!(Operation::SetColorSpaceStroke(name)),
                _ => return None,
            },
            Object::Operator(Operator::SetColorSpaceNonstroke) => match self.peek()? {
                Object::Name(name) => op!(Operation::SetColorSpaceNonstroke(name)),
                _ => return None,
            },
            Object::Operator(Operator::SetRGBColorStroke) => {
                let blue = coerce_f32!(self.peek()?);
                let green = coerce_f32!(self.nth(1)?);
                let red = coerce_f32!(self.nth(2)?);
                let rgb = RGB {
                    red,
                    green,
                    blue,
                };
                op!(Operation::SetRGBColorStroke(rgb), 3)
            },
            Object::Operator(Operator::SetRGBColorNonstroke) => {
                let blue = coerce_f32!(self.peek()?);
                let green = coerce_f32!(self.nth(1)?);
                let red = coerce_f32!(self.nth(2)?);
                let rgb = RGB {
                    red,
                    green,
                    blue,
                };
                op!(Operation::SetRGBColorNonstroke(rgb), 3)
            },
            Object::Operator(Operator::SetGrayStroke) => {
                let gray = coerce_f32!(self.peek()?);
                op!(Operation::SetGrayStroke(gray))
            },
            Object::Operator(Operator::SetGrayNonstroke) => {
                let gray = coerce_f32!(self.peek()?);
                op!(Operation::SetGrayNonstroke(gray))
            },
            Object::Operator(Operator::ShFill) => match self.peek()? {
                Object::Name(name) => op!(Operation::ShFill(name)),
                _ => return None,
            },
            _ => return None,
        };
        Some(operand)
    }

    fn handle_color(&mut self, start: usize) -> Option<Color> {
        let first = coerce_f32!(self.nth(start)?);
        if matches!(self.nth(start + 1), Some(Object::Integer(_)) | Some(Object::Real(_))) {
            let second = coerce_f32!(self.nth(start + 1)?);
            let third = match self.nth(start + 2) {
                Some(Object::Real(r)) => *r as f32,
                Some(Object::Integer(i)) => *i as f32,
                _ => {
                    self.seek(start + 2);
                    return Some(Color::Gray(first));
                },
            };
            let fourth = match self.nth(start + 3) {
                Some(Object::Real(r)) => *r as f32,
                Some(Object::Integer(i)) => *i as f32,
                _ => {
                    self.seek(start + 3);
                    let rgb = RGB {
                        red: third,
                        green: second,
                        blue: first,
                    };
                    return Some(Color::RGB(rgb));
                },
            };
            let cmyk = CMYK {
                cyan: fourth,
                magenta: third,
                yellow: second,
                black: first,
            };
            self.seek(start + 4);
            return Some(Color::CMYK(cmyk));
        } else {
            self.seek(start + 1);
            return Some(Color::Gray(first));
        }
    }

    fn pop(&mut self) -> Option<&'a Object> {
        if self.done {
            return None;
        }
        // We need special handling or else we will never return the first
        // element since self.pos + 1 will always be 1 if pos is 0
        let is_first = self.pos == 0;
        self.advance();
        if is_first && self.len != 0 {
            Some(&self.operators[0])
        } else if self.pos + 1 < self.len {
            Some(&self.operators[self.pos + 1])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.seek(1)
    }

    fn seek(&mut self, n: usize) {
        if self.pos == 0 {
            self.done = true;
        } else if n <= self.pos {
            self.pos -= n;
        }
    }

    fn peek(&self) -> Option<&'a Object> {
        self.nth(0)
    }

    fn nth(&self, n: usize) -> Option<&'a Object> {
        if n <= self.pos && self.len != 0 {
            Some(&self.operators[self.pos - n])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use super::*;

    #[test]
    fn test_empty() {
        let objects = vec![];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_no_args() {
        let objects = vec![Object::Operator(Operator::CloseStrokePath)];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![Operation::CloseStrokePath];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_too_few_args() {
        let objects = vec![
            Object::Operator(Operator::SetGrayStroke)
        ];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_one_arg_int() {
        let objects = vec![
            Object::Integer(0),
            Object::Operator(Operator::SetGrayStroke)
        ];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![Operation::SetGrayStroke(0.0)];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_one_arg_real() {
        let objects = vec![
            Object::Real(0.5),
            Object::Operator(Operator::SetGrayNonstroke)
        ];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![Operation::SetGrayNonstroke(0.5)];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_one_arg_name() {
        let objects = vec![
            Object::Name(b"test".to_vec()),
            Object::Operator(Operator::DefineMarkedContentPoint)
        ];
        let mut parser = OperatorParser::new(&objects);
        let name = b"test".to_vec();
        let operations = vec![Operation::DefineMarkedContentPoint(&name)];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_two_args_name_dict() {
        let mut properties = BTreeMap::new();
        properties.insert(b"Length".to_vec(), Object::Integer(6));
        let objects = vec![
            Object::Name(b"test".to_vec()),
            Object::Dictionary(properties),
            Object::Operator(Operator::DefineMarkedContentPointPropertyList)
        ];
        let mut parser = OperatorParser::new(&objects);
        let key = b"Length".to_vec();
        let result = parser.parse();
        assert!(matches!(result[0], Operation::DefineMarkedContentPointPropertyList(..)));
        let length = match result[0] {
            Operation::DefineMarkedContentPointPropertyList(_, dict) => dict.get(&key),
            _ => None
        };
        assert!(matches!(length, Some(&Object::Integer(6))));
        let name = match result[0] {
            Operation::DefineMarkedContentPointPropertyList(n, _) => Some(n),
            _ => None,
        };
        let tag = b"test".to_vec();
        assert_eq!(name, Some(&tag));
    }

    #[test]
    fn test_three_args_real() {
        let objects = vec![
            Object::Real(1.0),
            Object::Real(1.0),
            Object::Real(1.0),
            Object::Operator(Operator::SetRGBColorStroke)
        ];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![Operation::SetRGBColorStroke(RGB {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        })];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_three_args_too_few() {
        let objects = vec![
            Object::Null,
            Object::Real(1.0),
            Object::Real(1.0),
            Object::Operator(Operator::SetRGBColorStroke)
        ];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_cmyk() {
        let objects = vec![
            Object::Real(1.0),
            Object::Integer(0),
            Object::Real(1.0),
            Object::Real(0.0),
            Object::Operator(Operator::SetColorStroke)
        ];
        let mut parser = OperatorParser::new(&objects);
        let color = CMYK {
            cyan: 1.0,
            magenta: 0.0,
            yellow: 1.0,
            black: 0.0,
        };
        let operations = vec![Operation::SetColorStroke(Color::CMYK(color))];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_rgb() {
        let objects = vec![
            Object::Real(0.3),
            Object::Integer(0),
            Object::Real(0.7),
            Object::Operator(Operator::SetColorStroke)
        ];
        let mut parser = OperatorParser::new(&objects);
        let color = RGB {
            red: 0.3,
            green: 0.0,
            blue: 0.7,
        };
        let operations = vec![Operation::SetColorStroke(Color::RGB(color))];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_gray() {
        let objects = vec![
            Object::Real(0.7),
            Object::Operator(Operator::SetColorStroke)
        ];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![Operation::SetColorStroke(Color::Gray(0.7))];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_pattern_name_cmyk() {
        let objects = vec![
            Object::Real(1.0),
            Object::Integer(0),
            Object::Real(1.0),
            Object::Real(0.0),
            Object::Name(b"COLOR".to_vec()),
            Object::Operator(Operator::SetColorSpecialStroke)
        ];
        let mut parser = OperatorParser::new(&objects);
        let color = CMYK {
            cyan: 1.0,
            magenta: 0.0,
            yellow: 1.0,
            black: 0.0,
        };
        let name = b"COLOR".to_vec();
        let operations = vec![Operation::SetColorSpecialStroke(Color::CMYK(color), Some(&name))];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_pattern_name_rgb() {
        let objects = vec![
            Object::Real(0.3),
            Object::Integer(0),
            Object::Real(0.7),
            Object::Name(b"COLOR".to_vec()),
            Object::Operator(Operator::SetColorSpecialStroke)
        ];
        let mut parser = OperatorParser::new(&objects);
        let color = RGB {
            red: 0.3,
            green: 0.0,
            blue: 0.7,
        };
        let name = b"COLOR".to_vec();
        let operations = vec![Operation::SetColorSpecialStroke(Color::RGB(color), Some(&name))];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_pattern_name_gray() {
        let objects = vec![
            Object::Real(0.7),
            Object::Name(b"COLOR".to_vec()),
            Object::Operator(Operator::SetColorSpecialStroke)
        ];
        let mut parser = OperatorParser::new(&objects);
        let name = b"COLOR".to_vec();
        let operations = vec![Operation::SetColorSpecialStroke(Color::Gray(0.7), Some(&name))];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_pattern_cmyk() {
        let objects = vec![
            Object::Real(1.0),
            Object::Integer(0),
            Object::Real(1.0),
            Object::Real(0.0),
            Object::Operator(Operator::SetColorSpecialStroke)
        ];
        let mut parser = OperatorParser::new(&objects);
        let color = CMYK {
            cyan: 1.0,
            magenta: 0.0,
            yellow: 1.0,
            black: 0.0,
        };
        let operations = vec![Operation::SetColorSpecialStroke(Color::CMYK(color), None)];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_pattern_rgb() {
        let objects = vec![
            Object::Real(0.3),
            Object::Integer(0),
            Object::Real(0.7),
            Object::Operator(Operator::SetColorSpecialStroke)
        ];
        let mut parser = OperatorParser::new(&objects);
        let color = RGB {
            red: 0.3,
            green: 0.0,
            blue: 0.7,
        };
        let operations = vec![Operation::SetColorSpecialStroke(Color::RGB(color), None)];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_pattern_gray() {
        let objects = vec![
            Object::Real(0.7),
            Object::Operator(Operator::SetColorSpecialStroke)
        ];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![Operation::SetColorSpecialStroke(Color::Gray(0.7), None)];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_show_text_adjusted_string_number() {
        // [(Le)15(x)-250(Fridman)]TJ
        let array = vec![
            Object::String(b"Le".to_vec()),
            Object::Integer(15),
            Object::String(b"x".to_vec()),
            Object::Integer(-250),
            Object::String(b"Fridman".to_vec()),
        ];
        let objects = vec![
            Object::Array(array),
            Object::Operator(Operator::ShowTextAdjusted)
        ];
        let mut parser = OperatorParser::new(&objects);
        let le = b"Le".to_vec();
        let x = b"x".to_vec();
        let fridman = b"Fridman".to_vec();
        let text = vec![
            StringOrNumber::String(&le),
            StringOrNumber::Number(15.0),
            StringOrNumber::String(&x),
            StringOrNumber::Number(-250.0),
            StringOrNumber::String(&fridman),
        ];
        let operations = vec![Operation::ShowTextAdjusted(text)];
        assert_eq!(parser.parse(), operations);
    }
}