#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Color {
    Default,

    Black,
    DarkGrey,

    DarkRed,
    Red,

    DarkGreen,
    Green,

    DarkYellow,
    Yellow,

    DarkBlue,
    Blue,

    DarkMagenta,
    Magenta,

    DarkCyan,
    Cyan,

    Grey,
    White,

    Ansi(u8),
    Rgba { r: u8, g: u8, b: u8, a: u8 },
}

pub fn standard_blend(lower_color: (u8, u8, u8, u8), upper_color: (u8, u8, u8, u8)) -> Color {
    let lower_alpha = lower_color.3 as f32 / 255f32;
    let upper_alpha = upper_color.3 as f32 / 255f32;
    let alpha = upper_alpha + (lower_alpha * (1f32 - upper_alpha));

    let color = Color::Rgba {
        r: ((upper_color.0 as f32 * upper_alpha)
            + (lower_color.0 as f32 * lower_alpha * (1f32 - upper_alpha)) / alpha)
            .round() as u8,
        g: ((upper_color.1 as f32 * upper_alpha)
            + (lower_color.1 as f32 * lower_alpha * (1f32 - upper_alpha)) / alpha)
            .round() as u8,
        b: ((upper_color.2 as f32 * upper_alpha)
            + (lower_color.2 as f32 * lower_alpha * (1f32 - upper_alpha)) / alpha)
            .round() as u8,
        a: (alpha * 255f32).round() as u8,
    };

    color
}

pub enum ColorType {
    Fg,
    Bg,
}

impl Color {
    pub fn to_rgba(&self, color_type: ColorType) -> (u8, u8, u8, u8) {
        match self {
            Color::Default => match color_type {
                ColorType::Fg => (255, 255, 255, 255),
                ColorType::Bg => (0, 0, 0, 0),
            },

            Color::Black => (0, 0, 0, 255),
            Color::DarkGrey => (100, 100, 100, 255),
            Color::DarkRed => (128, 0, 0, 255),
            Color::Red => (255, 0, 0, 255),
            Color::DarkGreen => (0, 128, 0, 255),
            Color::Green => (0, 255, 0, 255),
            Color::DarkYellow => (128, 128, 0, 255),
            Color::Yellow => (255, 255, 0, 255),
            Color::DarkBlue => (0, 0, 128, 255),
            Color::Blue => (0, 0, 255, 255),
            Color::DarkMagenta => (128, 0, 128, 255),
            Color::Magenta => (255, 0, 255, 255),
            Color::DarkCyan => (0, 128, 128, 255),
            Color::Cyan => (0, 255, 255, 255),
            Color::Grey => (192, 192, 192, 255),
            Color::White => (255, 255, 255, 255),

            Color::Ansi(v) => {
                match v {
                    // Standard 16 colors
                    0 => (0, 0, 0, 255),
                    1 => (128, 0, 0, 255),
                    2 => (0, 128, 0, 255),
                    3 => (128, 128, 0, 255),
                    4 => (0, 0, 128, 255),
                    5 => (128, 0, 128, 255),
                    6 => (0, 128, 128, 255),
                    7 => (192, 192, 192, 255),
                    8 => (100, 100, 100, 255),
                    9 => (255, 0, 0, 255),
                    10 => (0, 255, 0, 255),
                    11 => (255, 255, 0, 255),
                    12 => (0, 0, 255, 255),
                    13 => (255, 0, 255, 255),
                    14 => (0, 255, 255, 255),
                    15 => (255, 255, 255, 255),
                    // 216 Color Cube (16..=231)
                    16..=231 => {
                        let i = v - 16;
                        let r = i / 36;
                        let g = (i % 36) / 6;
                        let b = i % 6;
                        let steps = [0, 95, 135, 175, 215, 255];
                        (steps[r as usize], steps[g as usize], steps[b as usize], 255)
                    }
                    // Grayscale ramp (232..=255)
                    232..=255 => {
                        let gray = 8 + (v - 232) * 10;
                        (gray, gray, gray, 255)
                    }
                }
            }
            Color::Rgba { r, g, b, a } => (*r, *g, *b, *a),
        }
    }
}
