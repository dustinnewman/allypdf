use crate::parser::parser::Name;

use super::{
    color::Color,
    matrix::Matrix,
    operations::{DashPattern, LineCap, LineJoin, Percent, RenderingIntent, TextRendering},
    path::{Path, PathMode, Point},
    rect::Rectangle,
};

// PDF 9.3.1
#[derive(Debug, Clone)]
pub struct TextState {
    // Character spacing
    pub char_spacing: f64,
    // Word spacing
    pub word_spacing: f64,
    // Horizontal scaling
    pub hor_scaling: f64,
    // Leading
    pub leading: f64,
    // Font
    // TODO
    // Font size
    pub font_size: f64,
    // Text rendering mode
    pub render_mode: TextRendering,
    // Text rise
    pub rise: f64,
    // Text knockout
    pub knockout: bool,
}

impl Default for TextState {
    fn default() -> Self {
        Self {
            char_spacing: 0.,
            word_spacing: 0.,
            hor_scaling: 100.,
            leading: 0.,
            // SPEC_BREAK: Font size is not supposed to have default value (9.3.1 Table 103)
            font_size: 12.,
            render_mode: TextRendering::Fill,
            rise: 0.,
            knockout: false,
        }
    }
}

// PDF 8.4.1
#[derive(Debug, Clone)]
pub struct GraphicsState {
    // Current transformation matrix
    pub ctm: Matrix,
    // Fill color
    pub fill_color: Color,
    // Stroke color
    pub stroke_color: Color,
    // Current text matrix
    pub text_matrix: Matrix,
    // Current text line matrix
    pub line_matrix: Matrix,
    // Text state
    pub text_state: TextState,
    // Current coordinates
    pub current_point: Point,
    // Line coordinates
    pub line_coordinates: Point,
    // Path
    pub path: Path,
    // Clipping mode
    pub path_mode: PathMode,
    // Current clip path
    pub clip_path: Rectangle,
    // Line width
    pub line_width: f64,
    // Line cap
    pub line_cap: LineCap,
    // Line join
    pub line_join: LineJoin,
    // Miter limit
    pub miter_limit: f64,
    // Dash pattern
    pub dash_pattern: DashPattern,
    // Rendering intent
    pub render_intent: RenderingIntent,
    // Stroke adjustment
    pub stroke_adjustment: bool,
    // Blend mode
    // TODO
    // Soft mask
    // TODO
    // Alpha constant
    pub alpha_constant: f64,
    // Alpha source
    pub alpha_source: bool,
    // Black point compensation
    pub black_point_compensation: Name,
    // Overprint
    pub overprint: bool,
    // Overprint mode
    pub overprint_mode: f64,
    // Black generation
    // TODO
    // Undercolor removal
    // TODO
    // Transfer
    // TODO
    // Halftone
    // TODO
    // Flatness
    pub flatness: Percent,
    // Smoothness
    pub smoothness: f64,
}

impl Default for GraphicsState {
    // PDF 8.4.1 Table 51
    fn default() -> Self {
        Self {
            ctm: Matrix::default(),
            fill_color: Color::default(),
            stroke_color: Color::default(),
            text_matrix: Matrix::default(),
            line_matrix: Matrix::default(),
            current_point: Point::default(),
            line_coordinates: Point::default(),
            text_state: TextState::default(),
            path_mode: PathMode::default(),
            path: Path::default(),
            clip_path: Rectangle::default(),
            line_width: 1.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            miter_limit: 10.,
            dash_pattern: DashPattern::default(),
            render_intent: RenderingIntent::RelativeColorimetric,
            stroke_adjustment: false,
            alpha_constant: 1.0,
            alpha_source: false,
            black_point_compensation: Default::default(),
            overprint: false,
            overprint_mode: 0.,
            flatness: 1.0,
            smoothness: Default::default(),
        }
    }
}
