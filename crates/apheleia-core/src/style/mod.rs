use crate::style::{color::Color, modifiers::Modifiers};

pub mod modifiers;
pub mod color;

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

