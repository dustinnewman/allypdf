// use crate::operators::operations::Operation;
// use crate::parser::lexer::Token;

// pub struct Parser<'a> {
//     tokens: &'a [Token<'a>],
//     pos: usize,
// }

// impl<'a> Parser<'a> {
//     pub fn new(tokens: &'a [Token<'a>]) -> Self {
//         Self {
//             tokens,
//             pos: tokens.len(),
//         }
//     }

//     pub fn parse(&mut self) -> Vec<Operation> {
//         let mut vec = vec![];
//         loop {
//             if let Some(object) = self.next() {
//                 vec.push(object);
//             } else {
//                 break;
//             }
//         }
//         vec
//     }

//     fn next(&mut self) -> Option<Operation> {
//         let token = self.pop()?;
//         let object = match token {
//             Token::Boolean(b) => Object::Boolean(*b),
//             Token::Null => Object::Null,
//             Token::Real(r) => Object::Real(*r),
//             Token::Header(m, n) => Object::Header(*m, *n),
//             Token::Trailer => self.trailer()?,
//             Token::Integer(i) => {
//                 let i = *i;
//                 self.integer(i)?
//             },
//             Token::LiteralString(lit) => {
//                 let lit = *lit;
//                 self.literal_string_to_string(lit)?
//             },
//             Token::HexString(hex) => {
//                 let hex = *hex;
//                 self.hex_string_to_string(hex)?
//             }
//             Token::Name(name) => {
//                 let name = *name;
//                 self.name_to_name(name)?
//             }
//             Token::Xref => self.xref(),
//             Token::StartXref => self.start_xref()?,
//             Token::LBracket => self.array()?,
//             Token::DoubleLThan => self.dictionary()?,
//             _ => return None,
//         };
//         Some(object)
//     }

//     fn pop(&mut self) -> Option<&Token<'a>> {
//         // TODO: Fix this
//         // let token = self.peek();
//         self.advance();
//         Some(&self.tokens[self.pos - 1])
//     }

//     fn advance(&mut self) {
//         self.seek(1);
//     }

//     fn seek(&mut self, n: usize) {
//         if self.pos + n <= self.tokens.len() {
//             self.pos += n;
//         }
//     }

//     fn peek(&self) -> Option<&Token<'a>> {
//         self.nth(0)
//     }

//     fn nth(&self, n: usize) -> Option<&Token<'a>> {
//         if self.pos + n < self.tokens.len() {
//             Some(&self.tokens[self.pos + n])
//         } else {
//             None
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use std::fs;
//     use std::path::PathBuf;
//     use super::*;
//     use crate::parser::lexer::Lexer;

//     #[test]
//     fn test_content_parser_file() {
//         let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//         d.push("test_data/hello.pdf");
//         let file = fs::read(d).unwrap();
//         let mut lexer = Lexer::new(&file);
//         let tokens = lexer.lex();
//         // assert_eq!(tokens, vec![]);
//         let mut parser = Parser::new(&tokens);
//         let objects = parser.parse();
//         assert_eq!(objects, vec![]);
//     }
// }