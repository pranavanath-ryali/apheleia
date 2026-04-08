use std::collections::BTreeSet;

use crate::style::Style;

pub struct RichString {
    text: String,
    i_text: BTreeSet<usize>,
}

impl RichString {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            i_text: map_text_index(text),
        }
    }
    pub fn to_rich(text: &str, style: Style) -> Self {
        let text = format!(
            "<{}{}{}>{}",
            style.get_fg_markup(),
            style.get_bg_markup(),
            style.get_style_markup(),
            text
        )
        .to_string();
        let i_text = map_text_index(text.as_str());

        Self { text, i_text }
    }
}

fn map_text_index(text: &str) -> BTreeSet<usize> {
    let mut i_text: BTreeSet<usize> = BTreeSet::new();
    let mut is_in_tag = false;
    for (i, c) in text.chars().enumerate() {
        match c {
            '<' => is_in_tag = true,
            '>' => is_in_tag = false,
            _ => {
                if !is_in_tag {
                    i_text.insert(i);
                }
            }
        }
    }
    i_text
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

    #[test]
    fn test_text_index_set() {
        let rich_str0 = RichString::new("Hello<red>Wor<i>ld");
        let rich_str1 = RichString::to_rich(
            "Hello<red>Wor<i>ld",
            Style {
                flags: StyleFlags::BOLD,
                ..Default::default()
            },
        );

        let i_text0: Vec<usize> = rich_str0.i_text.into_iter().collect();
        let i_text1: Vec<usize> = rich_str1.i_text.into_iter().collect();

        println!("Rich String 0 Text: {}", rich_str0.text);
        println!("Rich String 1 Text: {}", rich_str1.text);

        assert_eq!(i_text0, vec![0, 1, 2, 3, 4, 10, 11, 12, 16, 17]);
        assert_eq!(i_text1, vec![7, 8, 9, 10, 11, 17, 18, 19, 23, 24]);
    }
}
