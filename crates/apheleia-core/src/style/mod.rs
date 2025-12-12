use bitflags::bitflags;
use std::fmt::Display;

use crossterm::style::{Attribute, Attributes, Color};

bitflags! {
    #[derive(Copy, Clone)]
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

    pub flags: StyleFlags,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            fg: Color::Reset,
            bg: Color::Reset,

            flags: StyleFlags::EMPTY,
        }
    }
}
