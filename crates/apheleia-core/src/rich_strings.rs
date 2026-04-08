use crate::style::Style;

pub struct RichString {
    pub text: String,
}

impl RichString {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
    pub fn to_rich(text: &str, style: Style) -> Self {
        Self {
            text: format!(
                "<{};{};{}>{}",
                style.get_fg_markup(),
                style.get_bg_markup(),
                style.get_style_markup(),
                text
            )
            .to_string(),
        }
    }
}

#[cfg(test)]
mod rich_string_test {
    use crate::style::StyleFlags;

    use super::*;

    #[test]
    fn test_to_rich() {
        let rich_str = RichString::to_rich(
            "HelloWorld",
            Style {
                fg: crossterm::style::Color::Green,
                bg: crossterm::style::Color::Red,
                flags: StyleFlags::BOLD,
            },
        );

        assert_eq!(
            rich_str.text,
            "<fg:green;bg:red;bold;>HelloWorld".to_string()
        );
    }
}
