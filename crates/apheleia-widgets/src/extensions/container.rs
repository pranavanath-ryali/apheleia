use apheleia_core::style::Style;
use apheleia_ecs::traits::extension::Extension;

#[derive(Debug)]
pub struct BorderExtension {
    pub horizontal: char,
    pub vertical: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,

    pub style: Style,
}
impl Extension for BorderExtension {}
impl Default for BorderExtension {
    fn default() -> Self {
        BorderExtension::boxed()
    }
}
impl BorderExtension {
    pub fn boxed() -> Self {
        Self {
            horizontal: '─',
            vertical: '│',
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            style: Style::default(),
        }
    }

    pub fn rounded() -> Self {
        Self {
            horizontal: '─',
            vertical: '│',
            top_left: '╭',
            top_right: '╮',
            bottom_left: '╰',
            bottom_right: '╯',
            style: Style::default(),
        }
    }

    pub fn heavy() -> Self {
        Self {
            horizontal: '━',
            vertical: '┃',
            top_left: '┏',
            top_right: '┓',
            bottom_left: '┗',
            bottom_right: '┛',
            style: Style::default(),
        }
    }

    pub fn double() -> Self {
        Self {
            horizontal: '═',
            vertical: '║',
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
            style: Style::default(),
        }
    }
}
