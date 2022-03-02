use std::convert::TryFrom;

use crate::{
    error::PdfError,
    operators::{
        color::{Color, CMYK, RGB},
        engine::GraphicsState,
        matrix::Matrix,
        operations::{
            LineCap, LineJoin, Operation, Percent, StringOrNumber, TextRendering, UnitInterval,
            UnscaledTextSpaceUnit,
        },
        parser::OperatorParser,
        path::{Path, PathMode, Point},
        rect::Rectangle,
    },
    parser::{
        lexer::Lexer,
        parser::{Dictionary, IndirectReference, Name, Object, ObjectKind, Parser, Stream},
    },
    render::canvas::Canvas,
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
    // Graphics state
    state: GraphicsState,
    // Graphics state stack
    stack: Vec<GraphicsState>,
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
        let state = GraphicsState {
            clip_path: media_box,
            ..Default::default()
        };
        let stack = vec![];
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
            state,
            stack,
            canvas,
        }
    }

    pub fn parse_contents(&mut self) {
        let mut objects = vec![];
        if let Some(content_streams) = &self.contents {
            for content_stream in content_streams {
                let content = &content_stream.content;
                let mut lexer = Lexer::new(content);
                let tokens = lexer.lex();
                let mut parser = Parser::new(&tokens);
                objects.extend(parser.parse());
            }
        }
        let mut operator_parser = OperatorParser::new(&objects);
        let operations = operator_parser.parse();
        for operation in operations {
            self.transition(operation);
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
        for character_code in bytes {}
    }
    fn show_text_adjusted(&mut self, vec: Vec<StringOrNumber>) {}
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
    fn select_font(&mut self, font_name: &Name, font_size: f64) {
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
    fn set_cmyk_color_stroke(&mut self, color: CMYK) {
        self.state.stroke_color = Color::CMYK(color);
    }
    fn set_cmyk_color_nonstroke(&mut self, color: CMYK) {
        self.state.fill_color = Color::CMYK(color);
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
    fn set_rgb_color_stroke(&mut self, color: RGB) {
        self.state.stroke_color = Color::RGB(color);
    }
    fn set_rgb_color_nonstroke(&mut self, color: RGB) {
        self.state.fill_color = Color::RGB(color);
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
