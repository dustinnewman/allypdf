use crate::{
    document::page::Rectangle,
    parser::parser::{Dictionary, Name},
};

use super::operations::{
    Color, DashPattern, LineCap, LineJoin, Operation, Percent, RenderingIntent, StringOrNumber,
    TextRendering, UnitInterval, UnscaledTextSpaceUnit, CMYK, RGB,
};

// PDF 9.3.1
pub struct TextState {
    // Character spacing
    char_spacing: f64,
    // Word spacing
    word_spacing: f64,
    // Horizontal scaling
    hor_scaling: f64,
    // Leading
    leading: f64,
    // Font
    // TODO
    // Font size
    font_size: f64,
    // Text rendering mode
    render_mode: TextRendering,
    // Text rise
    rise: f64,
    // Text knockout
    knockout: f64,
}

// PDF 8.4.1
#[derive(Debug)]
pub struct GraphicsState {
    // Current clip path
    clip_path: Rectangle,
    // Line width
    line_width: f64,
    // Line cap
    line_cap: LineCap,
    // Line join
    line_join: LineJoin,
    // Miter limit
    miter_limit: f64,
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
    alpha_constant: f64,
    // Alpha source
    alpha_source: bool,
    // Black point compensation
    black_point_compensation: Name,
    // Overprint
    overprint: bool,
    // Overprint mode
    overprint_mode: f64,
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
    smoothness: f64,
}

impl GraphicsState {
    pub fn set_color_stroke(&mut self) {}
    pub fn set_color_fill(&mut self) {}
    pub fn set_dash_pattern(&mut self) {}
}

