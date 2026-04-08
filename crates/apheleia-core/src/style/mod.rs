use bitflags::bitflags;
use crossterm::style::Color;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct StyleFlags: u16 {
        const BOLD = 0b1000000000;
        const ITALIC = 0b0100000000;
        const DIM = 0b0010000000;
        const REVERSE = 0b0001000000;
        const UNDER_CURLED = 0b0000100000;
        const UNDER_LINED = 0b0000010000;
        const UNDER_DOTTED = 0b0000001000;
        const UNDER_DASHED = 0b0000000100;
        const DOUBLE_UNDERLINED = 0b0000000010;
        const SLOW_BLINK = 0b0000000001;
    }
}

#[derive(Clone, Copy)]
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
            flags: StyleFlags::empty(),
        }
    }
}
impl Style {
    pub fn get_fg_markup(&self) -> String {
        get_markup_for_color(self.fg)
    }

    pub fn get_bg_markup(&self) -> String {
        get_markup_for_color(self.bg)
    }

    pub fn get_style_markup(&self) -> String {
        let mut text: String = "".to_string();

        if self.flags.contains(StyleFlags::BOLD) {
            text.push_str("bold;");
        }
        if self.flags.contains(StyleFlags::ITALIC) {
            text.push_str("italic;");
        }
        if self.flags.contains(StyleFlags::DIM) {
            text.push_str("dim;");
        }
        if self.flags.contains(StyleFlags::REVERSE) {
            text.push_str("reverse;");
        }
        if self.flags.contains(StyleFlags::UNDER_CURLED) {
            text.push_str("under_curled;");
        }
        if self.flags.contains(StyleFlags::UNDER_LINED) {
            text.push_str("under_lined;");
        }
        if self.flags.contains(StyleFlags::UNDER_DOTTED) {
            text.push_str("under_dotted;");
        }
        if self.flags.contains(StyleFlags::UNDER_DASHED) {
            text.push_str("under_dashed;");
        }
        if self.flags.contains(StyleFlags::DOUBLE_UNDERLINED) {
            text.push_str("double_underlined;");
        }
        if self.flags.contains(StyleFlags::SLOW_BLINK) {
            text.push_str("slow_blink;");
        }

        text
    }
}

fn get_markup_for_color(color: Color) -> String {
    let text: String;
    match color {
        Color::Reset => text = "fg:reset".to_string(),
        Color::Black => text = "fg:black".to_string(),
        Color::DarkGrey => text = "fg:dark_grey".to_string(),
        Color::Red => text = "fg:red".to_string(),
        Color::DarkRed => text = "fg:dark_red".to_string(),
        Color::Green => text = "fg:green".to_string(),
        Color::DarkGreen => text = "fg:dark_green".to_string(),
        Color::Yellow => text = "fg:yellow".to_string(),
        Color::DarkYellow => text = "fg:dark_yellow".to_string(),
        Color::Blue => text = "fg:blue".to_string(),
        Color::DarkBlue => text = "fg:dark_blue".to_string(),
        Color::Magenta => text = "fg:magenta".to_string(),
        Color::DarkMagenta => text = "fg:dark_magenta".to_string(),
        Color::Cyan => text = "fg:cyan".to_string(),
        Color::DarkCyan => text = "fg:dark_cyan".to_string(),
        Color::White => text = "fg:white".to_string(),
        Color::Grey => text = "fg:grey".to_string(),

        Color::Rgb { r, g, b } => todo!(),
        Color::AnsiValue(v) => todo!(),
        // Color::Rgb { r, g, b } => text = format!("fg:rgb({}, {}, {})", r, g, b),
        // Color::AnsiValue(v) => text = format!("fg:ansi({})", v),
    }

    text
}

#[cfg(test)]
mod style_tests {
    use super::*;

    #[test]
    fn test_markup() {
        let style_opts = Style {
            fg: Color::Cyan,
            bg: Color::DarkBlue,
            flags: StyleFlags::BOLD | StyleFlags::ITALIC | StyleFlags::REVERSE,
        };

        assert_eq!(style_opts.get_fg_markup(), "fg:cyan");
        assert_eq!(style_opts.get_bg_markup(), "fg:dark_blue");
        assert_eq!(style_opts.get_style_markup(), "bold;italic;reverse;")
    }
}
