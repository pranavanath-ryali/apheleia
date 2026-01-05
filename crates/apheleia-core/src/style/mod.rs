use bitflags::bitflags;
use std::fmt::Display;

use crossterm::style::{Attribute, Attributes, Color};

bitflags! {
    #[derive(Copy, Clone)]
    pub struct StyleFlags: u16 {
        const Empty = 0b0000000000;

        const Bold = 0b1000000000;
        const Italic = 0b0100000000;
        const Dim = 0b0010000000;
        const Reverse = 0b0001000000;
        const UnderCurled = 0b0000100000;
        const UnderLined = 0b0000010000;
        const UnderDotted = 0b0000001000;
        const UnderDashed = 0b0000000100;
        const DoubleUnderLined = 0b0000000010;
        const SlowBlink = 0b0000000001;
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

            flags: StyleFlags::Empty,
        }
    }
}
