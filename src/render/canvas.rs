use crate::operators::engine::GraphicsState;

use crate::operators::{
    color::Color,
    path::{Curve, Point},
};

#[derive(Debug)]
pub struct Text {
    text: String,
    font: String,
    italic: bool,
    bold: bool,
    underlined: bool,
}

#[derive(Debug)]
pub struct Rectangle {
    width: f32,
    height: f32,
    border_radius: f32,
}

#[derive(Debug)]
pub enum AbstractElementKind {
    Text(Text),
    Rectangle(Rectangle),
    Curve(Curve),
}

#[derive(Debug)]
pub struct AbstractElement {
    kind: AbstractElementKind,
    upper_left_point: Point,
    stroke_color: Color,
    fill_color: Color,
    stroke_width: f32,
}

#[derive(Debug)]
pub struct Canvas {
    elements: Vec<AbstractElement>,
}

impl Canvas {
    pub fn new() -> Self {
        Self { elements: vec![] }
    }
    pub fn stroke(&mut self, state: &GraphicsState) {}
    pub fn fill(&mut self, state: &GraphicsState) {}
    pub fn draw_rectangle(&mut self, state: &GraphicsState) {
    }
}
