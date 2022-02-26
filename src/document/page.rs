use std::convert::TryFrom;

use crate::{
    error::PdfError,
    operators::{rect::Rectangle, parser::OperatorParser, engine::GraphicsStateMachine},
    parser::{parser::{Dictionary, IndirectReference, Object, ObjectKind, Stream, Parser}, lexer::Lexer},
};

use super::annotation::Annotation;

const PDF: &[u8] = b"PDF";
const TEXT: &[u8] = b"Text";
const IMAGE_B: &[u8] = b"ImageB";
const IMAGE_C: &[u8] = b"ImageC";
const IMAGE_I: &[u8] = b"ImageI";

#[derive(Debug)]
pub enum ProcSet {
    PDF,
    Text,
    ImageBlack,
    ImageColor,
    ImageIndexed,
}

impl TryFrom<&Object> for ProcSet {
    type Error = PdfError;

    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        let error = PdfError::Other {
            msg: "Could not convert name to ProcSet".to_string(),
        };
        match object {
            Object {
                kind: ObjectKind::Name(name),
                ..
            } => match name.as_ref() {
                PDF => Ok(Self::PDF),
                TEXT => Ok(Self::Text),
                IMAGE_B => Ok(Self::ImageBlack),
                IMAGE_C => Ok(Self::ImageColor),
                IMAGE_I => Ok(Self::ImageIndexed),
                _ => Err(error),
            },
            _ => Err(error),
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
    pub font: Option<&'a Dictionary>,
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
    resources: Resources<'a>,
    rotate: u32,
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
        }
    }

    pub fn parse_contents(&self) {
        if let Some(content_streams) = &self.contents {
            for content_stream in content_streams {
                let content = &content_stream.content;
                let mut lexer = Lexer::new(content);
                let tokens = lexer.lex();
                let mut parser = Parser::new(&tokens);
                let objects = parser.parse();
                let mut operator_parser = OperatorParser::new(&objects);
                let operations = operator_parser.parse();
                let mut graphics_engine = GraphicsStateMachine::new(Some(self.media_box));
                graphics_engine.process(operations);
            }
        }
    }
}
