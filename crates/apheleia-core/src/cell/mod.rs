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
                upper_cell.style.bg.to_rgba(crate::style::color::ColorType::Bg),
            ),
        };
        self.style.fg = upper_cell.style.fg;
    }

    // pub fn update_cell(&self, upper_cell: &Cell) -> Cell {
    //     match (self, upper_cell) {
    //         (Cell::Transparent, _) => upper_cell.clone(),
    //         (
    //             _,
    //             Cell::Opaque {
    //                 grapheme: _grapheme,
    //                 style: _style,
    //             },
    //         ) => upper_cell.clone(),
    //         (_, Cell::Transparent) => self.clone(),
    //
    //         (
    //             Cell::Opaque {
    //                 grapheme: _grapheme,
    //                 style: lower_style,
    //             },
    //             Cell::Translucent {
    //                 grapheme: upper_grapheme,
    //                 style: upper_style,
    //                 alpha,
    //             },
    //         ) => Cell::Opaque {
    //             grapheme: *upper_grapheme,
    //             style: Style {
    //                 fg: upper_style.fg,
    //                 bg: {
    //                     if Color::Reset == upper_style.bg {
    //                         lower_style.bg
    //                     } else {
    //                         let (color, _) = standard_blend(
    //                             lower_style.bg.to_rgb(),
    //                             None,
    //                             upper_style.bg.to_rgb(),
    //                             *alpha,
    //                         );
    //
    //                         color
    //                     }
    //                 },
    //                 modifiers: upper_style.modifiers,
    //             },
    //         },
    //         (
    //             Cell::Translucent {
    //                 grapheme: _grapheme,
    //                 style: lower_style,
    //                 alpha: lower_alpha,
    //             },
    //             Cell::Translucent {
    //                 grapheme: upper_grapheme,
    //                 style: upper_style,
    //                 alpha: upper_alpha,
    //             },
    //         ) => {
    //             // TODO: Fix case when lower/upper.bg might be Reset. Treat Reset as transparent
    //             let (color, alpha) = standard_blend(
    //                 lower_style.bg.to_rgb(),
    //                 Some(*lower_alpha),
    //                 upper_style.bg.to_rgb(),
    //                 *upper_alpha,
    //             );
    //             Cell::Translucent {
    //                 grapheme: *upper_grapheme,
    //                 style: Style {
    //                     fg: upper_style.fg,
    //                     bg: color,
    //                     modifiers: upper_style.modifiers,
    //                 },
    //                 alpha,
    //             }
    //         }
    //     }
    // }
}
