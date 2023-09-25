use std::convert::{TryFrom, TryInto};

use super::annotation::Annotation;
use crate::error::PdfError;
use crate::font::font::FontDictionary;
use crate::operators::{engine::GraphicsEngine, parser::OperatorParser, rect::Rectangle};
use crate::parser::lexer::Lexer;
use crate::parser::object::{Dictionary, IndirectReference, Name, Object, Stream};
use crate::parser::parser::Parser;
use crate::render::canvas::Canvas;

const PDF: &[u8] = b"PDF";
const TEXT: &[u8] = b"Text";
const IMAGE_B: &[u8] = b"ImageB";
const IMAGE_C: &[u8] = b"ImageC";
const IMAGE_I: &[u8] = b"ImageI";

#[derive(Debug)]
pub enum ProcSet {
    Pdf,
    Text,
    ImageBlack,
    ImageColor,
    ImageIndexed,
}

impl TryFrom<&Object> for ProcSet {
    type Error = PdfError;

    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        let name: &Name = object.try_into()?;
        match name.0.as_ref() {
            PDF => Ok(Self::Pdf),
            TEXT => Ok(Self::Text),
            IMAGE_B => Ok(Self::ImageBlack),
            IMAGE_C => Ok(Self::ImageColor),
            IMAGE_I => Ok(Self::ImageIndexed),
            _ => Err(PdfError::InvalidProcSet),
        }
    }
}

#[derive(Debug)]
pub struct Resources<'a> {
    pub ext_g_state: Option<&'a Dictionary>,
    pub color_space: Option<&'a Dictionary>,
    pub pattern: Option<&'a Dictionary>,
    pub shading: Option<&'a Dictionary>,
    pub x_object: Option<&'a Dictionary>,
    pub font: Option<FontDictionary<'a>>,
    pub proc_set: Option<Vec<ProcSet>>,
    pub properties: Option<&'a Dictionary>,
}

#[derive(Debug)]
pub struct Page<'a> {
    pub r#ref: IndirectReference,
    parent: IndirectReference,
    media_box: Rectangle,
    crop_box: Rectangle,
    bleed_box: Rectangle,
    trim_box: Rectangle,
    art_box: Rectangle,
    contents: Option<Vec<&'a Stream>>,
    annotations: Option<Vec<Annotation<'a>>>,
    pub resources: Resources<'a>,
    rotate: u32,
    // Rendering device
    canvas: Canvas,
}

pub struct PagesRoot<'a> {
    kids: Vec<Page<'a>>,
    count: u64,
}

pub struct Catalog<'a> {
    pages: PagesRoot<'a>,
}

impl<'a> Page<'a> {
    pub fn new(
        r#ref: IndirectReference,
        parent: IndirectReference,
        media_box: Rectangle,
        crop_box: Rectangle,
        bleed_box: Rectangle,
        trim_box: Rectangle,
        art_box: Rectangle,
        resources: Resources<'a>,
        contents: Option<Vec<&'a Stream>>,
        annotations: Option<Vec<Annotation<'a>>>,
        rotate: u32,
    ) -> Self {
        let canvas = Canvas::new();
        Self {
            r#ref,
            parent,
            media_box,
            crop_box,
            bleed_box,
            trim_box,
            art_box,
            contents,
            annotations,
            resources,
            rotate,
            canvas,
        }
    }

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
