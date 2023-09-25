use super::color::{Color, Cmyk, Rgb};
use super::matrix::Matrix;
use super::operations::{
    DashPattern, LineCap, LineJoin, Operation, Percent, RenderingIntent, StringOrNumber,
    TextRendering, UnitInterval, UnscaledTextSpaceUnit,
};
use super::path::{Path, PathMode, Point};
use super::rect::Rectangle;
use crate::cmaps::cid::CharCodeToUnicode;
use crate::document::page::Resources;
use crate::font::font::Font;
use crate::parser::object::{Dictionary, Name};
use crate::render::canvas::Canvas;

// PDF 9.3.1
#[derive(Debug, Clone)]
pub struct TextState<'a> {
    // Character spacing
    pub char_spacing: f64,
    // Word spacing
    pub word_spacing: f64,
    // Horizontal scaling
    pub hor_scaling: f64,
    // Leading
    pub leading: f64,
    // Font
    pub font: Option<&'a Font<'a>>,
    // Font size
    pub font_size: f64,
    // Text rendering mode
    pub render_mode: TextRendering,
    // Text rise
    pub rise: f64,
    // Text knockout
    pub knockout: bool,
}

impl<'a> Default for TextState<'a> {
    fn default() -> Self {
        Self {
            char_spacing: 0.,
            word_spacing: 0.,
            hor_scaling: 100.,
            leading: 0.,
            font: None,
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
pub struct GraphicsState<'a> {
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
    pub text_state: TextState<'a>,
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

impl<'a> Default for GraphicsState<'a> {
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

#[derive(Debug)]
pub struct GraphicsEngine<'a> {
    // Current state
    state: GraphicsState<'a>,
    // Stack of states
    stack: Vec<GraphicsState<'a>>,
    // Resources needed for font lookup, etc.
    resources: &'a Resources<'a>,
    // Rendering device
    canvas: Canvas,
}

impl<'a> GraphicsEngine<'a> {
    pub fn new(clip_path: Rectangle, resources: &'a Resources<'a>) -> Self {
        let state = GraphicsState {
            clip_path,
            ..Default::default()
        };
        let stack = vec![];
        let canvas = Canvas::new();
        Self {
            state,
            stack,
            resources,
            canvas,
        }
    }

    pub fn process_operations(&mut self, operations: Vec<Operation<'_>>) {
        for operation in operations {
            self.transition(operation);
        }
    }

    fn transition(&mut self, op: Operation<'_>) {
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
            Operation::DefineMarkedContentPointPropertyList(name, dict) => {
                self.define_marked_content_point_property_list(name, dict)
            }
            Operation::BeginMarkedContentSequence(name) => self.begin_marked_content_sequence(name),
            Operation::BeginMarkedContentSequencePropertyList(name, dict) => {
                self.begin_marked_content_sequence_property_list(name, dict)
            }
            Operation::EndMarkedContentSequence => self.end_marked_content_sequence(),
            Operation::BeginInlineImageObject => self.begin_inline_image_object(),
            Operation::BeginInlineImageData => self.begin_inline_image_data(),
            Operation::EndInlineImage => self.end_inline_image(),
            Operation::BeginText => self.begin_text(),
            Operation::ShowText(bytes) => self.show_text(bytes),
            Operation::ShowTextAdjusted(vec) => self.show_text_adjusted(vec),
            Operation::MoveNextLineShowText(bytes) => self.move_next_line_show_text(bytes),
            Operation::SetSpacingMoveNextLineShowText(word_spacing, character_spacing, bytes) => {
                self.set_spacing_move_next_line_show_text(word_spacing, character_spacing, bytes)
            }
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
            Operation::SetHorizontalTextScaling(text_scaling) => {
                self.set_horizontal_text_scaling(text_scaling)
            }
            Operation::EndText => self.end_text(),
            Operation::BeginCompat => self.begin_compat(),
            Operation::SetCharWidth(horizontal_displacement, vertical_displacement) => {
                self.set_char_width(horizontal_displacement, vertical_displacement)
            }
            Operation::SetCacheDevice(s, ll, ur) => self.set_cache_device(s, ll, ur),
            Operation::InvokeXObject(name) => self.invoke_xobject(name),
            Operation::EndCompat => self.end_compat(),
            Operation::MoveTo(x, y) => self.move_to(x, y),
            Operation::LineTo(x, y) => self.line_to(x, y),
            Operation::AppendCurveThreePoints(p1, p2, p3) => {
                self.append_curve_three_points(p1, p2, p3)
            }
            Operation::AppendCurveInitialReplicated(p2, p3) => {
                self.append_curve_initial_replicated(p2, p3)
            }
            Operation::AppendCurveFinalReplicated(p1, p3) => {
                self.append_curve_final_replicated(p1, p3)
            }
            Operation::AppendRectangle(x, y, width, height) => {
                self.append_rectangle(x, y, width, height)
            }
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
            Operation::SetGraphicsStateParams(dict_name) => {
                self.set_graphics_state_params(dict_name)
            }
            Operation::SetCMYKColorStroke(color) => self.set_cmyk_color_stroke(color),
            Operation::SetCMYKColorNonstroke(color) => self.set_cmyk_color_nonstroke(color),
            Operation::SetColorStroke(color) => self.set_color_stroke(color),
            Operation::SetColorNonstroke(color) => self.set_color_nonstroke(color),
            Operation::SetColorSpecialStroke(color, name) => {
                self.set_color_special_stroke(color, name)
            }
            Operation::SetColorSpecialNonstroke(color, name) => {
                self.set_color_special_nonstroke(color, name)
            }
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
    fn close_stroke_path(&mut self) {
        self.state.path.close();
    }

    fn stroke_path(&mut self) {
        self.end_path_no_fill();
    }

    fn fill_path(&mut self) {
        self.end_path_no_fill();
    }

    fn fill_path_even_odd(&mut self) {
        self.end_path_no_fill();
    }
    fn close_fill_stroke_path(&mut self) {
        self.close_stroke_path();
        self.end_path_no_fill();
    }
    fn fill_stroke_path(&mut self) {
        // Fill
        // Stroke
        self.end_path_no_fill();
    }
    fn close_fill_stroke_path_even_odd(&mut self) {
        self.close_stroke_path();
        self.end_path_no_fill();
    }
    fn fill_stroke_path_even_odd(&mut self) {
        self.end_path_no_fill();
    }
    fn end_path_no_fill(&mut self) {
        self.clip();
        // self.state.clip_path = None;
        self.state.path = Path::default();
    }
    fn clip(&mut self) {
        self.state.clip_path = Rectangle::from(&self.state.path);
    }
    fn set_clipping_path(&mut self) {
        self.state.path_mode = PathMode::NonzeroWinding;
    }
    fn set_clipping_path_even_odd(&mut self) {
        self.state.path_mode = PathMode::EvenOdd;
    }
    // Marked content operators
    fn define_marked_content_point(&mut self, _name: &Name) {}
    fn define_marked_content_point_property_list(&mut self, _name: &Name, _dict: &Dictionary) {}
    fn begin_marked_content_sequence(&mut self, _name: &Name) {}
    fn begin_marked_content_sequence_property_list(&mut self, _name: &Name, _dict: &Dictionary) {}
    fn end_marked_content_sequence(&mut self) {}
    // Image operators
    fn begin_inline_image_object(&mut self) {}
    fn begin_inline_image_data(&mut self) {}
    fn end_inline_image(&mut self) {}
    // Text operators
    fn begin_text(&mut self) {
        // PDF 9.4.1 Table 105 Set text matrix to identity matrix for begin text
        self.state.text_matrix = Matrix::default();
        self.state.line_matrix = Matrix::default();
    }
    fn show_text(&mut self, bytes: &[u8]) {
        if let Some(Font::Type3(font)) = &self.state.text_state.font {
            // PDF 9.4.4
            let font_size = self.state.text_state.font_size;
            let hor_scaling = self.state.text_state.hor_scaling;
            let rise = self.state.text_state.rise;
            let mut rendering_matrix =
                Matrix::new([font_size * hor_scaling, 0.0, 0.0, 0.0, font_size, rise]);
            rendering_matrix.concatenate(&self.state.text_matrix);
            rendering_matrix.concatenate(&self.state.ctm);
        } else if let Some(Font::TrueType(font)) = &self.state.text_state.font {
            println!("{:?}", bytes);
            for byte in bytes {
                println!("{:?}", font.get_unicode(*byte as u32));
            }
        }
        // After rendering glyph, then update text matrix
        // let (tx, ty) = ;
    }
    fn show_text_adjusted(&mut self, vec: Vec<StringOrNumber>) {
        for element in vec {
            match element {
                StringOrNumber::String(string) => self.show_text(string),
                StringOrNumber::Number(number) => {
                    let delta = -number
                        * 0.001
                        * self.state.text_state.font_size
                        * self.state.text_state.hor_scaling;
                    self.state.current_point.x += self.state.text_matrix[0] * delta;
                    self.state.current_point.y += self.state.text_matrix[1] * delta;
                }
            }
        }
    }
    fn move_next_line_show_text(&mut self, bytes: &[u8]) {
        // PDF 9.4.3 Table 107 Equivalent to T* Tj
        // Move next line
        self.state.line_coordinates.y -= self.state.text_state.leading;
        // Show text
        self.show_text(bytes);
    }
    fn set_spacing_move_next_line_show_text(
        &mut self,
        word_spacing: UnscaledTextSpaceUnit,
        char_spacing: UnscaledTextSpaceUnit,
        bytes: &[u8],
    ) {
        // PDF 9.4.3 Table 107 Equivalent to Tw Tc '
        self.set_word_spacing(word_spacing);
        self.set_char_spacing(char_spacing);
        self.move_next_line_show_text(bytes);
    }
    fn move_text_position(&mut self, tx: UnscaledTextSpaceUnit, ty: UnscaledTextSpaceUnit) {
        self.state.line_coordinates.x += tx;
        self.state.line_coordinates.y += ty;
    }
    fn move_text_position_leading(&mut self, tx: UnscaledTextSpaceUnit, ty: UnscaledTextSpaceUnit) {
        self.set_text_leading(-ty);
        self.move_text_position(tx, ty);
    }
    fn set_text_matrix(&mut self, matrix: [f64; 6]) {
        self.state.text_matrix = Matrix::new(matrix);
    }
    fn move_start_next_line(&mut self) {
        self.set_text_leading(0. - self.state.text_state.leading);
    }
    fn set_char_spacing(&mut self, char_spacing: UnscaledTextSpaceUnit) {
        self.state.text_state.char_spacing = char_spacing;
    }
    fn select_font(&mut self, font_name: &'_ Name, font_size: f64) {
        if let Some(font_dict) = &self.resources.font {
            if let Some(font) = font_dict.get(font_name) {
                self.state.text_state.font = Some(font);
            }
        }
        self.state.text_state.font_size = font_size;
    }
    fn set_text_leading(&mut self, text_leading: UnscaledTextSpaceUnit) {
        self.state.text_state.leading = text_leading;
    }
    fn set_text_rendering(&mut self, rendering_mode: TextRendering) {
        self.state.text_state.render_mode = rendering_mode;
    }
    fn set_text_rise(&mut self, text_rise: UnscaledTextSpaceUnit) {
        self.state.text_state.rise = text_rise;
    }
    fn set_word_spacing(&mut self, word_spacing: UnscaledTextSpaceUnit) {
        self.state.text_state.word_spacing = word_spacing;
    }
    fn set_horizontal_text_scaling(&mut self, text_scaling: f64) {
        self.state.text_state.hor_scaling = text_scaling;
    }
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
    fn move_to(&mut self, x: f64, y: f64) {
        self.state.path.m(Point { x, y });
    }
    fn line_to(&mut self, x: f64, y: f64) {
        self.state.path.l(Point { x, y });
    }
    fn append_curve_three_points(
        &mut self,
        (x1, y1): (f64, f64),
        (x2, y2): (f64, f64),
        (x3, y3): (f64, f64),
    ) {
        self.state.path.c(
            Point { x: x1, y: y1 },
            Point { x: x2, y: y2 },
            Point { x: x3, y: y3 },
        );
    }
    fn append_curve_initial_replicated(&mut self, (x2, y2): (f64, f64), (x3, y3): (f64, f64)) {
        self.state
            .path
            .v(Point { x: x2, y: y2 }, Point { x: x3, y: y3 });
    }
    fn append_curve_final_replicated(&mut self, (x1, y1): (f64, f64), (x3, y3): (f64, f64)) {
        self.state
            .path
            .y(Point { x: x1, y: y1 }, Point { x: x3, y: y3 });
    }
    fn append_rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.state.path.re(x, y, width, height);
        self.canvas.draw_rectangle(&self.state);
    }
    fn close_subpath(&mut self) {
        self.state.path.close();
    }
    fn set_miter_limit(&mut self, miter_limit: f64) {
        self.state.miter_limit = miter_limit;
    }
    fn concat_matrix(&mut self, matrix: [f64; 6]) {
        let matrix = Matrix::new(matrix);
        self.state.ctm.concatenate(&matrix);
    }
    fn set_line_width(&mut self, line_width: f64) {
        self.state.line_width = line_width;
    }
    fn set_line_join(&mut self, line_join: LineJoin) {
        self.state.line_join = line_join;
    }
    fn set_line_cap(&mut self, line_cap: LineCap) {
        self.state.line_cap = line_cap;
    }
    fn set_dash(&mut self, dash_array: Vec<f64>, dash_phase: f64) {
        self.state.dash_pattern.dash_array = dash_array;
        self.state.dash_pattern.dash_phase = dash_phase;
    }
    fn g_save(&mut self) {
        self.stack.push(self.state.clone())
    }
    fn g_restore(&mut self) {
        if let Some(last) = self.stack.pop() {
            self.state = last;
        }
    }
    fn set_color_rendering_intent(&mut self, name: &Name) {}
    fn set_flat(&mut self, flatness: Percent) {
        self.state.flatness = flatness;
    }
    fn set_graphics_state_params(&mut self, dict_name: &Name) {}
    // Color operators
    fn set_cmyk_color_stroke(&mut self, color: Cmyk) {
        self.state.stroke_color = Color::Cmyk(color);
    }
    fn set_cmyk_color_nonstroke(&mut self, color: Cmyk) {
        self.state.fill_color = Color::Cmyk(color);
    }
    fn set_color_stroke(&mut self, color: Color) {
        self.state.stroke_color = color;
    }
    fn set_color_nonstroke(&mut self, color: Color) {
        self.state.fill_color = color;
    }
    fn set_color_special_stroke(&mut self, color: Color, name: Option<&Name>) {}
    fn set_color_special_nonstroke(&mut self, color: Color, name: Option<&Name>) {}
    fn set_color_space_stroke(&mut self, name: &Name) {}
    fn set_color_space_nonstroke(&mut self, name: &Name) {}
    fn set_rgb_color_stroke(&mut self, color: Rgb) {
        self.state.stroke_color = Color::Rgb(color);
    }
    fn set_rgb_color_nonstroke(&mut self, color: Rgb) {
        self.state.fill_color = Color::Rgb(color);
    }
    fn set_gray_stroke(&mut self, gray: UnitInterval) {
        self.state.stroke_color = Color::Gray(gray);
    }
    fn set_gray_nonstroke(&mut self, gray: UnitInterval) {
        self.state.fill_color = Color::Gray(gray);
    }
    // Shading operators
    fn sh_fill(&mut self, name: &Name) {}
}
