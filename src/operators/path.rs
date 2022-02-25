use super::rect::Rectangle;

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

// PDF 8.5.3.3.1
#[derive(Debug, Clone, Copy)]
pub enum PathMode {
    NonzeroWinding,
    EvenOdd,
}

impl Default for PathMode {
    fn default() -> Self {
        Self::NonzeroWinding
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    pub subpaths: Vec<Subpath>,
    pub current_point: Point,
}

impl Path {
    pub fn m(&mut self, to: Point) {
        let subpath = Subpath {
            segments: vec![],
            closed: false,
        };
        self.subpaths.push(subpath);
        self.current_point = to;
    }

    pub fn l(&mut self, to: Point) {
        if let Some(subpaths) = self.subpaths.last_mut() {
            subpaths.append_line(self.current_point, to);
            self.current_point = to;
        }
    }

    fn append_curve(&mut self, p1: Point, p2: Point, p3: Point) {
        if let Some(subpaths) = self.subpaths.last_mut() {
            subpaths.append_curve(self.current_point, p1, p2, p3);
            self.current_point = p3;
        }
    }

    pub fn c(&mut self, p1: Point, p2: Point, p3: Point) {
        self.append_curve(p1, p2, p3);
    }

    pub fn v(&mut self, p2: Point, p3: Point) {
        self.append_curve(self.current_point, p2, p3);
    }

    pub fn y(&mut self, p1: Point, p3: Point) {
        self.append_curve(p1, p3, p3);
    }

    pub fn close(&mut self) {
        if let Some(subpaths) = self.subpaths.last_mut() {
            subpaths.close(self.current_point);
        }
    }

    // PDF 8.5.2.1 Table 58
    pub fn re(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let to = Point { x, y };
        self.m(to);
        let to = Point { x: x + width, y };
        self.l(to);
        let to = Point {
            x: x + width,
            y: y + height,
        };
        self.l(to);
        let to = Point { x, y: y + height };
        self.l(to);
        self.close();
    }
}

impl Default for Path {
    fn default() -> Self {
        Self {
            subpaths: vec![],
            current_point: Point { x: 0., y: 0. },
        }
    }
}

impl From<&Path> for Rectangle {
    fn from(path: &Path) -> Self {
        let rectangles: Vec<Rectangle> = path
            .subpaths
            .iter()
            .map(|subpath| Rectangle::from(subpath))
            .collect();
        let lower_left_x = rectangles
            .iter()
            .map(|rectangle| rectangle.lower_left_x)
            .reduce(f64::min)
            .unwrap_or_default();
        let lower_left_y = rectangles
            .iter()
            .map(|rectangle| rectangle.lower_left_y)
            .reduce(f64::min)
            .unwrap_or_default();
        let upper_right_x = rectangles
            .iter()
            .map(|rectangle| rectangle.upper_right_x)
            .reduce(f64::max)
            .unwrap_or_default();
        let upper_right_y = rectangles
            .iter()
            .map(|rectangle| rectangle.upper_right_y)
            .reduce(f64::max)
            .unwrap_or_default();
        Rectangle {
            lower_left_x,
            lower_left_y,
            upper_right_x,
            upper_right_y,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    pub from: Point,
    pub to: Point,
}

#[derive(Debug, Clone)]
pub struct Curve {
    pub line: Line,
    pub control_points: (Point, Point),
}

#[derive(Debug, Clone)]
pub enum PathSegment {
    Line(Line),
    Curve(Curve),
}

#[derive(Debug, Clone)]
pub struct Subpath {
    pub segments: Vec<PathSegment>,
    pub closed: bool,
}

impl Subpath {
    pub fn append_line(&mut self, from: Point, to: Point) {
        if self.closed {
            return;
        }
        let line = Line { from, to };
        self.segments.push(PathSegment::Line(line));
    }

    pub fn append_curve(&mut self, p0: Point, p1: Point, p2: Point, p3: Point) {
        if self.closed {
            return;
        }
        let curve = Curve {
            line: Line { from: p0, to: p3 },
            control_points: (p1, p2),
        };
        self.segments.push(PathSegment::Curve(curve));
    }

    pub fn close(&mut self, from: Point) {
        if self.closed {
            return;
        }
        if let Some(starting) = self.segments.first() {
            let to = match starting {
                PathSegment::Line(line) => line.from,
                PathSegment::Curve(curve) => curve.line.from,
            };
            self.append_line(from, to);
        }
        self.closed = true;
    }
}

impl From<&Subpath> for Rectangle {
    fn from(subpath: &Subpath) -> Self {
        let lower_left_x = subpath
            .segments
            .iter()
            .map(|segment| match segment {
                &PathSegment::Line(Line { from, to }) => f64::min(from.x, to.x),
                &PathSegment::Curve(Curve {
                    line: Line { from, to },
                    ..
                }) => f64::min(from.x, to.x),
            })
            .reduce(f64::min)
            .unwrap_or_default();
        let upper_right_x = subpath
            .segments
            .iter()
            .map(|segment| match segment {
                &PathSegment::Line(Line { from, to }) => f64::max(from.x, to.x),
                &PathSegment::Curve(Curve {
                    line: Line { from, to },
                    ..
                }) => f64::max(from.x, to.x),
            })
            .reduce(f64::max)
            .unwrap_or_default();
        let lower_left_y = subpath
            .segments
            .iter()
            .map(|segment| match segment {
                &PathSegment::Line(Line { from, to }) => f64::min(from.y, to.y),
                &PathSegment::Curve(Curve {
                    line: Line { from, to },
                    ..
                }) => f64::min(from.y, to.y),
            })
            .reduce(f64::min)
            .unwrap_or_default();
        let upper_right_y = subpath
            .segments
            .iter()
            .map(|segment| match segment {
                &PathSegment::Line(Line { from, to }) => f64::max(from.y, to.y),
                &PathSegment::Curve(Curve {
                    line: Line { from, to },
                    ..
                }) => f64::max(from.y, to.y),
            })
            .reduce(f64::max)
            .unwrap_or_default();
        Rectangle {
            lower_left_x,
            lower_left_y,
            upper_right_x,
            upper_right_y,
        }
    }
}
