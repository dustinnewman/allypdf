use super::operations::UnitInterval;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rgb {
    pub red: UnitInterval,
    pub green: UnitInterval,
    pub blue: UnitInterval,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cmyk {
    pub cyan: UnitInterval,
    pub magenta: UnitInterval,
    pub yellow: UnitInterval,
    pub black: UnitInterval,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Gray(UnitInterval),
    Rgb(Rgb),
    Cmyk(Cmyk),
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

fn max(a: f64, b: f64) -> f64 {
    if a < b {
        b
    } else {
        a
    }
}

impl From<UnitInterval> for Rgb {
    // PDF 10.4.2.1 Conversion between DeviceGray and DeviceRGB
    fn from(gray: UnitInterval) -> Self {
        Self {
            red: gray,
            green: gray,
            blue: gray,
        }
    }
}

impl From<Rgb> for UnitInterval {
    // PDF 10.4.2.1 Conversion between DeviceGray and DeviceRGB
    fn from(Rgb { red, green, blue }: Rgb) -> Self {
        // NTSC video standard conversion from RGB to gray
        0.3 * red + 0.59 * green + 0.11 * blue
    }
}

impl From<UnitInterval> for Cmyk {
    // PDF 10.4.2.2 Conversion between DeviceGray and DeviceCMYK
    fn from(gray: UnitInterval) -> Self {
        Self {
            cyan: 0.,
            magenta: 0.,
            yellow: 0.,
            black: 1. - gray,
        }
    }
}

impl From<Cmyk> for UnitInterval {
    // PDF 10.4.2.2 Conversion between DeviceGray and DeviceCMYK
    fn from(
        Cmyk {
            cyan,
            magenta,
            yellow,
            black,
        }: Cmyk,
    ) -> Self {
        1. - min(1., 0.3 * cyan + 0.59 * magenta + 0.11 * yellow + black)
    }
}

impl From<Rgb> for Cmyk {
    // PDF 10.4.2.3 Conversion from DeviceRGB to DeviceCMYK
    fn from(Rgb { red, green, blue }: Rgb) -> Self {
        let c = 1. - red;
        let m = 1. - green;
        let y = 1. - blue;
        let k = min(min(c, m), y);
        let cyan = min(1., max(0., c - k));
        let magenta = min(1., max(0., m - k));
        let yellow = min(1., max(0., y - k));
        let black = min(1., max(0., k));
        Self {
            cyan,
            magenta,
            yellow,
            black,
        }
    }
}

impl From<Cmyk> for Rgb {
    // PDF 10.4.2.4 Conversion from DeviceCMYK to DeviceRGB
    fn from(
        Cmyk {
            cyan,
            magenta,
            yellow,
            black,
        }: Cmyk,
    ) -> Self {
        let red = 1. - min(1., cyan + black);
        let green = 1. - min(1., magenta + black);
        let blue = 1. - min(1., yellow + black);
        Self { red, green, blue }
    }
}

impl Default for Color {
    fn default() -> Self {
        // PDF 8.4.1 Table 51
        Color::Gray(0.)
    }
}
