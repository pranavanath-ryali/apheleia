use std::fmt::Display;

use crossterm::style::{Attribute, Attributes, Color};

#[derive(Copy, Clone)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub attrs: Option<Attribute>,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            fg: Color::White,
            bg: Color::Black,
            attrs: None,
        }
    }
}
