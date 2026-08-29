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
pub struct Style {
    pub modifiers: Modifiers,
}
impl Default for Style {
    fn default() -> Self {
        Self {
            modifiers: Modifiers::NONE
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
    let mut cell: Cell = Cell::Transparent;
    match lower_cell {
        Cell::Transparent => cell = *upper_cell,
        Cell::Opaque { grapheme, style } => {}
        Cell::Translucent {
            grapheme,
            style,
            alpha,
        } => {}
    };

    cell
}
