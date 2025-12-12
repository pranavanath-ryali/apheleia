use bitflags::bitflags;
use std::fmt::Display;

use crossterm::style::{Attribute, Attributes, Color};

bitflags! {
    pub struct StyleFlags: u16 {
        const EMPTY = 0b0000000000;

        const BOLD = 0b1000000000;
        const ITALIC = 0b0100000000;
        const DIM = 0b0010000000;
        const REVERSE = 0b0001000000;
        const UNDERCURLED = 0b0000100000;
        const UNDERLINED = 0b0000010000;
        const UNDERDOTTED = 0b0000001000;
        const UNDERDASHED = 0b0000000100;
        const DOUBLE_UNDERLINED = 0b0000000010;
        const SLOW_BLINK = 0b0000000001;
    }
}

#[derive(Copy, Clone)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,

    pub dim: bool,
    pub reverse: bool,
    pub bold: bool,
    pub italic: bool,
    pub undercurled: bool,
    pub underlined: bool,
    pub underdotted: bool,
    pub underdashed: bool,
    pub double_underlined: bool,
    pub slow_blink: bool,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            fg: Color::Reset,
            bg: Color::Reset,

            dim: false,
            reverse: false,
            bold: false,
            italic: false,
            undercurled: false,
            underlined: false,
            underdotted: false,
            underdashed: false,
            double_underlined: false,
            slow_blink: false,
        }
    }
}