impl Default for GraphicsState {
    // PDF 8.4.1 Table 51
    fn default() -> Self {
        Self {
            clip_path: Rectangle::default(),
            line_width: 1.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::MiterJoin,
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

pub struct GraphicsStateMachine {
    // Current transformation matrix
    ctm: [f64; 6],
    // Current text matrix
    text_matrix: [f64; 6],
    // Current text line matrix
    line_matrix: [f64; 6],
    // Graphics state
    state: GraphicsState,
}

// Yeah, yeah, this is not a real state machine since it mutates but it's faster
impl GraphicsStateMachine {
    pub fn new(clip_path: Rectangle) -> Self {
        let mut state = GraphicsState::default();
        state.clip_path = clip_path;
        Self {
            ctm: [0.0; 6],
            text_matrix: [0.0; 6],
            line_matrix: [0.0; 6],
            state,
        }
    }

    fn process(&mut self, ops: Vec<Operation>) {
        for op in ops {
            self.transition(op);
        }
    }

    fn transition(&mut self, op: Operation) {
        match op {
            Operation::CloseStrokePath => self.close_stroke_path(),
            Operation::StrokePath => self.stroke_path(),
            Operation::FillPath => self.fill_path(),
            Operation::FillPathEvenOdd => self.fill_path_even_odd(),
            Operation::CloseFillStrokePath => self.close_fill_stroke_path(),
            Operation::FillStrokePath => self.fill_stroke_path(),
            Operation::CloseFillStrokePathEvenOdd => self.close_fill_stroke_path_even_odd(),
            Operation::FillStrokePathEvenOdd => self.fill_stroke_path_even_odd(),
            Operation::EndPathNoFill => self.end_path_no_fill(),
            Operation::SetClippingPath => self.set_clipping_path(),
            Operation::SetClippingPathEvenOdd => self.set_clipping_path_even_odd(),
            Operation::DefineMarkedContentPoint(name) => self.define_marked_content_point(name),
            Operation::DefineMarkedContentPointPropertyList(name, dict) => self.define_marked_content_point_property_list(name, dict),
            Operation::BeginMarkedContentSequence(name) => self.begin_marked_content_sequence(name),
            Operation::BeginMarkedContentSequencePropertyList(name, dict) => self.begin_marked_content_sequence_property_list(name, dict),
            Operation::EndMarkedContentSequence => self.end_marked_content_sequence(),
            Operation::BeginInlineImageObject => self.begin_inline_image_object(),
            Operation::BeginInlineImageData => self.begin_inline_image_data(),
            Operation::EndInlineImage => self.end_inline_image(),
            Operation::BeginText => self.begin_text(),
            Operation::ShowText(bytes) => self.show_text(bytes),
            Operation::ShowTextAdjusted(vec) => self.show_text_adjusted(vec),
            Operation::MoveNextLineShowText(bytes) => self.move_next_line_show_text(bytes),
            Operation::SetSpacingMoveNextLineShowText(word_spacing, character_spacing, bytes) => self.set_spacing_move_next_line_show_text(word_spacing, character_spacing, bytes),
            Operation::MoveTextPosition(tx, ty) => self.move_text_position(tx, ty),
            Operation::MoveTextPositionLeading(tx, ty) => self.move_text_position_leading(tx, ty),
            Operation::SetTextMatrix(matrix) => self.set_text_matrix(matrix),
            Operation::MoveStartNextLine => self.move_start_next_line(),
            Operation::SetCharSpacing(char_spacing) => self.set_char_spacing(char_spacing),
            Operation::SelectFont(font_name, font_size) => self.select_font(font_name, font_size),
            Operation::SetTextLeading(text_leading) => self.set_text_leading(text_leading),
            Operation::SetTextRendering(rendering_mode) => self.set_text_rendering(rendering_mode),
            Operation::SetTextRise(text_rise) => self.set_text_rise(text_rise),
            Operation::SetWordSpacing(word_spacing) => self.set_word_spacing(word_spacing),
            Operation::SetHorizontalTextScaling(text_scaling) => self.set_horizontal_text_scaling(text_scaling),
            Operation::EndText => self.end_text(),
            Operation::BeginCompat => self.begin_compat(),
            Operation::SetCharWidth(horizontal_displacement, vertical_displacement) => self.set_char_width(horizontal_displacement, vertical_displacement),
            Operation::SetCacheDevice(s, ll, ur) => self.set_cache_device(s, ll, ur),
            Operation::InvokeXObject(name) => self.invoke_xobject(name),
            Operation::EndCompat => self.end_compat(),
            Operation::MoveTo(x, y) => self.move_to(x, y),
            Operation::LineTo(x, y) => self.line_to(x, y),
            Operation::AppendCurveThreePoints(p1, p2, p3) => self.append_curve_three_points(p1, p2, p3),
            Operation::AppendCurveInitialReplicated(p2, p3) => self.append_curve_initial_replicated(p2, p3),
            Operation::AppendCurveFinalReplicated(p1, p3) => self.append_curve_final_replicated(p1, p3),
            Operation::AppendRectangle(x, y, width, height) => self.append_rectangle(x, y, width, height),
            Operation::CloseSubpath => self.close_subpath(),
            Operation::SetMiterLimit(miter_limit) => self.set_miter_limit(miter_limit),
            Operation::ConcatMatrix(matrix) => self.concat_matrix(matrix),
            Operation::SetLineWidth(line_width) => self.set_line_width(line_width),
            Operation::SetLineJoin(line_join) => self.set_line_join(line_join),
            Operation::SetLineCap(line_cap) => self.set_line_cap(line_cap),
            Operation::SetDash(dash_array, dash_phase) => self.set_dash(dash_array, dash_phase),
            Operation::GSave => self.g_save(),
            Operation::GRestore => self.g_restore(),
            Operation::SetColorRenderingIntent(name) => self.set_color_rendering_intent(name),
            Operation::SetFlat(flatness) => self.set_flat(flatness),
            Operation::SetGraphicsStateParams(dict_name) => self.set_graphics_state_params(dict_name),
            Operation::SetCMYKColorStroke(color) => self.set_cmyk_color_stroke(color),
            Operation::SetCMYKColorNonstroke(color) => self.set_cmyk_color_nonstroke(color),
            Operation::SetColorStroke(color) => self.set_color_stroke(color),
            Operation::SetColorNonstroke(color) => self.set_color_nonstroke(color),
            Operation::SetColorSpecialStroke(color, name) => self.set_color_special_stroke(color, name),
            Operation::SetColorSpecialNonstroke(color, name) => self.set_color_special_nonstroke(color, name),
            Operation::SetColorSpaceStroke(name) => self.set_color_space_stroke(name),
            Operation::SetColorSpaceNonstroke(name) => self.set_color_space_nonstroke(name),
            Operation::SetRGBColorStroke(color) => self.set_rgb_color_stroke(color),
            Operation::SetRGBColorNonstroke(color) => self.set_rgb_color_nonstroke(color),
            Operation::SetGrayStroke(gray) => self.set_gray_stroke(gray),
            Operation::SetGrayNonstroke(gray) => self.set_gray_nonstroke(gray),
            Operation::ShFill(name) => self.sh_fill(name),
        }
    }

    // Path operators
    fn close_stroke_path(&mut self) {}

    fn stroke_path(&mut self) {}

    fn fill_path(&mut self) {}

    fn fill_path_even_odd(&mut self) {}
    fn close_fill_stroke_path(&mut self) {}
    fn fill_stroke_path(&mut self) {}
    fn close_fill_stroke_path_even_odd(&mut self) {}
    fn fill_stroke_path_even_odd(&mut self) {}
    fn end_path_no_fill(&mut self) {}
    fn set_clipping_path(&mut self) {}
    fn set_clipping_path_even_odd(&mut self) {}
    // Marked content operators
    fn define_marked_content_point(&mut self, name: &Name) {}
    fn define_marked_content_point_property_list(&mut self, name: &Name, dict: &Dictionary) {}
    fn begin_marked_content_sequence(&mut self, name: &Name) {}
    fn begin_marked_content_sequence_property_list(&mut self, name: &Name, dict: &Dictionary) {}
    fn end_marked_content_sequence(&mut self) {}
    // Image operators
    fn begin_inline_image_object(&mut self) {}
    fn begin_inline_image_data(&mut self) {}
    fn end_inline_image(&mut self) {}
    // Text operators
    fn begin_text(&mut self) {}
    fn show_text(&mut self, bytes: &Vec<u8>) {}
    fn show_text_adjusted(&mut self, vec: Vec<StringOrNumber>) {}
    fn move_next_line_show_text(&mut self, bytes: &Vec<u8>) {}
    fn set_spacing_move_next_line_show_text(
        &mut self,
        word_spacing: UnscaledTextSpaceUnit,
        character_spacing: UnscaledTextSpaceUnit,
        bytes: &Vec<u8>,
    ) {
    }
    fn move_text_position(&mut self, tx: UnscaledTextSpaceUnit, ty: UnscaledTextSpaceUnit) {}
    fn move_text_position_leading(&mut self, tx: UnscaledTextSpaceUnit, ty: UnscaledTextSpaceUnit) {
    }
    fn set_text_matrix(&mut self, matrix: [f64; 6]) {}
    fn move_start_next_line(&mut self) {}
    fn set_char_spacing(&mut self, char_spacing: UnscaledTextSpaceUnit) {}
    fn select_font(&mut self, font_name: &Name, font_size: f64) {}
    fn set_text_leading(&mut self, text_leading: UnscaledTextSpaceUnit) {}
    fn set_text_rendering(&mut self, rendering_mode: TextRendering) {}
    fn set_text_rise(&mut self, text_rise: UnscaledTextSpaceUnit) {}
    fn set_word_spacing(&mut self, word_spacing: UnscaledTextSpaceUnit) {}
    fn set_horizontal_text_scaling(&mut self, text_scaling: f64) {}
    fn end_text(&mut self) {}
    fn begin_compat(&mut self) {}
    // Type 3 font operators
    fn set_char_width(&mut self, horizontal_displacement: f64, _vertical_displacement: f64) {}
    fn set_cache_device(
        &mut self,
        (horizontal_displacement, _vertical_displacement): (f64, f64),
        (lower_left_x, lower_left_y): (f64, f64),
        (upper_right_x, upper_right_y): (f64, f64),
    ) {
    }
    fn invoke_xobject(&mut self, name: &Name) {}
    fn end_compat(&mut self) {}
    // Path construction operators
    fn move_to(&mut self, x: f64, y: f64) {}
    fn line_to(&mut self, x: f64, y: f64) {}
    fn append_curve_three_points(
        &mut self,
        (x1, y1): (f64, f64),
        (x2, y2): (f64, f64),
        (x3, y3): (f64, f64),
    ) {
    }
    fn append_curve_initial_replicated(&mut self, (x2, y2): (f64, f64), (x3, y3): (f64, f64)) {}
    fn append_curve_final_replicated(&mut self, (x1, y1): (f64, f64), (x3, y3): (f64, f64)) {}
    fn append_rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {}
    fn close_subpath(&mut self) {}
    fn set_miter_limit(&mut self, miter_limit: f64) {}
    fn concat_matrix(&mut self, matrix: [f64; 6]) {}
    fn set_line_width(&mut self, line_width: f64) {}
    fn set_line_join(&mut self, line_join: LineJoin) {}
    fn set_line_cap(&mut self, line_cap: LineCap) {}
    fn set_dash(&mut self, dash_array: Vec<i64>, dash_phase: i64) {}
    fn g_save(&mut self) {}
    fn g_restore(&mut self) {}
    fn set_color_rendering_intent(&mut self, name: &Name) {}
    fn set_flat(&mut self, flatness: Percent) {}
    fn set_graphics_state_params(&mut self, dict_name: &Name) {}
    // Color operators
    fn set_cmyk_color_stroke(&mut self, color: CMYK) {}
    fn set_cmyk_color_nonstroke(&mut self, color: CMYK) {}
    fn set_color_stroke(&mut self, color: Color) {}
    fn set_color_nonstroke(&mut self, color: Color) {}
    fn set_color_special_stroke(&mut self, color: Color, name: Option<&Name>) {}
    fn set_color_special_nonstroke(&mut self, color: Color, name: Option<&Name>) {}
    fn set_color_space_stroke(&mut self, name: &Name) {}
    fn set_color_space_nonstroke(&mut self, name: &Name) {}
    fn set_rgb_color_stroke(&mut self, color: RGB) {}
    fn set_rgb_color_nonstroke(&mut self, color: RGB) {}
    fn set_gray_stroke(&mut self, gray: UnitInterval) {}
    fn set_gray_nonstroke(&mut self, gray: UnitInterval) {}
    // Shading operators
    fn sh_fill(&mut self, name: &Name) {}
}