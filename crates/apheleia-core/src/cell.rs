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
    Ascii(u8),
    Char(char),
    Width(char),
    Extended, // TODO: todo
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
        alpha: u8,
    },
}

pub fn update_cell(lower_cell: &Cell, upper_cell: &Cell) -> Cell {
    let mut cell: Cell = match lower_cell {
        Cell::Transparent => *upper_cell,
        Cell::Opaque {
            grapheme: _l_grapheme,
            style: l_style,
        } => match upper_cell {
            Cell::Transparent => *lower_cell,
            Cell::Opaque {
                grapheme: u_grapheme,
                style: u_style,
            } => Cell::Opaque {
                grapheme: *u_grapheme,
                style: Style {
                    fg: u_style.fg,
                    bg: if u_style.bg == Color::Reset {
                        l_style.bg
                    } else {
                        u_style.bg
                    },
                    modifiers: u_style.modifiers,
                },
            },
            Cell::Translucent {
                grapheme,
                style,
                alpha,
            } => todo!(),
        },
        Cell::Translucent {
            grapheme,
            style,
            alpha,
        } => todo!(),
    };

    cell
}
