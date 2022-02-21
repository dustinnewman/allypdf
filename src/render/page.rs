use crate::operators::{
    operations::Color,
    path::{Curve, Point},
};

pub struct Text {
    text: String,
    font: String,
    italic: bool,
    bold: bool,
    underlined: bool,
}

pub struct Rectangle {
    width: f32,
    height: f32,
    border_radius: f32,
}

pub enum AbstractElementKind {
    Text(Text),
    Rectangle(Rectangle),
    Curve(Curve),
}

pub struct AbstractElement {
    kind: AbstractElementKind,
    upper_left_point: Point,
    stroke_color: Color,
    fill_color: Color,
    stroke_width: f32,
}

pub struct GraphicsOutputModel {
    elements: Vec<AbstractElement>,
}
