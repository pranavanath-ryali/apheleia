use crate::style::{color::Color, modifiers::Modifiers};

pub mod modifiers;
pub mod color;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,

    pub modifiers: Modifiers
}
impl Default for Style {
    fn default() -> Self {
        Self {
            fg: Color::Default,
            bg: Color::Default,
            modifiers: Modifiers::empty()
        }
    }
}

