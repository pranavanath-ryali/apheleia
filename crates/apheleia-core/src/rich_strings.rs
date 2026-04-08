use std::{
    collections::{BTreeSet, btree_set},
    vec,
};

use crate::style::Style;

#[derive(Debug)]
pub struct RichString {
    text: String,
    i_text: BTreeSet<usize>,
    ij_markup: Vec<(usize, usize)>,
}
impl RichString {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            i_text: map_text_index(text),
            ij_markup: map_markup_index(text),
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
        let ij_markup = map_markup_index(text.as_str());

        Self {
            text,
            i_text,
            ij_markup,
        }
    }

    pub fn iter(&self) -> RichStringIter<'_> {
        RichStringIter {
            chars: self.text.chars().collect(),
            i_text_iter: self.i_text.iter(),
            ij_markup: &self.ij_markup,
        }
    }
}

pub struct RichStringIter<'a> {
    chars: Vec<char>,
    i_text_iter: btree_set::Iter<'a, usize>,
    ij_markup: &'a Vec<(usize, usize)>,
}
impl<'a> Iterator for RichStringIter<'a> {
    type Item = (char, Style);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.i_text_iter.next() {
            let c = self.chars.get(*index).expect("Unexpected char not found.");
            let markup: Option<(usize, usize)> = self.ij_markup.iter().find_map(|(i, j)| {
                if i > index {
                    return None;
                }
                Some((*i, *j))
            });
            let style = match markup {
                Some(markup) => {
                    let markup_text = self.chars[markup.0..markup.1].iter().collect::<String>();
                    Style::from_markup(markup_text.as_str())
                }
                None => Style::default(),
            };

            return Some((*c, style));
        }
        None
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
fn map_markup_index(text: &str) -> Vec<(usize, usize)> {
    let mut ij_markup: Vec<(usize, usize)> = vec![];

    let mut start: usize = 0;
    for (i, c) in text.chars().enumerate() {
        match c {
            '<' => start = i,
            '>' => {
                ij_markup.push((start + 1, i));
            }
            _ => (),
        }
    }

    ij_markup.reverse();
    ij_markup
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
    fn test_map_text_index() {
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

    #[test]
    fn test_map_markup_index() {
        let rich_str = RichString::new("Hello<red>Wor<i>ld");
        let ij_markup: Vec<(usize, usize)> = rich_str.ij_markup;

        println!("Rich String Text: {}", rich_str.text);

        assert_eq!(ij_markup, vec![(14, 15), (6, 9)]);
    }

    #[test]
    fn test_richstring_iter() {
        let rich_str = RichString::new("He<bold>llo");
        let vec: Vec<(char, Style)> = rich_str.iter().collect();

        assert_eq!(
            vec,
            vec![
                (
                    'H',
                    Style {
                        ..Default::default()
                    }
                ),
                (
                    'e',
                    Style {
                        ..Default::default()
                    }
                ),
                (
                    'l',
                    Style {
                        flags: StyleFlags::BOLD,
                        ..Default::default()
                    }
                ),
                (
                    'l',
                    Style {
                        flags: StyleFlags::BOLD,
                        ..Default::default()
                    }
                ),
                (
                    'o',
                    Style {
                        flags: StyleFlags::BOLD,
                        ..Default::default()
                    }
                ),
            ]
        );
    }
}
