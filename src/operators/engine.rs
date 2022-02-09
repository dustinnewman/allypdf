use nalgebra::Matrix3;

use crate::{document::page::Rectangle, parser::parser::Name};

use super::operations::{DashPattern, LineCap, LineJoin, Percent, RenderingIntent, TextRendering};

// PDF 9.3.1
pub struct TextState {
    // Character spacing
    char_spacing: f32,
    // Word spacing
    word_spacing: f32,
    // Horizontal scaling
    hor_scaling: f32,
    // Leading
    leading: f32,
    // Font
    // TODO
    // Font size
    font_size: f32,
    // Text rendering mode
    render_mode: TextRendering,
    // Text rise
    rise: f32,
    // Text knockout
    knockout: f32,
}

// PDF 8.4.1
#[derive(Debug)]
pub struct GraphicsState {
    // Current clip path
    clip_path: Rectangle,
    // Line width
    line_width: f32,
    // Line cap
    line_cap: LineCap,
    // Line join
    line_join: LineJoin,
    // Miter limit
    miter_limit: f32,
    // Dash pattern
    dash_pattern: DashPattern,
    // Rendering intent
    render_intent: RenderingIntent,
    // Stroke adjustment
    stroke_adjustment: bool,
    // Blend mode
    // TODO
    // Soft mask
    // TODO
    // Alpha constant
    alpha_constant: f32,
    // Alpha source
    alpha_source: bool,
    // Black point compensation
    black_point_compensation: Name,
    // Overprint
    overprint: bool,
    // Overprint mode
    overprint_mode: f32,
    // Black generation
    // TODO
    // Undercolor removal
    // TODO
    // Transfer
    // TODO
    // Halftone
    // TODO
    // Flatness
    flatness: Percent,
    // Smoothness
    smoothness: f32,
}

impl Default for GraphicsState {
    // PDF 8.4.1 Table 51
    fn default() -> Self {
        Self {
            clip_path: Default::default(),
            line_width: 1.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::MiterJoin,
            miter_limit: 10.,
            dash_pattern: DashPattern {
                dash_array: vec![],
                dash_phase: 0.,
            },
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

pub struct GraphicsStateMachine {
    // Current transformation matrix
    ctm: Matrix3<f32>,
    // Current text matrix
    text_matrix: Matrix3<f32>,
    // Current text line matrix
    line_matrix: Matrix3<f32>,
    // Graphics state
    state: GraphicsState,
}

impl GraphicsStateMachine {
    pub fn new() -> Self {
        Self {
            ctm: Matrix3::default(),
            text_matrix: Matrix3::default(),
            line_matrix: Matrix3::default(),
            state: GraphicsState::default(),
        }
    }
}
