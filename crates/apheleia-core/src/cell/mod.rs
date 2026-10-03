use crate::style::{
    Style,
    color::{Color, standard_blend},
};

pub(crate) mod layered;

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub c: char,
    pub style: Style,

    pub transparent: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            c: Default::default(),
            style: Default::default(),
            transparent: true,
        }
    }
}
impl Cell {
    pub fn update(&mut self, upper_cell: &Cell) {
        if self.transparent {
            if !upper_cell.transparent {
                *self = upper_cell.clone();
            }
            return;
        }

        if upper_cell.transparent {
            return;
        }

        self.c = upper_cell.c;
        self.style.modifiers = upper_cell.style.modifiers;
        self.style.bg = match (self.style.bg, upper_cell.style.bg) {
            (Color::Default, _) => upper_cell.style.bg,
            (_, Color::Default) => self.style.bg,
            (_, _) => standard_blend(
                self.style.bg.to_rgba(crate::style::color::ColorType::Bg),
                upper_cell
                    .style
                    .bg
                    .to_rgba(crate::style::color::ColorType::Bg),
            ),
        };
        self.style.fg = upper_cell.style.fg;
    }
}
