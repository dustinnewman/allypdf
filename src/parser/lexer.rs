use crate::error::{Result, PdfError};
use crate::util::{Byte, DQUOTE, FSLASH, LBRACE, LBRACKET, LINE_FEED, LPAREN, LTHAN, PERCENT, PERIOD, RBRACE, RBRACKET, RPAREN, RTHAN, SQUOTE, byte_to_numeric, is_decimal, is_hexadecimal, is_newline, is_regular, is_whitespace};
use crate::operators::operators::Operator;

enum Character {
    Regular(Byte),
    Delimiter(Byte),
    Whitespace(Byte),
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
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
    // Major Minor (ex: 1.7 = (1, 7))
    Header(u8, u8),
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
    Eof,
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
        // TODO: This is susceptible to attack if someone uploads a PDF
        // without EOF marker
        loop {
            if let Some(token) = self.next() {
                let do_break = token == Token::Eof;
                vec.push(token);
                if do_break {
                    break;
                }
            }
        }
        vec
    }

    fn next(&mut self) -> Option<Token<'a>> {
        // Yes defining a macro inside a function is very ugly but this way we
        // can use `self` without problems
        macro_rules! op {
            ($op:expr) => {
                {
                    self.advance();
                    Token::Operator($op)
                }
            };
        }

        let curr = self.peek()?;
        if is_whitespace(curr) {
            self.skip_whitespace();
        }
        let curr = self.pop()?;
        let token: Token = match curr {
            LTHAN => match self.peek() {
                Some(LTHAN) => {
                    self.advance();
                    Token::DoubleLThan
                },
                _ => self.hex_string()?,
            },
            RTHAN => match self.peek() {
                Some(RTHAN) => {
                    self.advance();
                    Token::DoubleRThan
                },
                _ => return None,
            },
            b'b' => match self.peek() {
                Some(b'*') => op!(Operator::CloseFillStrokePathEvenOdd),
                _ => Token::Operator(Operator::CloseFillStrokePath),
            },
            b'B' => match self.peek() {
                Some(b'*') => op!(Operator::FillStrokePathEvenOdd),
                Some(b'I') => op!(Operator::BeginInlineImageObject),
                Some(b'T') => op!(Operator::BeginText),
                Some(b'X') => op!(Operator::BeginCompat),
                Some(b'M') => {
                    self.advance();
                    match self.pop()? {
                        b'C' => Token::Operator(Operator::BeginMarkedContentSequence),
                        _ => return None,
                    }
                },
                Some(b'D') => {
                    self.advance();
                    match self.pop()? {
                        b'C' => Token::Operator(Operator::BeginMarkedContentSequencePropertyList),
                        _ => return None,
                    }
                },
                _ => Token::Operator(Operator::FillStrokePath),
            },
            b'c' => match self.peek() {
                Some(b'm') => op!(Operator::ConcatMatrix),
                Some(b's') => op!(Operator::SetColorSpaceNonstroke),
                _ => Token::Operator(Operator::AppendCurveThreePoints),
            },
            b'C' => match self.peek()? {
                b'S' => op!(Operator::SetColorSpaceStroke),
                _ => return None,
            },
            b'd' => match self.peek() {
                Some(b'0') => op!(Operator::SetCharWidth),
                Some(b'1') => op!(Operator::SetCacheDevice),
                _ => Token::Operator(Operator::SetDash)
            },
            b'D' => match self.peek()? {
                b'o' => op!(Operator::InvokeXObject),
                b'P' => op!(Operator::DefineMarkedContentPointPropertyList),
                _ => return None,
            },
            b'e' => self.endobj()?,
            b'E' => match self.peek()? {
                b'I' => op!(Operator::EndInlineImage),
                b'T' => op!(Operator::EndText),
                b'X' => op!(Operator::EndCompat),
                b'M' => {
                    self.advance();
                    match self.pop()? {
                        b'C' => Token::Operator(Operator::EndMarkedContentSequence),
                        _ => return None,
                    }
                },
                _ => return None,
            },
            b'f' => match self.peek() {
                Some(b'*') => op!(Operator::FillPathEvenOdd),
                Some(b'a') => match (self.pop()?, self.pop()?, self.pop()?) {
                    (b'l', b's', b'e') => Token::Boolean(false),
                    _ => return None,
                },
                _ => Token::F,
            },
            b'F' => Token::Operator(Operator::FillPath),
            b'g' => match self.peek() {
                Some(b's') => op!(Operator::SetGraphicsStateParams),
                _ => Token::Operator(Operator::SetGrayNonstroke),
            },
            b'G' => Token::Operator(Operator::SetGrayStroke),
            b'h' => Token::Operator(Operator::CloseSubpath),
            b'i' => Token::Operator(Operator::SetFlat),
            b'I' => match self.peek()? {
                b'D' => op!(Operator::BeginInlineImageData),
                _ => return None,
            },
            b'j' => Token::Operator(Operator::SetLineJoin),
            b'J' => Token::Operator(Operator::SetLineCap),
            b'k' => Token::Operator(Operator::SetCMYKColorNonstroke),
            b'K' => Token::Operator(Operator::SetCMYKColorStroke),
            b'l' => Token::Operator(Operator::LineTo),
            b'm' => Token::Operator(Operator::MoveTo),
            b'M' => match self.peek() {
                Some(b'P') => op!(Operator::DefineMarkedContentPoint),
                _ => Token::Operator(Operator::SetMiterLimit),
            },
            b'n' => match self.peek() {
                Some(b'u') => match (self.pop()?, self.pop()?, self.pop()?) {
                    (b'u', b'l', b'l') => Token::Null,
                    _ => return None,
                },
                _ => Token::N,
            },
            b'o' => match (self.pop(), self.pop()) {
                (Some(b'b'), Some(b'j')) => Token::Obj,
                _ => return None,
            },
            b'q' => Token::Operator(Operator::GSave),
            b'Q' => Token::Operator(Operator::GRestore),
            b'r' => match self.peek()? {
                b'e' => op!(Operator::AppendRectangle),
                b'g' => op!(Operator::SetRGBColorNonstroke),
                b'i' => op!(Operator::SetColorRenderingIntent),
                _ => return None,
            },
            b'R' => match self.peek() {
                Some(b'G') => op!(Operator::SetRGBColorStroke),
                _ => Token::Reference,
            },
            b's' => match self.peek() {
                Some(b'c') => {
                    self.advance();
                    match self.peek() {
                        Some(b'n') => op!(Operator::SetColorSpecialNonstroke),
                        _ => Token::Operator(Operator::SetColorNonstroke),
                    }
                },
                Some(b't') => {
                    self.advance();
                    match self.pop()? {
                        b'r' => self.stream()?,
                        b'a' => self.startxref()?,
                        _ => return None,
                    }
                },
                Some(b'h') => op!(Operator::ShFill),
                _ => Token::Operator(Operator::CloseStrokePath),
            },
            b'S' => match self.peek() {
                Some(b'C') => {
                    self.advance();
                    match self.peek() {
                        Some(b'N') => op!(Operator::SetColorSpecialStroke),
                        _ => Token::Operator(Operator::SetColorStroke),
                    }
                },
                _ => Token::Operator(Operator::StrokePath),
            },
            b't' => match (self.pop(), self.pop(), self.pop()) {
                (Some(b'r'), Some(b'u'), Some(b'e')) => Token::Boolean(true),
                (Some(b'r'), Some(b'a'), Some(b'i')) => self.trailer()?,
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
            b'v' => Token::Operator(Operator::AppendCurveInitialReplicated),
            b'w' => Token::Operator(Operator::SetLineWidth),
            b'W' => match self.peek() {
                Some(b'*') => op!(Operator::SetClippingPathEvenOdd),
                _ => Token::Operator(Operator::SetClippingPath),
            },
            b'x' => match (self.pop(), self.pop(), self.pop()) {
                (Some(b'r'), Some(b'e'), Some(b'f')) => Token::Xref,
                _ => return None,
            },
            b'y' => Token::Operator(Operator::AppendCurveFinalReplicated),
            b'0'..=b'9' | b'+' | b'-' | PERIOD => self.number()?,
            PERCENT => self.percent()?,
            LPAREN => self.literal_string()?,
            LBRACKET => Token::LBracket,
            RBRACKET => Token::RBracket,
            LBRACE => Token::LBrace,
            RBRACE => Token::RBrace,
            FSLASH => self.name()?,
            SQUOTE => Token::Operator(Operator::MoveNextLineShowText),
            DQUOTE => Token::Operator(Operator::SetSpacingMoveNextLineShowText),
            _ => return None,
        };
        Some(token)
    }

    fn number(&mut self) -> Option<Token<'a>> {
        let start = self.pos - 1;
        let mut is_real = false;
        loop {
            match self.peek() {
                Some(c) if is_decimal(c) => self.advance(),
                Some(PERIOD) => {
                    self.advance();
                    is_real = true
                },
                _ => break,
            }
        }
        let buf = &(self.buf[start..self.pos]);
        if is_real {
            let number = std::str::from_utf8(buf).ok().and_then(|x| x.parse().ok())?;
            Some(Token::Real(number))
        } else {
            let number = std::str::from_utf8(buf).ok().and_then(|x| x.parse().ok())?;
            Some(Token::Integer(number))
        }
    }

    fn trailer(&mut self) -> Option<Token<'a>> {
        let string = "ler".as_bytes();
        let token = match self.slice(self.pos, string.len()) {
            Some(x) if x == string => {
                self.seek(string.len());
                Token::Trailer
            },
            _ => return None,
        };
        Some(token)
    }

    fn startxref(&mut self) -> Option<Token<'a>> {
        let string = "rtxref".as_bytes();
        let token = match self.slice(self.pos, string.len()) {
            Some(x) if x == string => {
                self.seek(string.len());
                Token::StartXref
            },
            _ => return None,
        };
        Some(token)
    }

    fn endobj(&mut self) -> Option<Token<'a>> {
        let string = "ndobj".as_bytes();
        let token = match self.slice(self.pos, string.len()) {
            Some(x) if x == string => {
                self.seek(string.len());
                Token::Endobj
            },
            _ => return None,
        };
        Some(token)
    }

    fn stream(&mut self) -> Option<Token<'a>> {
        let stream_string = "eam".as_bytes();
        let endstream_string = "endstream".as_bytes();
        match self.slice(self.pos, stream_string.len()) {
            Some(x) if x == stream_string => {
                // PDF32000_2008 7.3.8.1 paragraph 5
                self.get_next_char_while(|c| c != LINE_FEED);
                // Skip newline character
                self.advance();
                let start = self.pos;
                loop {
                    self.skip_whitespace();
                    match self.slice(self.pos, endstream_string.len()) {
                        Some(y) if y == endstream_string => {
                            let end = self.pos;
                            self.seek(endstream_string.len());
                            return Some(Token::Stream(&self.buf[start..end]));
                        },
                        Some(_) => {
                            self.advance();
                        },
                        None => {
                            return None;
                        },
                    }
                }
            },
            _ => None
        }
    }

    fn hex_string(&mut self) -> Option<Token<'a>> {
        let string = self.get_next_char_while(is_hexadecimal);
        Some(Token::HexString(string))
    }

    fn literal_string(&mut self) -> Option<Token<'a>> {
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
                    return Some(Token::LiteralString(&self.buf[start..self.pos-1]));
                }
            }
        }
        None
    }

    fn name(&mut self) -> Option<Token<'a>> {
        let start = self.pos;
        loop {
            match self.peek() {
                Some(c) if is_whitespace(c) => {
                    break;
                },
                Some(FSLASH) => {
                    break;
                },
                Some(c) if is_regular(c) => {
                    self.advance();
                    continue;
                },
                _ => break,
            };
        }
        Some(Token::Name(&self.buf[start..self.pos]))
    }

    fn eof(&mut self) -> Option<Token<'a>> {
        let string = "EOF".as_bytes();
        let token = match self.slice(self.pos, string.len()) {
            Some(x) if x == string => {
                self.seek_end();
                Token::Eof
            },
            _ => {
                self.next_line();
                return None
            },
        };
        Some(token)
    }
    
    fn header_version(&mut self) -> Option<Token<'a>> {
        let token = match (self.nth(1), self.nth(2), self.nth(3), self.nth(4), self.nth(5), self.nth(6)) {
            (Some(b'D'), Some(b'F'), Some(b'-'), Some(m), Some(b'.'), Some(n)) if is_decimal(m) && is_decimal(n) => {
                // Skip past "DF-m.n"
                self.seek(7);
                let major = byte_to_numeric(m, 10)?;
                let minor = byte_to_numeric(n, 10)?;
                Token::Header(major, minor)
            },
            _ => {
                self.next_line();
                return None
            },
        };
        Some(token)
    }

    fn percent(&mut self) -> Option<Token<'a>> {
        let token = match self.peek() {
            Some(PERCENT) => {
                self.advance();
                self.eof()?
            },
            Some(b'P') => self.header_version()?,
            _ => {
                    self.next_line();
                    return None
            },
        };
        Some(token)
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

    fn slice(&self, start: usize, length: usize) -> Option<&'a [Byte]> {
        if start + length > self.len {
            None
        } else {
            Some(&self.buf[start..start+length])
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

    fn back_until(&mut self, predicate: impl Fn(Byte) -> bool) -> Result<usize> {
        self.buf[..self.pos]
            .iter()
            .rposition(|&b| predicate(b))
            .ok_or(PdfError::BOF)
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
    use std::fs;
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_null() {
        let text = "null
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = vec![Token::Null, Token::Eof];
        assert_eq!(lexer.lex(), tokens);
    }

    #[test]
    fn test_scn_op() {
        let text = "sscshscnf
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = vec![
            Token::Operator(Operator::CloseStrokePath),
            Token::Operator(Operator::SetColorNonstroke),
            Token::Operator(Operator::ShFill),
            Token::Operator(Operator::SetColorSpecialNonstroke),
            Token::F,
            Token::Eof];
        assert_eq!(lexer.lex(), tokens);
    }

    #[test]
    fn test_cs_op() {
        let text = "cscmc5cs
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = vec![
            Token::Operator(Operator::SetColorSpaceNonstroke),
            Token::Operator(Operator::ConcatMatrix),
            Token::Operator(Operator::AppendCurveThreePoints),
            Token::Integer(5),
            Token::Operator(Operator::SetColorSpaceNonstroke),
            Token::Eof];
        assert_eq!(lexer.lex(), tokens);
    }

    #[test]
    fn test_capital_t_op() {
        let text = "TcTdTTsf
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = vec![
            Token::Operator(Operator::SetCharSpacing),
            Token::Operator(Operator::MoveTextPosition),
            Token::Operator(Operator::SetTextRise),
            Token::F,
            Token::Eof];
        assert_eq!(lexer.lex(), tokens);
    }

    #[test]
    fn test_capital_e_op() {
        let text = "EIEMCTfETEXEEI5
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = vec![
            Token::Operator(Operator::EndInlineImage),
            Token::Operator(Operator::EndMarkedContentSequence),
            Token::Operator(Operator::SelectFont),
            Token::Operator(Operator::EndText),
            Token::Operator(Operator::EndCompat),
            Token::Operator(Operator::EndInlineImage),
            Token::Integer(5),
            Token::Eof];
        assert_eq!(lexer.lex(), tokens);
    }

    #[test]
    fn test_basic_op() {
        let text = "BT
        /F0 12 Tf
        100 700 Td
        (Hello, World) Tj
        ET
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = vec![
            Token::Operator(Operator::BeginText),
            Token::Name("F0".as_bytes()),
            Token::Integer(12),
            Token::Operator(Operator::SelectFont),
            Token::Integer(100),
            Token::Integer(700),
            Token::Operator(Operator::MoveTextPosition),
            Token::LiteralString(&"Hello, World".as_bytes()),
            Token::Operator(Operator::ShowText),
            Token::Operator(Operator::EndText),
            Token::Eof];
        assert_eq!(lexer.lex(), tokens);
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
        let tokens = vec![Token::Header(1, 4), Token::Integer(5), Token::Integer(0), Token::Obj, Token::Endobj, Token::StartXref, Token::Integer(57184), Token::Eof];
        assert_eq!(lexer.lex(), tokens);
    }

    #[test]
    fn test_dict_name() {
        let text = "<</Producer(GPL Ghostscript 8.71)>>
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = vec![Token::DoubleLThan, Token::Name("Producer".as_bytes()), Token::LiteralString(&"GPL Ghostscript 8.71".as_bytes()), Token::DoubleRThan, Token::Eof];
        assert_eq!(lexer.lex(), tokens);
    }

    #[test]
    fn test_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/hello.pdf");
        let file = fs::read(d).unwrap();
        let mut lexer = Lexer::new(&file);
        let tokens = vec![Token::Header(1, 4)];
        assert_eq!(lexer.lex(), tokens);
    }
}
