use super::cid_parser::CIDOperator;
use crate::operators::operators::Operator;
use crate::util::{
    byte_to_numeric, is_newline, is_regular, is_whitespace, Byte, DQUOTE, FSLASH, LBRACE, LBRACKET,
    LINE_FEED, LPAREN, LTHAN, PERCENT, PERIOD, RBRACE, RBRACKET, RPAREN, RTHAN, SQUOTE,
};

enum Character {
    Regular(Byte),
    Delimiter(Byte),
    Whitespace(Byte),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Header {
    pub major: u8,
    pub minor: u8,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind<'a> {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    Name(&'a [u8]),
    LiteralString(&'a [u8]),
    HexString(&'a [u8]),
    DoubleLThan,
    DoubleRThan,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Header(Header),
    Reference,
    Obj,
    Endobj,
    Xref,
    StartXref,
    Stream(&'a [u8]),
    Null,
    Trailer,
    F,
    N,
    Operator(Operator),
    CIDOperator(CIDOperator),
    Eof,
}

#[derive(Debug)]
#[cfg_attr(not(test), derive(PartialEq))]
pub struct Token<'a> {
    pub(crate) offset: u64,
    pub(crate) kind: TokenKind<'a>,
}

// We do not want to test if the offsets are equal during testing so we don't
// to specify the offsets everywhere when they are not relevant.
#[cfg(test)]
impl PartialEq for Token<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[cfg(test)]
impl PartialEq<TokenKind<'_>> for Token<'_> {
    fn eq(&self, other: &TokenKind) -> bool {
        self.kind == *other
    }
}

pub struct Lexer<'a> {
    pos: usize,
    len: usize,
    buf: &'a [Byte],
}

impl<'a> Lexer<'a> {
    pub fn new(buf: &'a [Byte]) -> Self {
        Self {
            pos: 0,
            len: buf.len(),
            buf,
        }
    }

    pub fn lex(&mut self) -> Vec<Token<'a>> {
        let mut vec = vec![];
        while self.pos < self.len {
            if let Some(token) = self.next() {
                vec.push(token);
            }
        }
        vec
    }

    fn next(&mut self) -> Option<Token<'a>> {
        // Yes defining a macro inside a function is very ugly but this way we
        // can use `self` without problems
        macro_rules! op {
            ($op:expr) => {{
                self.advance();
                TokenKind::Operator($op)
            }};
            ($op:expr, $len:expr) => {{
                self.seek($len);
                TokenKind::Operator($op)
            }};
        }
        macro_rules! cid {
            ($op:expr) => {{
                self.advance();
                TokenKind::CIDOperator($op)
            }};
            ($op:expr, $len:expr) => {{
                self.seek($len);
                TokenKind::CIDOperator($op)
            }};
        }

