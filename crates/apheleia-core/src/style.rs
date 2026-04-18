use bitflags::bitflags;
use crossterm::style::Color;

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub fn update(&mut self, style: Style) {
        self.flags |= style.flags;
        if style.fg != Color::Reset {
            self.fg = style.fg;
        }
        if style.bg != Color::Reset {
            self.bg = style.bg;
        }
    }

    pub fn from_markup(markup: &str) -> Self {
        let mut fg: Color = Color::Reset;
        let mut bg: Color = Color::Reset;
        let mut flags: StyleFlags = StyleFlags::empty();

        for field in markup.split(';') {
            let field = field.trim();
            let words: Vec<&str> = field.split(':').collect();
            let key = words[0].trim().to_ascii_lowercase();

            if key.eq("fg") {
                let value = words[1].trim().to_ascii_lowercase();
                fg = parse_color(value.as_str());
            } else if key.eq("bg") {
                let value = words[1].trim().to_ascii_lowercase();
                bg = parse_color(value.as_str());
            } else if key.eq("bold") || key.eq("b") {
                flags |= StyleFlags::BOLD;
            } else if key.eq("italic") || key.eq("i") {
                flags |= StyleFlags::ITALIC;
            } else if key.eq("dim") {
                flags |= StyleFlags::DIM;
            } else if key.eq("reverse") {
                flags |= StyleFlags::REVERSE;
            } else if key.eq("under_curled") {
                flags |= StyleFlags::UNDER_CURLED;
            } else if key.eq("under_lined") || key.eq("u") {
                flags |= StyleFlags::UNDER_LINED;
            } else if key.eq("under_dotted") {
                flags |= StyleFlags::UNDER_DOTTED;
            } else if key.eq("under_dashed") {
                flags |= StyleFlags::UNDER_DASHED;
            } else if key.eq("double_underlined") {
                flags |= StyleFlags::DOUBLE_UNDERLINED;
            } else if key.eq("slow_blink") {
                flags |= StyleFlags::SLOW_BLINK;
            }
        }

        Self { fg, bg, flags }
    }

    pub fn get_fg_markup(&self) -> String {
        if self.fg == Color::Reset {
            return "".to_string();
        }
        format!("fg:{};", get_markup_for_color(self.fg))
    }

    pub fn get_bg_markup(&self) -> String {
        if self.bg == Color::Reset {
            return "".to_string();
        }
        format!("bg:{};", get_markup_for_color(self.bg))
    }

    pub fn get_flags_markup(&self) -> String {
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

fn parse_color(text: &str) -> Color {
    match text {
        "reset" => Color::Reset,
        "black" => Color::Black,
        "dark_grey" => Color::DarkGrey,
        "red" => Color::Red,
        "dark_red" => Color::DarkRed,
        "green" => Color::Green,
        "dark_green" => Color::DarkGreen,
        "yellow" => Color::Yellow,
        "dark_yellow" => Color::DarkYellow,
        "blue" => Color::Blue,
        "dark_blue" => Color::DarkBlue,
        "magenta" => Color::Magenta,
        "dark_magenta" => Color::DarkMagenta,
        "cyan" => Color::Cyan,
        "dark_cyan" => Color::DarkCyan,
        "white" => Color::White,
        "grey" => Color::Grey,

        text => {
            if text.starts_with("rgb") {
                let params = text.split_at(3).1.trim();
                if params.starts_with('(') && params.ends_with(')') {
                    let mut iter = params[1..params.len() - 1].split(',').map(|t| t.trim());
                    let r: u8 = iter
                        .next()
                        .expect("Expected atleast 3 parameters")
                        .parse()
                        .expect("No a valid u8 number");
                    let g: u8 = iter
                        .next()
                        .expect("Expected atleast 3 parameters")
                        .parse()
                        .expect("No a valid u8 number");
                    let b: u8 = iter
                        .next()
                        .expect("Expected atleast 3 parameters")
                        .parse()
                        .expect("No a valid u8 number");
                    return Color::Rgb { r, g, b };
                }
            }
            Color::Reset
        }
    }
}

fn get_markup_for_color(color: Color) -> String {
    let text: String = match color {
        Color::Reset => "reset".to_string(),
        Color::Black => "black".to_string(),
        Color::DarkGrey => "dark_grey".to_string(),
        Color::Red => "red".to_string(),
        Color::DarkRed => "dark_red".to_string(),
        Color::Green => "green".to_string(),
        Color::DarkGreen => "dark_green".to_string(),
        Color::Yellow => "yellow".to_string(),
        Color::DarkYellow => "dark_yellow".to_string(),
        Color::Blue => "blue".to_string(),
        Color::DarkBlue => "dark_blue".to_string(),
        Color::Magenta => "magenta".to_string(),
        Color::DarkMagenta => "dark_magenta".to_string(),
        Color::Cyan => "cyan".to_string(),
        Color::DarkCyan => "dark_cyan".to_string(),
        Color::White => "white".to_string(),
        Color::Grey => "grey".to_string(),

        Color::Rgb { r, g, b } => format!("rgb({},{},{})", r, g, b).to_string(),
        Color::AnsiValue(v) => todo!(),
        // Color::Rgb { r, g, b } => text = format!("rgb({}, {}, {})", r, g, b),
        // Color::AnsiValue(v) => text = format!("ansi({})", v),
    };
    text
}

#[cfg(test)]
mod style_tests {
    use super::*;

    #[test]
    fn test_style_to_markup() {
        let style_opts = Style {
            fg: Color::Cyan,
            bg: Color::DarkBlue,
            flags: StyleFlags::BOLD | StyleFlags::ITALIC | StyleFlags::REVERSE,
        };

        assert_eq!(style_opts.get_fg_markup(), "fg:cyan;");
        assert_eq!(style_opts.get_bg_markup(), "bg:dark_blue;");
        assert_eq!(style_opts.get_flags_markup(), "bold;italic;reverse;")
    }

    #[test]
    fn test_markup_to_style() {
        assert_eq!(
            Style::from_markup("bold;italic;under_lined;reverse"),
            Style {
                flags: StyleFlags::BOLD
                    | StyleFlags::ITALIC
                    | StyleFlags::UNDER_LINED
                    | StyleFlags::REVERSE,
                ..Default::default()
            }
        );

        // Test All Foreground Colors
        assert_eq!(
            Style::from_markup("fg:reset"),
            Style {
                fg: Color::Reset,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:black"),
            Style {
                fg: Color::Black,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:dark_grey"),
            Style {
                fg: Color::DarkGrey,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:red"),
            Style {
                fg: Color::Red,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:dark_red"),
            Style {
                fg: Color::DarkRed,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:green"),
            Style {
                fg: Color::Green,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:dark_green"),
            Style {
                fg: Color::DarkGreen,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:yellow"),
            Style {
                fg: Color::Yellow,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:dark_yellow"),
            Style {
                fg: Color::DarkYellow,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:blue"),
            Style {
                fg: Color::Blue,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:dark_blue"),
            Style {
                fg: Color::DarkBlue,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:magenta"),
            Style {
                fg: Color::Magenta,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:dark_magenta"),
            Style {
                fg: Color::DarkMagenta,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:cyan"),
            Style {
                fg: Color::Cyan,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:dark_cyan"),
            Style {
                fg: Color::DarkCyan,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:white"),
            Style {
                fg: Color::White,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("fg:grey"),
            Style {
                fg: Color::Grey,
                ..Default::default()
            }
        );

        // Test All bg colors
        assert_eq!(
            Style::from_markup("bg:reset"),
            Style {
                bg: Color::Reset,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:black"),
            Style {
                bg: Color::Black,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:dark_grey"),
            Style {
                bg: Color::DarkGrey,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:red"),
            Style {
                bg: Color::Red,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:dark_red"),
            Style {
                bg: Color::DarkRed,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:green"),
            Style {
                bg: Color::Green,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:dark_green"),
            Style {
                bg: Color::DarkGreen,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:yellow"),
            Style {
                bg: Color::Yellow,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:dark_yellow"),
            Style {
                bg: Color::DarkYellow,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:blue"),
            Style {
                bg: Color::Blue,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:dark_blue"),
            Style {
                bg: Color::DarkBlue,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:magenta"),
            Style {
                bg: Color::Magenta,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:dark_magenta"),
            Style {
                bg: Color::DarkMagenta,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:cyan"),
            Style {
                bg: Color::Cyan,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:dark_cyan"),
            Style {
                bg: Color::DarkCyan,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:white"),
            Style {
                bg: Color::White,
                ..Default::default()
            }
        );
        assert_eq!(
            Style::from_markup("bg:grey"),
            Style {
                bg: Color::Grey,
                ..Default::default()
            }
        );

        // WHY? For fun

        assert_eq!(
            Style::from_markup("bold;i;fg:Red;bg:WhIte"),
            Style {
                fg: Color::Red,
                bg: Color::White,
                flags: StyleFlags::BOLD | StyleFlags::ITALIC
            }
        );
    }
}
