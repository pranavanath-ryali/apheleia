use crate::style::{color::Color, modifiers::Modifiers};

pub mod modifiers;
pub mod color;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}
impl Default for Style {
    fn default() -> Self {
        Self {
            fg: None,
            bg: None,
        }
    }
}