        let curr = self.peek()?;
        if is_whitespace(curr) {
            self.skip_whitespace();
        }
        let offset = self.pos as u64;
        let curr = self.pop()?;
        let kind: TokenKind = match curr {
            LTHAN => match self.peek() {
                Some(LTHAN) => {
                    self.advance();
                    TokenKind::DoubleLThan
                }
                _ => self.hex_string()?,
            },
            RTHAN => match self.peek()? {
                RTHAN => {
                    self.advance();
                    TokenKind::DoubleRThan
                }
                _ => return None,
            },
            b'b' => match self.peek() {
                Some(b'*') => op!(Operator::CloseFillStrokePathEvenOdd),
                Some(b'e') => match self.pop_slice(self.pos, 4)? {
                    b"egin" => self.begin()?,
                    _ => return None,
                },
                _ => TokenKind::Operator(Operator::CloseFillStrokePath),
            },
            b'B' => match self.peek() {
                Some(b'*') => op!(Operator::FillStrokePathEvenOdd),
                Some(b'I') => op!(Operator::BeginInlineImageObject),
                Some(b'T') => op!(Operator::BeginText),
                Some(b'X') => op!(Operator::BeginCompat),
                Some(b'M') => match self.peek_next()? {
                    b'C' => op!(Operator::BeginMarkedContentSequence, 2),
                    _ => return None,
                },
                Some(b'D') => match self.peek_next()? {
                    b'C' => op!(Operator::BeginMarkedContentSequencePropertyList, 2),
                    _ => return None,
                },
                _ => TokenKind::Operator(Operator::FillStrokePath),
            },
            b'c' => match self.peek() {
                Some(b'm') => op!(Operator::ConcatMatrix),
                Some(b's') => op!(Operator::SetColorSpaceNonstroke),
                _ => TokenKind::Operator(Operator::AppendCurveThreePoints),
            },
            b'C' => match self.peek()? {
                b'S' => op!(Operator::SetColorSpaceStroke),
                _ => return None,
            },
            b'd' => match self.peek() {
                Some(b'0') => op!(Operator::SetCharWidth),
                Some(b'1') => op!(Operator::SetCacheDevice),
                Some(b'i') => match self.pop_slice(self.pos, 3)? {
                    b"ict" => TokenKind::CIDOperator(CIDOperator::Dict),
                    _ => return None,
                },
                Some(b'e') => match self.pop_slice(self.pos, 2)? {
                    b"ef" => TokenKind::CIDOperator(CIDOperator::Def),
                    _ => return None,
                },
                Some(b'u') => match self.pop_slice(self.pos, 2)? {
                    b"up" => TokenKind::CIDOperator(CIDOperator::Dup),
                    _ => return None,
                },
                _ => TokenKind::Operator(Operator::SetDash),
            },
            b'D' => match self.peek()? {
                b'o' => op!(Operator::InvokeXObject),
                b'P' => op!(Operator::DefineMarkedContentPointPropertyList),
                _ => return None,
            },
            b'e' => match self.pop_slice(self.pos, 2)? {
                b"nd" => self.end()?,
                _ => return None,
            },
            b'E' => match self.peek()? {
                b'I' => op!(Operator::EndInlineImage),
                b'T' => op!(Operator::EndText),
                b'X' => op!(Operator::EndCompat),
                b'M' => match self.peek_next()? {
                    b'C' => op!(Operator::EndMarkedContentSequence, 2),
                    _ => return None,
                },
                _ => return None,
            },
            b'f' => match self.peek() {
                Some(b'*') => op!(Operator::FillPathEvenOdd),
                Some(b'a') => match self.pop_slice(self.pos, 4)? {
                    b"alse" => TokenKind::Boolean(false),
                    _ => return None,
                },
                Some(b'i') => match self.peek_slice(self.pos, 11)? {
                    b"indresource" => cid!(CIDOperator::FindResource, 12),
                    _ => return None,
                },
                _ => TokenKind::F,
            },
            b'F' => TokenKind::Operator(Operator::FillPath),
            b'g' => match self.peek() {
                Some(b's') => op!(Operator::SetGraphicsStateParams),
                _ => TokenKind::Operator(Operator::SetGrayNonstroke),
            },
            b'G' => TokenKind::Operator(Operator::SetGrayStroke),
            b'h' => TokenKind::Operator(Operator::CloseSubpath),
            b'i' => TokenKind::Operator(Operator::SetFlat),
            b'I' => match self.peek()? {
                b'D' => op!(Operator::BeginInlineImageData),
                _ => return None,
            },
            b'j' => TokenKind::Operator(Operator::SetLineJoin),
            b'J' => TokenKind::Operator(Operator::SetLineCap),
            b'k' => TokenKind::Operator(Operator::SetCMYKColorNonstroke),
            b'K' => TokenKind::Operator(Operator::SetCMYKColorStroke),
            b'l' => TokenKind::Operator(Operator::LineTo),
            b'm' => TokenKind::Operator(Operator::MoveTo),
            b'M' => match self.peek() {
                Some(b'P') => op!(Operator::DefineMarkedContentPoint),
                _ => TokenKind::Operator(Operator::SetMiterLimit),
            },
            b'n' => match self.peek_slice(self.pos, 3) {
                Some(b"ull") => {
                    self.seek(3);
                    TokenKind::Null
                }
                _ => TokenKind::N,
            },
            b'o' => match self.peek_slice(self.pos, 2)? {
                b"bj" => {
                    self.seek(2);
                    TokenKind::Obj
                }
                _ => return None,
            },
            b'q' => TokenKind::Operator(Operator::GSave),
            b'Q' => TokenKind::Operator(Operator::GRestore),
            b'r' => match self.peek()? {
                b'e' => op!(Operator::AppendRectangle),
                b'g' => op!(Operator::SetRGBColorNonstroke),
                b'i' => op!(Operator::SetColorRenderingIntent),
                _ => return None,
            },
            b'R' => match self.peek() {
                Some(b'G') => op!(Operator::SetRGBColorStroke),
                _ => TokenKind::Reference,
            },
            b's' => match self.peek() {
                Some(b'c') => match self.peek_next() {
                    Some(b'n') => op!(Operator::SetColorSpecialNonstroke, 2),
                    _ => op!(Operator::SetColorNonstroke),
                },
                Some(b't') => {
                    self.advance();
                    match self.pop()? {
                        b'r' => self.stream()?,
                        b'a' => self.startxref()?,
                        _ => return None,
                    }
                }
                Some(b'h') => op!(Operator::ShFill),
                _ => TokenKind::Operator(Operator::CloseStrokePath),
            },
            b'S' => match self.peek() {
                Some(b'C') => match self.peek_next() {
                    Some(b'N') => op!(Operator::SetColorSpecialStroke, 2),
                    _ => op!(Operator::SetColorStroke),
                },
                _ => TokenKind::Operator(Operator::StrokePath),
            },
            b't' => match self.pop_slice(self.pos, 3)? {
                b"rue" => TokenKind::Boolean(true),
                b"rai" => match self.pop_slice(self.pos, 3)? {
                    b"ler" => TokenKind::Trailer,
                    _ => return None,
                },
                _ => return None,
            },
            b'T' => match self.peek()? {
                b'*' => op!(Operator::MoveStartNextLine),
                b'c' => op!(Operator::SetCharSpacing),
                b'd' => op!(Operator::MoveTextPosition),
                b'D' => op!(Operator::MoveTextPositionLeading),
                b'f' => op!(Operator::SelectFont),
                b'j' => op!(Operator::ShowText),
                b'J' => op!(Operator::ShowTextAdjusted),
                b'L' => op!(Operator::SetTextLeading),
                b'm' => op!(Operator::SetTextMatrix),
                b'r' => op!(Operator::SetTextRendering),
                b's' => op!(Operator::SetTextRise),
                b'w' => op!(Operator::SetWordSpacing),
                b'z' => op!(Operator::SetHorizontalTextScaling),
                _ => return None,
            },
            b'u' => match self.peek_slice(self.pos, 6)? {
                b"secmap" => cid!(CIDOperator::UseCMap, 6),
                b"sefont" => cid!(CIDOperator::UseFont, 6),
                _ => return None,
            },
            b'v' => TokenKind::Operator(Operator::AppendCurveInitialReplicated),
            b'w' => TokenKind::Operator(Operator::SetLineWidth),
            b'W' => match self.peek() {
                Some(b'*') => op!(Operator::SetClippingPathEvenOdd),
                _ => TokenKind::Operator(Operator::SetClippingPath),
            },
            b'x' => match self.pop_slice(self.pos, 3)? {
                b"ref" => TokenKind::Xref,
                _ => return None,
            },
            b'y' => TokenKind::Operator(Operator::AppendCurveFinalReplicated),
            b'0'..=b'9' | b'+' | b'-' | PERIOD => self.number()?,
            PERCENT => self.percent()?,
            LPAREN => self.literal_string()?,
            LBRACKET => TokenKind::LBracket,
            RBRACKET => TokenKind::RBracket,
            LBRACE => TokenKind::LBrace,
            RBRACE => TokenKind::RBrace,
            FSLASH => self.name()?,
            SQUOTE => TokenKind::Operator(Operator::MoveNextLineShowText),
            DQUOTE => TokenKind::Operator(Operator::SetSpacingMoveNextLineShowText),
            _ => return None,
        };
        Some(Token { offset, kind })
    }

    fn number(&mut self) -> Option<TokenKind<'a>> {
        let start = self.pos - 1;
        let mut is_real = matches!(self.buf.get(start), Some(&b) if b == PERIOD);
        loop {
            match self.peek() {
                Some(c) if c.is_ascii_digit() => self.advance(),
                Some(PERIOD) => {
                    self.advance();
                    is_real = true
                }
                _ => break,
            }
        }
        let buf = &(self.buf[start..self.pos]);
        if is_real {
            let number = std::str::from_utf8(buf).ok().and_then(|x| x.parse().ok())?;
            Some(TokenKind::Real(number))
        } else {
            let number = std::str::from_utf8(buf).ok().and_then(|x| x.parse().ok())?;
            Some(TokenKind::Integer(number))
        }
    }

    fn begin(&mut self) -> Option<TokenKind<'a>> {
        macro_rules! cid {
            ($op:path, $len:expr) => {{
                self.seek($len);
                TokenKind::CIDOperator($op)
            }};
        }
        let kind = match self.peek() {
            Some(b'b') => match self.peek_slice(self.pos, 6)? {
                b"bfchar" => cid!(CIDOperator::BeginBfChar, 6),
                b"bfrang" => match self.nth(6)? {
                    b'e' => cid!(CIDOperator::BeginBfRange, 7),
                    _ => return None,
                },
                _ => TokenKind::CIDOperator(CIDOperator::Begin),
            },
            Some(b'n') => match self.peek_slice(self.pos, 10)? {
                b"notdefchar" => cid!(CIDOperator::BeginNotdefChar, 10),
                b"notdefrang" => match self.nth(10)? {
                    b'e' => cid!(CIDOperator::BeginNotdefRange, 11),
                    _ => return None,
                },
                _ => TokenKind::CIDOperator(CIDOperator::Begin),
            },
            Some(b'c') => match self.peek_slice(self.pos, 4)? {
                b"cmap" => cid!(CIDOperator::BeginCMap, 4),
                b"cidc" => match self.peek_slice(self.pos + 4, 3)? {
                    b"har" => cid!(CIDOperator::BeginCIDChar, 7),
                    _ => return None,
                },
                b"cidr" => match self.peek_slice(self.pos + 4, 4)? {
                    b"ange" => cid!(CIDOperator::BeginCIDRange, 8),
                    _ => return None,
                },
                b"code" => match self.peek_slice(self.pos + 4, 10)? {
                    b"spacerange" => cid!(CIDOperator::BeginCodeSpaceRange, 14),
                    _ => return None,
                },
                _ => TokenKind::CIDOperator(CIDOperator::Begin),
            },
            _ => TokenKind::CIDOperator(CIDOperator::Begin),
        };
        Some(kind)
    }

    fn startxref(&mut self) -> Option<TokenKind<'a>> {
        let string = "rtxref".as_bytes();
        let token = match self.peek_slice(self.pos, string.len()) {
            Some(x) if x == string => {
                self.seek(string.len());
                TokenKind::StartXref
            }
            _ => return None,
        };
        Some(token)
    }

    fn end(&mut self) -> Option<TokenKind<'a>> {
        macro_rules! cid {
            ($op:path, $len:expr) => {{
                self.seek($len);
                TokenKind::CIDOperator($op)
            }};
        }
        let kind = match self.peek() {
            Some(b'b') => match self.peek_slice(self.pos, 6)? {
                b"bfchar" => cid!(CIDOperator::EndBfChar, 6),
                b"brrang" => match self.nth(6)? {
                    b'e' => cid!(CIDOperator::EndBfRange, 7),
                    _ => return None,
                },
                _ => TokenKind::CIDOperator(CIDOperator::End),
            },
            Some(b'c') => match self.peek_slice(self.pos, 4)? {
                b"cidc" => match self.peek_slice(self.pos + 4, 3)? {
                    b"har" => cid!(CIDOperator::EndCIDChar, 7),
                    _ => return None,
                },
                b"cidr" => match self.peek_slice(self.pos + 4, 4)? {
                    b"ange" => cid!(CIDOperator::EndCIDRange, 8),
                    _ => return None,
                },
                b"cmap" => cid!(CIDOperator::EndCMap, 4),
                b"code" => match self.peek_slice(self.pos + 4, 5)? {
                    b"space" => cid!(CIDOperator::EndCodeSpaceRange, 9),
                    _ => return None,
                },
                _ => TokenKind::CIDOperator(CIDOperator::End),
            },
            Some(b'n') => match self.peek_slice(self.pos, 10)? {
                b"notdefchar" => cid!(CIDOperator::EndNotdefChar, 10),
                b"notdefrang" => match self.nth(11)? {
                    b'e' => cid!(CIDOperator::EndNotdefRange, 11),
                    _ => return None,
                },
                _ => TokenKind::CIDOperator(CIDOperator::End),
            },
            Some(b'o') => match self.pop_slice(self.pos, 3)? {
                b"obj" => TokenKind::Endobj,
                _ => TokenKind::CIDOperator(CIDOperator::End),
            },
            _ => TokenKind::CIDOperator(CIDOperator::End),
        };
        Some(kind)
    }

    fn stream(&mut self) -> Option<TokenKind<'a>> {
        let stream_string = "eam".as_bytes();
        let endstream_string = "endstream".as_bytes();
        match self.peek_slice(self.pos, stream_string.len()) {
            Some(x) if x == stream_string => {
                // PDF32000_2008 7.3.8.1 paragraph 5
                self.get_next_char_while(|c| c != LINE_FEED);
                // Skip newline character
                self.advance();
                let start = self.pos;
                loop {
                    self.skip_whitespace();
                    match self.peek_slice(self.pos, endstream_string.len()) {
                        Some(y) if y == endstream_string => {
                            let end = self.pos;
                            self.seek(endstream_string.len());
                            return Some(TokenKind::Stream(&self.buf[start..end]));
                        }
                        Some(_) => {
                            self.advance();
                        }
                        None => {
                            return None;
                        }
                    }
                }
            }
            _ => None,
        }
    }

    fn hex_string(&mut self) -> Option<TokenKind<'a>> {
        // PDF spec 7.3.4.3 - White space shall be ignored
        let string = self.get_next_char_while(|c| c.is_ascii_hexdigit() || is_whitespace(c));
        Some(TokenKind::HexString(string))
    }

    fn literal_string(&mut self) -> Option<TokenKind<'a>> {
        // Start at 1 because we have already seen first LPAREN
        let mut depth: u32 = 1;
        let start = self.pos;
        while self.pos < self.len {
            let curr = self.pop()?;
            if curr == LPAREN {
                depth += 1;
            } else if curr == RPAREN {
                depth -= 1;
                // End of string
                if depth == 0 {
                    return Some(TokenKind::LiteralString(&self.buf[start..self.pos - 1]));
                }
            }
        }
        None
    }

    fn name(&mut self) -> Option<TokenKind<'a>> {
        let start = self.pos;
        loop {
            match self.peek() {
                Some(c) if is_whitespace(c) => {
                    break;
                }
                Some(FSLASH) => {
                    break;
                }
                Some(c) if is_regular(c) => {
                    self.advance();
                    continue;
                }
                _ => break,
            };
        }
        Some(TokenKind::Name(&self.buf[start..self.pos]))
    }

    fn eof(&mut self) -> Option<TokenKind<'a>> {
        let string = "EOF".as_bytes();
        let token = match self.peek_slice(self.pos, string.len()) {
            Some(x) if x == string => {
                self.seek(string.len());
                TokenKind::Eof
            }
            _ => {
                self.next_line();
                return None;
            }
        };
        Some(token)
    }

    fn header_version(&mut self) -> Option<TokenKind<'a>> {
        let token = match (
            self.nth(1),
            self.nth(2),
            self.nth(3),
            self.nth(4),
            self.nth(5),
            self.nth(6),
        ) {
            (Some(b'D'), Some(b'F'), Some(b'-'), Some(m), Some(b'.'), Some(n))
                if m.is_ascii_digit() && n.is_ascii_digit() =>
            {
                // Skip past "DF-m.n"
                self.seek(7);
                let major = byte_to_numeric(m, 10)?;
                let minor = byte_to_numeric(n, 10)?;
                TokenKind::Header(Header { major, minor })
            }
            _ => {
                self.next_line();
                return None;
            }
        };
        Some(token)
    }

    fn percent(&mut self) -> Option<TokenKind<'a>> {
        let kind = match self.peek() {
            Some(PERCENT) => {
                self.advance();
                self.eof()?
            }
            Some(b'P') => self.header_version()?,
            _ => {
                self.next_line();
                return None;
            }
        };
        Some(kind)
    }

    fn advance(&mut self) {
        self.seek(1)
    }

    fn pop(&mut self) -> Option<Byte> {
        let prev = self.peek();
        self.advance();
        prev
    }

    fn nth(&self, n: usize) -> Option<Byte> {
        if self.pos < self.len - n {
            Some(self.buf[self.pos + n])
        } else {
            None
        }
    }

    fn peek(&self) -> Option<Byte> {
        self.nth(0)
    }

    fn peek_next(&self) -> Option<Byte> {
        self.nth(1)
    }

    fn get_next_char_while(&mut self, cond: fn(Byte) -> bool) -> &'a [Byte] {
        let start = self.pos;
        loop {
            match self.peek() {
                Some(c) if cond(c) => {
                    self.advance();
                }
                _ => {
                    return &self.buf[start..self.pos];
                }
            }
        }
    }

    fn seek(&mut self, n: usize) {
        if self.pos <= self.len - n {
            self.pos += n
        }
    }

    fn seek_start(&mut self) {
        self.pos = 0
    }

    fn seek_end(&mut self) {
        self.pos = self.len - 1
    }

    fn peek_slice(&self, start: usize, length: usize) -> Option<&'a [Byte]> {
        if start + length > self.len {
            None
        } else {
            Some(&self.buf[start..start + length])
        }
    }

    fn pop_slice(&mut self, start: usize, length: usize) -> Option<&'a [Byte]> {
        if let Some(slice) = self.peek_slice(start, length) {
            self.seek(length);
            Some(slice)
        } else {
            None
        }
    }

    fn skip_while(&mut self, predicate: impl Fn(Byte) -> bool) {
        loop {
            if let Some(c) = self.peek() {
                if predicate(c) && self.pos < self.len {
                    self.advance();
                } else {
                    return;
                }
            } else {
                return;
            }
        }
    }

    fn skip_until(&mut self, predicate: impl Fn(Byte) -> bool) {
        loop {
            if let Some(c) = self.peek() {
                if !predicate(c) && self.pos < self.len {
                    self.advance();
                } else {
                    return;
                }
            } else {
                return;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        self.skip_while(is_whitespace)
    }

    fn next_line(&mut self) {
        self.skip_until(is_newline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        let text = "null
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let kinds = vec![TokenKind::Null, TokenKind::Eof];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_real_leading() {
        let text = "0.75
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let kinds = vec![TokenKind::Real(0.75), TokenKind::Eof];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_real_no_leading() {
        let text = ".75
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let kinds = vec![TokenKind::Real(0.75), TokenKind::Eof];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_hex_string_with_space() {
        let text = b"<0644 0627>\n%%EOF";
        let mut lexer = Lexer::new(text);
        let kinds = vec![TokenKind::HexString(b"0644 0627"), TokenKind::Eof];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_dict_with_boolean_value() {
        let text = b"12 0 obj
        <<\n/Type /ExtGState\n/SA false\n>>\nendobj
        %%EOF";
        let mut lexer = Lexer::new(text);
        let kinds = vec![
            TokenKind::Integer(12),
            TokenKind::Integer(0),
            TokenKind::Obj,
            TokenKind::DoubleLThan,
            TokenKind::Name(b"Type"),
            TokenKind::Name(b"ExtGState"),
            TokenKind::Name(b"SA"),
            TokenKind::Boolean(false),
            TokenKind::DoubleRThan,
            TokenKind::Endobj,
            TokenKind::Eof,
        ];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_scn_op() {
        let text = "sscshscnf
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let kinds = vec![
            TokenKind::Operator(Operator::CloseStrokePath),
            TokenKind::Operator(Operator::SetColorNonstroke),
            TokenKind::Operator(Operator::ShFill),
            TokenKind::Operator(Operator::SetColorSpecialNonstroke),
            TokenKind::F,
            TokenKind::Eof,
        ];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_cs_op() {
        let text = "cscmc5cs
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let kinds = vec![
            TokenKind::Operator(Operator::SetColorSpaceNonstroke),
            TokenKind::Operator(Operator::ConcatMatrix),
            TokenKind::Operator(Operator::AppendCurveThreePoints),
            TokenKind::Integer(5),
            TokenKind::Operator(Operator::SetColorSpaceNonstroke),
            TokenKind::Eof,
        ];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_capital_t_op() {
        let text = "TcTdTTsf
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let kinds = vec![
            TokenKind::Operator(Operator::SetCharSpacing),
            TokenKind::Operator(Operator::MoveTextPosition),
            TokenKind::Operator(Operator::SetTextRise),
            TokenKind::F,
            TokenKind::Eof,
        ];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_capital_e_op() {
        let text = "EIEMCTfETEXEEI5
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let kinds = vec![
            TokenKind::Operator(Operator::EndInlineImage),
            TokenKind::Operator(Operator::EndMarkedContentSequence),
            TokenKind::Operator(Operator::SelectFont),
            TokenKind::Operator(Operator::EndText),
            TokenKind::Operator(Operator::EndCompat),
            TokenKind::Operator(Operator::EndInlineImage),
            TokenKind::Integer(5),
            TokenKind::Eof,
        ];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_cmap_begin() {
        let text = "begin12beginbegincmapendcmapendend";
        let mut lexer = Lexer::new(text.as_bytes());
        let kinds = vec![
            TokenKind::CIDOperator(CIDOperator::Begin),
            TokenKind::Integer(12),
            TokenKind::CIDOperator(CIDOperator::Begin),
            TokenKind::CIDOperator(CIDOperator::BeginCMap),
            TokenKind::CIDOperator(CIDOperator::EndCMap),
            TokenKind::CIDOperator(CIDOperator::End),
            TokenKind::CIDOperator(CIDOperator::End),
        ];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_cmap() {
        let text = "%!PS-Adobe-3.0 Resource-CMap
        /CIDInit /ProcSet findresource begin
        12 dict begin
        begincmap
        /CIDSystemInfo 3 dict dup begin
        /Registry (Adobe) def
        /Ordering (Japan1) def
        /Supplement 0 def
        end def
        /Ext-RKSJ-H usecmap
        /CMapName /Ext-RKSJ-V def
        /CMapVersion 1 def
        /CMapType 0 def
        /UIDOffset 800 def
        /XUID [1 10 25316] def
        /WMode 1 def
        1 begincidrange
        <8141> <8142> 7887
        endcidrange
        5 begincidchar
        <8143> 8286
        <8144> 8274
        <814a> 8272
        <8387> 7936
        <838e> 7937
        endcidchar
        endcmap
        end
        end
        %%EOF
        ";
        let mut lexer = Lexer::new(text.as_bytes());
        let kinds = vec![
            TokenKind::Name(b"CIDInit"),
            TokenKind::Name(b"ProcSet"),
            TokenKind::CIDOperator(CIDOperator::FindResource),
            TokenKind::CIDOperator(CIDOperator::Begin),
            TokenKind::Integer(12),
            TokenKind::CIDOperator(CIDOperator::Dict),
            TokenKind::CIDOperator(CIDOperator::Begin),
            TokenKind::CIDOperator(CIDOperator::BeginCMap),
            TokenKind::Name(b"CIDSystemInfo"),
            TokenKind::Integer(3),
            TokenKind::CIDOperator(CIDOperator::Dict),
            TokenKind::CIDOperator(CIDOperator::Dup),
            TokenKind::CIDOperator(CIDOperator::Begin),
            TokenKind::Name(b"Registry"),
            TokenKind::LiteralString(b"Adobe"),
            TokenKind::CIDOperator(CIDOperator::Def),
            TokenKind::Name(b"Ordering"),
            TokenKind::LiteralString(b"Japan1"),
            TokenKind::CIDOperator(CIDOperator::Def),
            TokenKind::Name(b"Supplement"),
            TokenKind::Integer(0),
            TokenKind::CIDOperator(CIDOperator::Def),
            TokenKind::CIDOperator(CIDOperator::End),
            TokenKind::CIDOperator(CIDOperator::Def),
            TokenKind::Name(b"Ext-RKSJ-H"),
            TokenKind::CIDOperator(CIDOperator::UseCMap),
            TokenKind::Name(b"CMapName"),
            TokenKind::Name(b"Ext-RKSJ-V"),
            TokenKind::CIDOperator(CIDOperator::Def),
            TokenKind::Name(b"CMapVersion"),
            TokenKind::Integer(1),
            TokenKind::CIDOperator(CIDOperator::Def),
            TokenKind::Name(b"CMapType"),
            TokenKind::Integer(0),
            TokenKind::CIDOperator(CIDOperator::Def),
            TokenKind::Name(b"UIDOffset"),
            TokenKind::Integer(800),
            TokenKind::CIDOperator(CIDOperator::Def),
            TokenKind::Name(b"XUID"),
            TokenKind::LBracket,
            TokenKind::Integer(1),
            TokenKind::Integer(10),
            TokenKind::Integer(25316),
            TokenKind::RBracket,
            TokenKind::CIDOperator(CIDOperator::Def),
            TokenKind::Name(b"WMode"),
            TokenKind::Integer(1),
            TokenKind::CIDOperator(CIDOperator::Def),
            TokenKind::Integer(1),
            TokenKind::CIDOperator(CIDOperator::BeginCIDRange),
            TokenKind::HexString(b"8141"),
            TokenKind::HexString(b"8142"),
            TokenKind::Integer(7887),
            TokenKind::CIDOperator(CIDOperator::EndCIDRange),
            TokenKind::Integer(5),
            TokenKind::CIDOperator(CIDOperator::BeginCIDChar),
            TokenKind::HexString(b"8143"),
            TokenKind::Integer(8286),
            TokenKind::HexString(b"8144"),
            TokenKind::Integer(8274),
            TokenKind::HexString(b"814a"),
            TokenKind::Integer(8272),
            TokenKind::HexString(b"8387"),
            TokenKind::Integer(7936),
            TokenKind::HexString(b"838e"),
            TokenKind::Integer(7937),
            TokenKind::CIDOperator(CIDOperator::EndCIDChar),
            TokenKind::CIDOperator(CIDOperator::EndCMap),
            TokenKind::CIDOperator(CIDOperator::End),
            TokenKind::CIDOperator(CIDOperator::End),
            TokenKind::Eof,
        ];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_begin_show_text_ops() {
        let text = "BT
        /F0 12 Tf
        100 700 Td
        (Hello, World) Tj
        ET
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let kinds = vec![
            TokenKind::Operator(Operator::BeginText),
            TokenKind::Name(b"F0"),
            TokenKind::Integer(12),
            TokenKind::Operator(Operator::SelectFont),
            TokenKind::Integer(100),
            TokenKind::Integer(700),
            TokenKind::Operator(Operator::MoveTextPosition),
            TokenKind::LiteralString(b"Hello, World"),
            TokenKind::Operator(Operator::ShowText),
            TokenKind::Operator(Operator::EndText),
            TokenKind::Eof,
        ];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_show_text_adjusted_select_font_ops() {
        let text = "[(Le)15(x)-250(Fridman)]TJ/F13 6.9738 Tf
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let kinds = vec![
            TokenKind::LBracket,
            TokenKind::LiteralString(b"Le"),
            TokenKind::Integer(15),
            TokenKind::LiteralString(b"x"),
            TokenKind::Integer(-250),
            TokenKind::LiteralString(b"Fridman"),
            TokenKind::RBracket,
            TokenKind::Operator(Operator::ShowTextAdjusted),
            TokenKind::Name(b"F13"),
            TokenKind::Real(6.9738),
            TokenKind::Operator(Operator::SelectFont),
            TokenKind::Eof,
        ];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_basic() {
        let text = "%PDF-1.4
        5 0 obj
        endobj
        startxref
        57184
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let kinds = vec![
            TokenKind::Header(Header { major: 1, minor: 4 }),
            TokenKind::Integer(5),
            TokenKind::Integer(0),
            TokenKind::Obj,
            TokenKind::Endobj,
            TokenKind::StartXref,
            TokenKind::Integer(57184),
            TokenKind::Eof,
        ];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_dict_name() {
        let text = "<</Producer(GPL Ghostscript 8.71)>>
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let kinds = vec![
            TokenKind::DoubleLThan,
            TokenKind::Name(b"Producer"),
            TokenKind::LiteralString(b"GPL Ghostscript 8.71"),
            TokenKind::DoubleRThan,
            TokenKind::Eof,
        ];
        assert_eq!(lexer.lex(), kinds);
    }

    #[test]
    fn test_trailer() {
        let text = "trailer
        <</Root 1 0 R
        /ID [<01234567890ABCDEF> <01234567890ABCDEF>]
        /Size 8
        >>
        startxref
        491
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let kinds = vec![
            TokenKind::Trailer,
            TokenKind::DoubleLThan,
            TokenKind::Name(b"Root"),
            TokenKind::Integer(1),
            TokenKind::Integer(0),
            TokenKind::Reference,
            TokenKind::Name(b"ID"),
            TokenKind::LBracket,
            TokenKind::HexString(b"01234567890ABCDEF"),
            TokenKind::HexString(b"01234567890ABCDEF"),
            TokenKind::RBracket,
            TokenKind::Name(b"Size"),
            TokenKind::Integer(8),
            TokenKind::DoubleRThan,
            TokenKind::StartXref,
            TokenKind::Integer(491),
            TokenKind::Eof,
        ];
        assert_eq!(lexer.lex(), kinds);
    }

    // #[test]
    // fn test_file() {
    //     let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     d.push("test_data/hello.pdf");
    //     let file = std::fs::read(d).unwrap();
    //     let mut lexer = Lexer::new(&file);
    //     let tokens = vec![TokenKind::Header(1, 4)];
    //     assert_eq!(lexer.lex(), tokens);
    // }
}
