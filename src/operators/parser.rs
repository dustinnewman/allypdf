use super::color::Color;
use super::operations::Operation;
use super::operators::Operator;
use crate::operators::color::{Cmyk, Rgb};
use crate::operators::operations::{LineCap, LineJoin, StringOrNumber, TextRendering};
use crate::parser::object::{Object, ObjectKind};

macro_rules! coerce_f64 {
    ($obj:expr) => {
        match $obj {
            ObjectKind::Real(n) => *n as f64,
            ObjectKind::Integer(n) => *n as f64,
            _ => return None,
        }
    };
}

macro_rules! coerce_string {
    ($obj:expr) => {
        match $obj {
            ObjectKind::String(s) => s,
            _ => return None,
        }
    };
}

macro_rules! coerce_name {
    ($obj:expr) => {
        match $obj {
            ObjectKind::Name(s) => s,
            _ => return None,
        }
    };
}

pub struct OperatorParser<'a> {
    operators: &'a [Object],
    pos: usize,
    len: usize,
    done: bool,
}

impl<'a> OperatorParser<'a> {
    pub fn new(operators: &'a [Object]) -> Self {
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
        while let Some(op) = self.next() {
            vec.push(op);
        }
        vec.reverse();
        vec
    }

    fn next(&mut self) -> Option<Operation<'a>> {
        macro_rules! op {
            ($op:expr) => {{
                self.advance();
                $op
            }};
            ($op:expr, $i:expr) => {{
                self.seek($i);
                $op
            }};
        }
        let object = self.pop()?;
        let operand = match object.kind {
            ObjectKind::Operator(Operator::CloseStrokePath) => Operation::CloseStrokePath,
            ObjectKind::Operator(Operator::StrokePath) => Operation::StrokePath,
            ObjectKind::Operator(Operator::FillPath) => Operation::FillPath,
            ObjectKind::Operator(Operator::FillPathEvenOdd) => Operation::FillPathEvenOdd,
            ObjectKind::Operator(Operator::CloseFillStrokePath) => Operation::CloseFillStrokePath,
            ObjectKind::Operator(Operator::FillStrokePath) => Operation::FillStrokePath,
            ObjectKind::Operator(Operator::CloseFillStrokePathEvenOdd) => {
                Operation::CloseFillStrokePathEvenOdd
            }
            ObjectKind::Operator(Operator::FillStrokePathEvenOdd) => {
                Operation::FillStrokePathEvenOdd
            }
            ObjectKind::Operator(Operator::EndPathNoFill) => Operation::EndPathNoFill,
            ObjectKind::Operator(Operator::SetClippingPath) => Operation::SetClippingPath,
            ObjectKind::Operator(Operator::SetClippingPathEvenOdd) => {
                Operation::SetClippingPathEvenOdd
            }
            ObjectKind::Operator(Operator::DefineMarkedContentPoint) => match &self.peek()?.kind {
                ObjectKind::Name(name) => op!(Operation::DefineMarkedContentPoint(name)),
                _ => return None,
            },
            ObjectKind::Operator(Operator::DefineMarkedContentPointPropertyList) => {
                match (&self.peek()?.kind, &self.nth(1)?.kind) {
                    (ObjectKind::Dictionary(dict), ObjectKind::Name(name)) => op!(
                        Operation::DefineMarkedContentPointPropertyList(name, dict),
                        2
                    ),
                    _ => return None,
                }
            }
            ObjectKind::Operator(Operator::BeginMarkedContentSequence) => {
                match &self.peek()?.kind {
                    ObjectKind::Name(name) => op!(Operation::BeginMarkedContentSequence(name)),
                    _ => return None,
                }
            }
            ObjectKind::Operator(Operator::BeginMarkedContentSequencePropertyList) => {
                match (&self.peek()?.kind, &self.nth(1)?.kind) {
                    (ObjectKind::Dictionary(dict), ObjectKind::Name(name)) => op!(
                        Operation::BeginMarkedContentSequencePropertyList(name, dict),
                        2
                    ),
                    _ => return None,
                }
            }
            ObjectKind::Operator(Operator::EndMarkedContentSequence) => {
                Operation::EndMarkedContentSequence
            }
            ObjectKind::Operator(Operator::BeginInlineImageObject) => {
                Operation::BeginInlineImageObject
            }
            ObjectKind::Operator(Operator::BeginInlineImageData) => Operation::BeginInlineImageData,
            ObjectKind::Operator(Operator::EndInlineImage) => Operation::EndInlineImage,
            ObjectKind::Operator(Operator::BeginText) => Operation::BeginText,
            ObjectKind::Operator(Operator::ShowText) => match &self.peek()?.kind {
                ObjectKind::String(string) => op!(Operation::ShowText(string)),
                _ => return None,
            },
            ObjectKind::Operator(Operator::ShowTextAdjusted) => match &self.peek()?.kind {
                ObjectKind::Array(arr) => {
                    let mut vec = vec![];
                    for obj in arr {
                        let element = match &obj.kind {
                            ObjectKind::String(string) => StringOrNumber::String(string),
                            ObjectKind::Real(n) => StringOrNumber::Number(*n),
                            ObjectKind::Integer(n) => StringOrNumber::Number(*n as f64),
                            _ => continue,
                        };
                        vec.push(element);
                    }
                    op!(Operation::ShowTextAdjusted(vec))
                }
                _ => return None,
            },
            ObjectKind::Operator(Operator::MoveNextLineShowText) => match &self.peek()?.kind {
                ObjectKind::String(string) => op!(Operation::MoveNextLineShowText(string)),
                _ => return None,
            },
            ObjectKind::Operator(Operator::SetSpacingMoveNextLineShowText) => {
                let text = &self.peek()?.kind;
                let ac = &self.nth(1)?.kind;
                let aw = &self.nth(2)?.kind;
                let aw = coerce_f64!(aw);
                let ac = coerce_f64!(ac);
                let text = coerce_string!(text);
                op!(Operation::SetSpacingMoveNextLineShowText(aw, ac, text), 3)
            }
            ObjectKind::Operator(Operator::MoveTextPosition) => {
                let ty = coerce_f64!(&self.peek()?.kind);
                let tx = coerce_f64!(&self.nth(1)?.kind);
                op!(Operation::MoveTextPosition(tx, ty), 2)
            }
            ObjectKind::Operator(Operator::MoveTextPositionLeading) => {
                let ty = coerce_f64!(&self.peek()?.kind);
                let tx = coerce_f64!(&self.nth(1)?.kind);
                op!(Operation::MoveTextPositionLeading(tx, ty), 2)
            }
            ObjectKind::Operator(Operator::SetTextMatrix) => {
                let f = coerce_f64!(&self.peek()?.kind);
                let e = coerce_f64!(&self.nth(1)?.kind);
                let d = coerce_f64!(&self.nth(2)?.kind);
                let c = coerce_f64!(&self.nth(3)?.kind);
                let b = coerce_f64!(&self.nth(4)?.kind);
                let a = coerce_f64!(&self.nth(5)?.kind);
                let matrix = [a, b, c, d, e, f];
                op!(Operation::SetTextMatrix(matrix), 6)
            }
            ObjectKind::Operator(Operator::MoveStartNextLine) => Operation::MoveStartNextLine,
            ObjectKind::Operator(Operator::SetCharSpacing) => {
                let char_space = coerce_f64!(&self.peek()?.kind);
                op!(Operation::SetCharSpacing(char_space))
            }
            ObjectKind::Operator(Operator::SelectFont) => {
                let size = coerce_f64!(&self.peek()?.kind);
                let text = coerce_name!(&self.nth(1)?.kind);
                op!(Operation::SelectFont(text, size), 2)
            }
            ObjectKind::Operator(Operator::SetTextLeading) => {
                let leading = coerce_f64!(&self.peek()?.kind);
                op!(Operation::SetTextLeading(leading))
            }
            ObjectKind::Operator(Operator::SetTextRendering) => match self.peek()?.kind {
                ObjectKind::Integer(i) => {
                    op!(Operation::SetTextRendering(TextRendering::from_i64(i)?))
                }
                _ => return None,
            },
            ObjectKind::Operator(Operator::SetTextRise) => {
                let rise = coerce_f64!(&self.peek()?.kind);
                op!(Operation::SetTextRise(rise))
            }
            ObjectKind::Operator(Operator::SetWordSpacing) => {
                let word_space = coerce_f64!(&self.peek()?.kind);
                op!(Operation::SetWordSpacing(word_space))
            }
            ObjectKind::Operator(Operator::SetHorizontalTextScaling) => {
                let scale = coerce_f64!(&self.peek()?.kind);
                op!(Operation::SetHorizontalTextScaling(scale))
            }
            ObjectKind::Operator(Operator::EndText) => Operation::EndText,
            ObjectKind::Operator(Operator::BeginCompat) => Operation::BeginCompat,
            ObjectKind::Operator(Operator::SetCharWidth) => {
                let wy = coerce_f64!(&self.peek()?.kind);
                let wx = coerce_f64!(&self.nth(1)?.kind);
                op!(Operation::SetCharWidth(wx, wy), 2)
            }
            ObjectKind::Operator(Operator::SetCacheDevice) => {
                let ur_y = coerce_f64!(&self.peek()?.kind);
                let ur_x = coerce_f64!(&self.nth(1)?.kind);
                let ll_y = coerce_f64!(&self.nth(2)?.kind);
                let ll_x = coerce_f64!(&self.nth(3)?.kind);
                let wy = coerce_f64!(&self.nth(4)?.kind);
                let wx = coerce_f64!(&self.nth(5)?.kind);
                op!(
                    Operation::SetCacheDevice((wx, wy), (ll_x, ll_y), (ur_x, ur_y)),
                    6
                )
            }
            ObjectKind::Operator(Operator::InvokeXObject) => match &self.peek()?.kind {
                ObjectKind::Name(name) => op!(Operation::InvokeXObject(name)),
                _ => return None,
            },
            ObjectKind::Operator(Operator::EndCompat) => Operation::EndCompat,
            ObjectKind::Operator(Operator::MoveTo) => {
                let y = coerce_f64!(&self.peek()?.kind);
                let x = coerce_f64!(&self.nth(1)?.kind);
                op!(Operation::MoveTo(x, y), 2)
            }
            ObjectKind::Operator(Operator::LineTo) => {
                let y = coerce_f64!(&self.peek()?.kind);
                let x = coerce_f64!(&self.nth(1)?.kind);
                op!(Operation::LineTo(x, y), 2)
            }
            ObjectKind::Operator(Operator::AppendCurveThreePoints) => {
                let y3 = coerce_f64!(&self.peek()?.kind);
                let x3 = coerce_f64!(&self.nth(1)?.kind);
                let y2 = coerce_f64!(&self.nth(2)?.kind);
                let x2 = coerce_f64!(&self.nth(3)?.kind);
                let y1 = coerce_f64!(&self.nth(4)?.kind);
                let x1 = coerce_f64!(&self.nth(5)?.kind);
                op!(
                    Operation::AppendCurveThreePoints((x1, y1), (x2, y2), (x3, y3)),
                    6
                )
            }
            ObjectKind::Operator(Operator::AppendCurveInitialReplicated) => {
                let y3 = coerce_f64!(&self.peek()?.kind);
                let x3 = coerce_f64!(&self.nth(1)?.kind);
                let y2 = coerce_f64!(&self.nth(2)?.kind);
                let x2 = coerce_f64!(&self.nth(3)?.kind);
                op!(
                    Operation::AppendCurveInitialReplicated((x2, y2), (x3, y3)),
                    4
                )
            }
            ObjectKind::Operator(Operator::AppendCurveFinalReplicated) => {
                let y3 = coerce_f64!(&self.peek()?.kind);
                let x3 = coerce_f64!(&self.nth(1)?.kind);
                let y1 = coerce_f64!(&self.nth(2)?.kind);
                let x1 = coerce_f64!(&self.nth(3)?.kind);
                op!(
                    Operation::AppendCurveInitialReplicated((x1, y1), (x3, y3)),
                    4
                )
            }
            ObjectKind::Operator(Operator::AppendRectangle) => {
                let height = coerce_f64!(&self.peek()?.kind);
                let width = coerce_f64!(&self.nth(1)?.kind);
                let y = coerce_f64!(&self.nth(2)?.kind);
                let x = coerce_f64!(&self.nth(3)?.kind);
                op!(Operation::AppendRectangle(x, y, width, height), 4)
            }
            ObjectKind::Operator(Operator::CloseSubpath) => Operation::CloseSubpath,
            ObjectKind::Operator(Operator::SetMiterLimit) => {
                let miter_limit = coerce_f64!(&self.peek()?.kind);
                op!(Operation::SetMiterLimit(miter_limit))
            }
            ObjectKind::Operator(Operator::ConcatMatrix) => {
                let f = coerce_f64!(&self.peek()?.kind);
                let e = coerce_f64!(&self.nth(1)?.kind);
                let d = coerce_f64!(&self.nth(2)?.kind);
                let c = coerce_f64!(&self.nth(3)?.kind);
                let b = coerce_f64!(&self.nth(4)?.kind);
                let a = coerce_f64!(&self.nth(5)?.kind);
                let matrix = [a, b, c, d, e, f];
                op!(Operation::ConcatMatrix(matrix), 6)
            }
            ObjectKind::Operator(Operator::SetLineWidth) => {
                let width = coerce_f64!(&self.peek()?.kind);
                op!(Operation::SetLineWidth(width))
            }
            ObjectKind::Operator(Operator::SetLineJoin) => match &self.peek()?.kind {
                ObjectKind::Integer(i) => {
                    let join = LineJoin::from_i64(*i)?;
                    op!(Operation::SetLineJoin(join))
                }
                _ => return None,
            },
            ObjectKind::Operator(Operator::SetLineCap) => match &self.peek()?.kind {
                ObjectKind::Integer(i) => op!(Operation::SetLineCap(LineCap::from_i64(*i)?)),
                _ => return None,
            },
            ObjectKind::Operator(Operator::SetDash) => {
                let phase = coerce_f64!(&self.peek()?.kind);
                if let ObjectKind::Array(arr) = &self.nth(1)?.kind {
                    let mut vec = vec![];
                    for obj in arr {
                        if let ObjectKind::Integer(j) = obj.kind {
                            vec.push(j as f64)
                        } else if let ObjectKind::Real(r) = obj.kind {
                            vec.push(r);
                        }
                    }
                    op!(Operation::SetDash(vec, phase), 2)
                } else {
                    return None;
                }
            }
            ObjectKind::Operator(Operator::GSave) => Operation::GSave,
            ObjectKind::Operator(Operator::GRestore) => Operation::GRestore,
            ObjectKind::Operator(Operator::SetColorRenderingIntent) => match &self.peek()?.kind {
                ObjectKind::Name(name) => op!(Operation::SetColorRenderingIntent(name)),
                _ => return None,
            },
            ObjectKind::Operator(Operator::SetFlat) => {
                let flatness = coerce_f64!(&self.peek()?.kind);
                op!(Operation::SetFlat(flatness))
            }
            ObjectKind::Operator(Operator::SetGraphicsStateParams) => match &self.peek()?.kind {
                ObjectKind::Name(name) => op!(Operation::SetGraphicsStateParams(name)),
                _ => return None,
            },
            ObjectKind::Operator(Operator::SetCMYKColorStroke) => {
                let black = coerce_f64!(&self.peek()?.kind);
                let yellow = coerce_f64!(&self.nth(1)?.kind);
                let magenta = coerce_f64!(&self.nth(2)?.kind);
                let cyan = coerce_f64!(&self.nth(3)?.kind);
                let cmyk = Cmyk {
                    cyan,
                    magenta,
                    yellow,
                    black,
                };
                op!(Operation::SetCMYKColorStroke(cmyk), 4)
            }
            ObjectKind::Operator(Operator::SetCMYKColorNonstroke) => {
                let black = coerce_f64!(&self.peek()?.kind);
                let yellow = coerce_f64!(&self.nth(1)?.kind);
                let magenta = coerce_f64!(&self.nth(2)?.kind);
                let cyan = coerce_f64!(&self.nth(3)?.kind);
                let cmyk = Cmyk {
                    cyan,
                    magenta,
                    yellow,
                    black,
                };
                op!(Operation::SetCMYKColorNonstroke(cmyk), 4)
            }
            ObjectKind::Operator(Operator::SetColorStroke) => {
                let color = self.handle_color(0)?;
                Operation::SetColorStroke(color)
            }
            ObjectKind::Operator(Operator::SetColorNonstroke) => {
                let color = self.handle_color(0)?;
                Operation::SetColorNonstroke(color)
            }
            ObjectKind::Operator(Operator::SetColorSpecialStroke) => {
                let name = match &self.peek()?.kind {
                    ObjectKind::Name(name) => Some(name),
                    _ => None,
                };
                let start: usize = if name.is_some() { 1 } else { 0 };
                let color = self.handle_color(start)?;
                Operation::SetColorSpecialStroke(color, name)
            }
            ObjectKind::Operator(Operator::SetColorSpecialNonstroke) => {
                let first = &self.peek()?.kind;
                let name = match first {
                    ObjectKind::Name(name) => Some(name),
                    _ => None,
                };
                let start: usize = if name.is_some() { 1 } else { 0 };
                let color = self.handle_color(start)?;
                Operation::SetColorSpecialNonstroke(color, name)
            }
            ObjectKind::Operator(Operator::SetColorSpaceStroke) => match &self.peek()?.kind {
                ObjectKind::Name(name) => op!(Operation::SetColorSpaceStroke(name)),
                _ => return None,
            },
            ObjectKind::Operator(Operator::SetColorSpaceNonstroke) => match &self.peek()?.kind {
                ObjectKind::Name(name) => op!(Operation::SetColorSpaceNonstroke(name)),
                _ => return None,
            },
            ObjectKind::Operator(Operator::SetRGBColorStroke) => {
                let blue = coerce_f64!(&self.peek()?.kind);
                let green = coerce_f64!(&self.nth(1)?.kind);
                let red = coerce_f64!(&self.nth(2)?.kind);
                let rgb = Rgb { red, green, blue };
                op!(Operation::SetRGBColorStroke(rgb), 3)
            }
            ObjectKind::Operator(Operator::SetRGBColorNonstroke) => {
                let blue = coerce_f64!(&self.peek()?.kind);
                let green = coerce_f64!(&self.nth(1)?.kind);
                let red = coerce_f64!(&self.nth(2)?.kind);
                let rgb = Rgb { red, green, blue };
                op!(Operation::SetRGBColorNonstroke(rgb), 3)
            }
            ObjectKind::Operator(Operator::SetGrayStroke) => {
                let gray = coerce_f64!(&self.peek()?.kind);
                op!(Operation::SetGrayStroke(gray))
            }
            ObjectKind::Operator(Operator::SetGrayNonstroke) => {
                let gray = coerce_f64!(&self.peek()?.kind);
                op!(Operation::SetGrayNonstroke(gray))
            }
            ObjectKind::Operator(Operator::ShFill) => match &self.peek()?.kind {
                ObjectKind::Name(name) => op!(Operation::ShFill(name)),
                _ => return None,
            },
            _ => return None,
        };
        Some(operand)
    }

    fn handle_color(&mut self, start: usize) -> Option<Color> {
        let first = coerce_f64!(&self.nth(start)?.kind);
        let second = self.nth(start + 1);
        if matches!(
            second,
            Some(Object {
                kind: ObjectKind::Integer(_),
                ..
            }) | Some(Object {
                kind: ObjectKind::Real(_),
                ..
            })
        ) {
            let second = coerce_f64!(&self.nth(start + 1)?.kind);
            let third = match self.nth(start + 2) {
                Some(Object {
                    kind: ObjectKind::Real(r),
                    ..
                }) => *r,
                Some(Object {
                    kind: ObjectKind::Integer(i),
                    ..
                }) => *i as f64,
                _ => {
                    self.seek(start + 2);
                    return Some(Color::Gray(first));
                }
            };
            let fourth = match self.nth(start + 3) {
                Some(Object {
                    kind: ObjectKind::Real(r),
                    ..
                }) => *r,
                Some(Object {
                    kind: ObjectKind::Integer(i),
                    ..
                }) => *i as f64,
                _ => {
                    self.seek(start + 3);
                    let rgb = Rgb {
                        red: third,
                        green: second,
                        blue: first,
                    };
                    return Some(Color::Rgb(rgb));
                }
            };
            let cmyk = Cmyk {
                cyan: fourth,
                magenta: third,
                yellow: second,
                black: first,
            };
            self.seek(start + 4);
            Some(Color::Cmyk(cmyk))
        } else {
            self.seek(start + 1);
            Some(Color::Gray(first))
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
mod tests {
    use crate::parser::object::Name;
    use crate::{array, dict, integer, name, offset, operator, real, string};

    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_empty() {
        let objects = vec![];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_no_args() {
        let objects = vec![operator!(Operator::CloseStrokePath)];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![Operation::CloseStrokePath];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_too_few_args() {
        let objects = vec![operator!(Operator::SetGrayStroke)];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_one_arg_int() {
        let objects = vec![integer!(0), operator!(Operator::SetGrayStroke)];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![Operation::SetGrayStroke(0.0)];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_one_arg_real() {
        let objects = vec![real!(0.5), operator!(Operator::SetGrayNonstroke)];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![Operation::SetGrayNonstroke(0.5)];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_one_arg_name() {
        let objects = vec![name!("test"), operator!(Operator::DefineMarkedContentPoint)];
        let mut parser = OperatorParser::new(&objects);
        let name = Name(b"test".to_vec());
        let operations = vec![Operation::DefineMarkedContentPoint(&name)];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_two_args_name_dict() {
        let mut properties = BTreeMap::new();
        properties.insert(b"Length".to_vec(), ObjectKind::Integer(6));
        let objects = vec![
            name!("test"),
            dict!(
                b"Length" => integer!(6)
            ),
            operator!(Operator::DefineMarkedContentPointPropertyList),
        ];
        let mut parser = OperatorParser::new(&objects);
        let key = b"Length".to_vec();
        let result = parser.parse();
        assert!(matches!(
            result[0],
            Operation::DefineMarkedContentPointPropertyList(..)
        ));
        let length = match result[0] {
            Operation::DefineMarkedContentPointPropertyList(_, dict) => dict.get(&key),
            _ => None,
        };
        assert!(matches!(
            length,
            Some(&Object {
                kind: ObjectKind::Integer(6),
                ..
            })
        ));
        let name = match result[0] {
            Operation::DefineMarkedContentPointPropertyList(n, _) => Some(n),
            _ => None,
        };
        let tag = b"test".to_vec();
        assert_eq!(name.unwrap(), &tag);
    }

    #[test]
    fn test_three_args_real() {
        let objects = vec![
            real!(1.0),
            real!(1.0),
            real!(1.0),
            operator!(Operator::SetRGBColorStroke),
        ];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![Operation::SetRGBColorStroke(Rgb {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        })];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_three_args_too_few() {
        let objects = vec![
            offset!(ObjectKind::Null),
            real!(1.0),
            real!(1.0),
            operator!(Operator::SetRGBColorStroke),
        ];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_cmyk() {
        let objects = vec![
            real!(1.0),
            integer!(0),
            real!(1.0),
            real!(0.0),
            operator!(Operator::SetColorStroke),
        ];
        let mut parser = OperatorParser::new(&objects);
        let color = Cmyk {
            cyan: 1.0,
            magenta: 0.0,
            yellow: 1.0,
            black: 0.0,
        };
        let operations = vec![Operation::SetColorStroke(Color::Cmyk(color))];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_rgb() {
        let objects = vec![
            real!(0.3),
            integer!(0),
            real!(0.7),
            operator!(Operator::SetColorStroke),
        ];
        let mut parser = OperatorParser::new(&objects);
        let color = Rgb {
            red: 0.3,
            green: 0.0,
            blue: 0.7,
        };
        let operations = vec![Operation::SetColorStroke(Color::Rgb(color))];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_gray() {
        let objects = vec![real!(0.7), operator!(Operator::SetColorStroke)];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![Operation::SetColorStroke(Color::Gray(0.7))];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_pattern_name_cmyk() {
        let objects = vec![
            real!(1.0),
            integer!(0),
            real!(1.0),
            real!(0.0),
            name!("COLOR"),
            operator!(Operator::SetColorSpecialStroke),
        ];
        let mut parser = OperatorParser::new(&objects);
        let color = Cmyk {
            cyan: 1.0,
            magenta: 0.0,
            yellow: 1.0,
            black: 0.0,
        };
        let name = Name(b"COLOR".to_vec());
        let operations = vec![Operation::SetColorSpecialStroke(
            Color::Cmyk(color),
            Some(&name),
        )];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_pattern_name_rgb() {
        let objects = vec![
            real!(0.3),
            integer!(0),
            real!(0.7),
            name!("COLOR"),
            operator!(Operator::SetColorSpecialStroke),
        ];
        let mut parser = OperatorParser::new(&objects);
        let color = Rgb {
            red: 0.3,
            green: 0.0,
            blue: 0.7,
        };
        let name = Name(b"COLOR".to_vec());
        let operations = vec![Operation::SetColorSpecialStroke(
            Color::Rgb(color),
            Some(&name),
        )];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_pattern_name_gray() {
        let objects = vec![
            real!(0.7),
            name!("COLOR"),
            operator!(Operator::SetColorSpecialStroke),
        ];
        let mut parser = OperatorParser::new(&objects);
        let name = Name(b"COLOR".to_vec());
        let operations = vec![Operation::SetColorSpecialStroke(
            Color::Gray(0.7),
            Some(&name),
        )];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_pattern_cmyk() {
        let objects = vec![
            real!(1.0),
            integer!(0),
            real!(1.0),
            real!(0.0),
            operator!(Operator::SetColorSpecialStroke),
        ];
        let mut parser = OperatorParser::new(&objects);
        let color = Cmyk {
            cyan: 1.0,
            magenta: 0.0,
            yellow: 1.0,
            black: 0.0,
        };
        let operations = vec![Operation::SetColorSpecialStroke(Color::Cmyk(color), None)];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_pattern_rgb() {
        let objects = vec![
            real!(0.3),
            integer!(0),
            real!(0.7),
            operator!(Operator::SetColorSpecialStroke),
        ];
        let mut parser = OperatorParser::new(&objects);
        let color = Rgb {
            red: 0.3,
            green: 0.0,
            blue: 0.7,
        };
        let operations = vec![Operation::SetColorSpecialStroke(Color::Rgb(color), None)];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_color_pattern_gray() {
        let objects = vec![real!(0.7), operator!(Operator::SetColorSpecialStroke)];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![Operation::SetColorSpecialStroke(Color::Gray(0.7), None)];
        assert_eq!(parser.parse(), operations);
    }

    #[test]
    fn test_show_text_adjusted_string_number() {
        // [(Le)15(x)-250(Fridman)]TJ
        let le = b"Le".to_vec();
        let x = b"x".to_vec();
        let fridman = b"Fridman".to_vec();
        let array = array![
            string!("Le"),
            integer!(15),
            string!("x"),
            integer!(-250),
            string!("Fridman"),
        ];
        let objects = vec![array, operator!(Operator::ShowTextAdjusted)];
        let mut parser = OperatorParser::new(&objects);
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

    #[test]
    fn test_select_font() {
        // /F13 6.9738 Tf
        let name = Name(b"F13".to_vec());
        let objects = vec![name!("F13"), real!(6.9738), operator!(Operator::SelectFont)];
        let mut parser = OperatorParser::new(&objects);
        let operations = vec![Operation::SelectFont(&name, 6.9738)];
        assert_eq!(parser.parse(), operations);
    }
}
