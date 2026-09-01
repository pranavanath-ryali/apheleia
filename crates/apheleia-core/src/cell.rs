bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub struct Modifiers: u8 {
        const NONE              = 0;
        const BOLD              = 1 << 0;
        const DOUBLE_UNDERLINE  = 1 << 1;
        const ITALIC            = 1 << 2;
        const UNDERLINE         = 1 << 3;
        const BLINK             = 1 << 4;
        const REVERSE           = 1 << 5;
        const CONCEAL           = 1 << 6;
        const STRIKETHROUGH     = 1 << 7;
    }
}
use bitflags::bitflags;

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
impl Color {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Reset => todo!(),
            Color::Black => todo!(),
            Color::DarkGrey => todo!(),
            Color::DarkRed => todo!(),
            Color::Red => todo!(),
            Color::DarkGreen => todo!(),
            Color::Green => todo!(),
            Color::DarkYellow => todo!(),
            Color::Yellow => todo!(),
            Color::DarkBlue => todo!(),
            Color::Blue => todo!(),
            Color::DarkMagenta => todo!(),
            Color::Magenta => todo!(),
            Color::DarkCyan => todo!(),
            Color::Cyan => todo!(),
            Color::Grey => todo!(),
            Color::White => todo!(),
            Color::Ansi(v) => todo!(),
            Color::Rgb { r, g, b } => (*r, *g, *b),
        }
    }
}

fn standard_blend(
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
        g: ((upper_color.0 as f32 * upper_alpha)
            + (lower_color.0 as f32 * lower_alpha * (1f32 - upper_alpha)) / alpha)
            .round() as u8,
        b: ((upper_color.0 as f32 * upper_alpha)
            + (lower_color.0 as f32 * lower_alpha * (1f32 - upper_alpha)) / alpha)
            .round() as u8,
    };

    (color, (alpha * 255f32).round() as u8)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub modifiers: Modifiers,
}
impl Default for Style {
    fn default() -> Self {
        Self {
            fg: Color::Reset,
            bg: Color::Reset,
            modifiers: Modifiers::NONE,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Grapheme {
    // Ascii(u8),
    Char(char),
    Width(char),
    // Extended, // TODO: todo
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Transparent,
    Opaque {
        grapheme: Grapheme,
        style: Style,
    },
    Translucent {
        grapheme: Grapheme,
        style: Style,

        /// Only affects the bg of the cell
        alpha: u8,
    },
}

pub fn update_cell(lower_cell: &Cell, upper_cell: &Cell) -> Cell {
    fn handle_lower_opaque(
        lower_grapheme: &Grapheme,
        lower_style: &Style,
        upper_cell: &Cell,
    ) -> Cell {
        match upper_cell {
            Cell::Transparent => Cell::Opaque {
                grapheme: *lower_grapheme,
                style: *lower_style,
            },
            Cell::Translucent {
                grapheme: upper_grapheme,
                style: upper_style,
                alpha,
            } => Cell::Opaque {
                grapheme: *upper_grapheme,
                style: Style {
                    fg: upper_style.fg,
                    bg: {
                        let (color, _) = standard_blend(
                            lower_style.bg.to_rgb(),
                            None,
                            upper_style.bg.to_rgb(),
                            *alpha,
                        );

                        color
                    },
                    modifiers: upper_style.modifiers,
                },
            },
            _ => *upper_cell,
        }
    }

    fn handle_lower_translucent(
        lower_graphene: &Grapheme,
        lower_style: &Style,
        lower_alpha: u8,
        upper_cell: &Cell,
    ) -> Cell {
        match upper_cell {
            Cell::Transparent => Cell::Translucent {
                grapheme: *lower_graphene,
                style: *lower_style,
                alpha: lower_alpha,
            },
            Cell::Opaque {
                grapheme: _grapheme,
                style: _style,
            } => *upper_cell,
            Cell::Translucent {
                grapheme: upper_grapheme,
                style: upper_style,
                alpha: upper_alpha,
            } => {
                let (color, alpha) = standard_blend(
                    lower_style.bg.to_rgb(),
                    Some(lower_alpha),
                    upper_style.bg.to_rgb(),
                    *upper_alpha,
                );
                Cell::Translucent {
                    grapheme: *upper_grapheme,
                    style: Style {
                        fg: upper_style.fg,
                        bg: color,
                        modifiers: upper_style.modifiers,
                    },
                    alpha: alpha,
                }
            }
        }
    }

    match lower_cell {
        Cell::Transparent => match upper_cell {
            Cell::Transparent => Cell::Transparent,
            _ => *upper_cell,
        },
        Cell::Translucent {
            grapheme,
            style,
            alpha,
        } => handle_lower_translucent(grapheme, style, *alpha, upper_cell),
        Cell::Opaque { grapheme, style } => handle_lower_opaque(grapheme, style, upper_cell),
    }
}
