use super::annotation::Annotation;
use super::resources::Resources;
use crate::operators::{engine::GraphicsEngine, parser::OperatorParser, rect::Rectangle};
use crate::parser::lexer::Lexer;
use crate::parser::object::{IndirectReference, Stream};
use crate::parser::parser::Parser;
use crate::render::canvas::Canvas;

#[derive(Debug)]
pub struct Page<'a> {
    pub r#ref: IndirectReference,
    pub parent: IndirectReference,
    pub media_box: Rectangle,
    pub crop_box: Rectangle,
    pub bleed_box: Rectangle,
    pub trim_box: Rectangle,
    pub art_box: Rectangle,
    pub contents: Option<Vec<&'a Stream>>,
    pub annotations: Option<Vec<Annotation<'a>>>,
    pub resources: Resources<'a>,
    pub rotate: u32,
    // Rendering device
    pub canvas: Canvas,
}

pub struct PagesRoot<'a> {
    kids: Vec<Page<'a>>,
    count: u64,
}

pub struct Catalog<'a> {
    pages: PagesRoot<'a>,
}

impl<'a> Page<'a> {
    pub fn process_operations(&'a mut self) {
        let mut objects = vec![];
        if let Some(content_streams) = &self.contents {
            for content_stream in content_streams {
                let content = &content_stream.content;
                let mut lexer = Lexer::new(content);
                let tokens = lexer.lex();
                let mut parser = Parser::new(&tokens);
                let content_objects = parser.parse();
                objects.extend(content_objects);
            }
        }
        let mut operator_parser = OperatorParser::new(&objects);
        let operations = operator_parser.parse();
        let mut engine = GraphicsEngine::new(self.media_box, &self.resources);
        engine.process_operations(operations);
    }
}

#[cfg(test)]
mod tests {}
