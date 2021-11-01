use std::collections::BTreeMap;
use super::grouper::{Dictionary, Object};

pub struct PDFDocument {
    // (major, minor)
    version: (u8, u8),
    trailer: Dictionary,
    object_table: BTreeMap<u64, Object>
}

pub struct Parser {
    objects: Vec<Object>,
    pos: usize,
}

impl Parser {
    fn new(objects: Vec<Object>) -> Self {
        Self {
            objects,
            pos: 0,
        }
    }

    fn parse(&mut self) -> PDFDocument {
        let mut header: (u8, u8) = (1, 4);
        for object in &self.objects {
            match *object {
                Object::Header(major, minor) => {
                    header = (major, minor)
                },
                _ => (),
            }
        }
        let document = PDFDocument {
            version: header,
            trailer: BTreeMap::new(),
            object_table: BTreeMap::new(),
        };
        document
    }

    fn pop(&mut self) -> Option<&Object> {
        self.advance();
        Some(&self.objects[self.pos - 1])
    }

    fn advance(&mut self) {
        self.seek(1)
    }

    fn seek(&mut self, n: usize) {
        if self.pos + n <= self.objects.len() {
            self.pos += n;
        }
    }

    fn peek(&self) -> Option<&Object> {
        self.nth(0)
    }

    fn nth(&self, n: usize) -> Option<&Object> {
        if self.pos + n < self.objects.len() {
            Some(&self.objects[self.pos + n])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::lexer::Lexer;
    use crate::parser::grouper::Grouper;

    #[test]
    fn test_integer() {
        let text = "0
        %%EOF";
        let mut lexer = Lexer::new(text.as_bytes());
        let tokens = lexer.lex();
        let mut grouper = Grouper::new(&tokens);
        let objects = grouper.group();
        let mut parser = Parser::new(objects);
        let pdf_document = parser.parse();
        assert_eq!(1, 1);
    }
}