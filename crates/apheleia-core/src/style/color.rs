#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Color {
    Reset,

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
    Rgb { r: u8, g: u8, b: u8 },
}

pub fn standard_blend(
    lower_color: (u8, u8, u8),
    lower_alpha: Option<u8>,
    upper_color: (u8, u8, u8),
    upper_alpha: u8,
) -> (Color, u8) {
    let lower_alpha = lower_alpha.unwrap_or(255) as f32 / 255f32;
    let upper_alpha = upper_alpha as f32 / 255f32;
    let alpha = upper_alpha + (lower_alpha * (1f32 - upper_alpha));

    let color = Color::Rgb {
        r: ((upper_color.0 as f32 * upper_alpha)
            + (lower_color.0 as f32 * lower_alpha * (1f32 - upper_alpha)) / alpha)
            .round() as u8,
        g: ((upper_color.1 as f32 * upper_alpha)
            + (lower_color.1 as f32 * lower_alpha * (1f32 - upper_alpha)) / alpha)
            .round() as u8,
        b: ((upper_color.2 as f32 * upper_alpha)
            + (lower_color.2 as f32 * lower_alpha * (1f32 - upper_alpha)) / alpha)
            .round() as u8,
    };

    (color, (alpha * 255f32).round() as u8)
}

impl Color {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Reset => panic!("to_rgb used on Color::Reset"),

            Color::Black => (0, 0, 0),
            Color::DarkGrey => (100, 100, 100),
            Color::DarkRed => (128, 0, 0),
            Color::Red => (255, 0, 0),
            Color::DarkGreen => (0, 128, 0),
            Color::Green => (0, 255, 0),
            Color::DarkYellow => (128, 128, 0),
            Color::Yellow => (255, 255, 0),
            Color::DarkBlue => (0, 0, 128),
            Color::Blue => (0, 0, 255),
            Color::DarkMagenta => (128, 0, 128),
            Color::Magenta => (255, 0, 255),
            Color::DarkCyan => (0, 128, 128),
            Color::Cyan => (0, 255, 255),
            Color::Grey => (192, 192, 192),
            Color::White => (255, 255, 255),

            Color::Ansi(v) => {
                match v {
                    // Standard 16 colors
                    0 => (0, 0, 0),
                    1 => (128, 0, 0),
                    2 => (0, 128, 0),
                    3 => (128, 128, 0),
                    4 => (0, 0, 128),
                    5 => (128, 0, 128),
                    6 => (0, 128, 128),
                    7 => (192, 192, 192),
                    8 => (100, 100, 100),
                    9 => (255, 0, 0),
                    10 => (0, 255, 0),
                    11 => (255, 255, 0),
                    12 => (0, 0, 255),
                    13 => (255, 0, 255),
                    14 => (0, 255, 255),
                    15 => (255, 255, 255),
                    // 216 Color Cube (16..=231)
                    16..=231 => {
                        let i = v - 16;
                        let r = i / 36;
                        let g = (i % 36) / 6;
                        let b = i % 6;
                        let steps = [0, 95, 135, 175, 215, 255];
                        (steps[r as usize], steps[g as usize], steps[b as usize])
                    }
                    // Grayscale ramp (232..=255)
                    232..=255 => {
                        let gray = 8 + (v - 232) * 10;
                        (gray, gray, gray)
                    }
                }
            }
            Color::Rgb { r, g, b } => (*r, *g, *b),
        }
    }
}

