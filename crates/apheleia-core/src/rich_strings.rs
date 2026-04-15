use std::{ops::Add, slice::Iter, vec::IntoIter};

use crate::style::Style;

#[derive(Debug, PartialEq)]
struct RichText {
    pub text: String,
    pub style: Style,
}

#[derive(Debug, Default, PartialEq)]
pub struct RichString {
    rich_texts: Vec<RichText>,
}
impl RichString {
    pub fn new(text: &str) -> Self {
        Self {
            rich_texts: parse_string(text),
        }
    }
    pub fn to_rich(text: &str, style: Style) -> Self {
        Self {
            rich_texts: parse_string(text),
        }
    }

    pub fn len(&self) -> usize {
        let mut len = 0_usize;
        for text in self.rich_texts.iter() {
            len += text.text.len();
        }
        len
    }

    pub fn is_empty(&self) -> bool {
        if self.len() == 0 {
            return true;
        }

        false
    }

    pub fn add_text(&mut self, text: &str, style: Style) {
        self.rich_texts.push(RichText {
            text: text.to_string(),
            style,
        });
    }

    pub fn append(&mut self, rich_str: &mut RichString) {
        self.rich_texts.append(&mut rich_str.rich_texts);
    }

    pub fn iter(&self) -> RichStringIter<'_> {
        let chars: Vec<char> = self.rich_texts[0].text.chars().collect();
        let mut texts_iter = self.rich_texts.iter();
        texts_iter.next();
        RichStringIter {
            current_iter: RichTextIter {
                chars: chars.into_iter(),
                style: self.rich_texts[0].style,
            },
            rich_texts: texts_iter,
        }
    }

    pub fn slice(&self, start: usize, end: usize) -> Self {
        let mut offset = 0_usize;
        let mut texts: Vec<RichText> = vec![];
        let mut started = false;
        for rich_text in self.rich_texts.iter() {
            if start >= offset && start < rich_text.text.len() + offset {
                if end >= offset && end < rich_text.text.len() + offset {
                    texts.push({
                        RichText {
                            text: rich_text
                                .text
                                .split_at(start - offset)
                                .1
                                .split_at(end - offset - start)
                                .0
                                .to_string(),
                            style: rich_text.style,
                        }
                    });

                    break;
                }

                started = true;
                texts.push(RichText {
                    text: rich_text.text.split_at(start - offset).1.to_string(),
                    style: rich_text.style,
                });
                offset += rich_text.text.len();
                continue;
            }

            if end >= offset && end < rich_text.text.len() + offset {
                let text = rich_text.text.split_at(end - offset).0.to_string();
                if !text.is_empty() {
                    texts.push(RichText {
                        text,
                        style: rich_text.style,
                    });
                }
                break;
            }

            if started {
                texts.push(RichText {
                    text: rich_text.text.to_string(),
                    style: rich_text.style,
                });
            }
            offset += rich_text.text.len();
        }

        RichString { rich_texts: texts }
    }
}

struct RichTextIter {
    chars: IntoIter<char>,
    style: Style,
}
impl Iterator for RichTextIter {
    type Item = (char, Style);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.chars.next() {
            return Some((c, self.style));
        }
        None
    }
}

pub struct RichStringIter<'a> {
    rich_texts: Iter<'a, RichText>,
    current_iter: RichTextIter,
}
impl<'a> Iterator for RichStringIter<'a> {
    type Item = (char, Style);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.current_iter.next() {
            return Some(item);
        }

        if let Some(text) = self.rich_texts.next() {
            let chars: Vec<char> = text.text.chars().collect();
            self.current_iter = RichTextIter {
                chars: chars.into_iter(),
                style: text.style,
            };

            return self.current_iter.next();
        }

        None
    }
}

fn parse_string(text: &str) -> Vec<RichText> {
    let mut rich_texts: Vec<RichText> = vec![];
    let mut current_style = Style::default();
    for token in text.split("</") {
        let t: Vec<&str> = token.split("/>").collect();
        if t.len() > 1 {
            current_style.update(Style::from_markup(t[0]));
            let text = t[1].to_string();

            rich_texts.push(RichText {
                text,
                style: current_style,
            });

            continue;
        }

        if !t[0].is_empty() {
            rich_texts.push(RichText {
                text: t[0].to_string(),
                style: current_style,
            });
        }
    }

    rich_texts
}

#[cfg(test)]
mod rich_string_test {
    use crossterm::style::Color;

    use crate::style::StyleFlags;

    use super::*;

    #[test]
    fn test_parse_text() {
        let rich_str = parse_string("</fg:blue;bold;under_lined/>HELLO");
        assert_eq!(
            rich_str,
            vec![RichText {
                text: "HELLO".to_string(),
                style: Style {
                    fg: Color::Blue,
                    flags: StyleFlags::UNDER_LINED | StyleFlags::BOLD,
                    ..Default::default()
                }
            }]
        );
    }

    #[test]
    fn test_richstring_iter() {
        let rich_str = RichString::new("</bold;fg: red; bg: white/>H</bg:blue;italic/>A");
        let vec: Vec<(char, Style)> = rich_str.iter().collect();

        assert_eq!(
            vec,
            vec![
                (
                    'H',
                    Style {
                        fg: Color::Red,
                        bg: Color::White,
                        flags: StyleFlags::BOLD,
                    }
                ),
                (
                    'A',
                    Style {
                        fg: Color::Red,
                        bg: Color::Blue,
                        flags: StyleFlags::BOLD | StyleFlags::ITALIC,
                    }
                ),
            ]
        );
    }

    #[test]
    fn test_richstring_slice() {
        let rich_str = RichString::new("</fg:red/>HELLO</bg:blue/>WORLD");
        assert_eq!(
            rich_str.slice(2, 7),
            RichString {
                rich_texts: vec![
                    RichText {
                        text: "LLO".to_string(),
                        style: Style {
                            fg: Color::Red,
                            ..Default::default()
                        }
                    },
                    RichText {
                        text: "WO".to_string(),
                        style: Style {
                            fg: Color::Red,
                            bg: Color::Blue,
                            ..Default::default()
                        }
                    }
                ]
            }
        );
        assert_eq!(
            rich_str.slice(2, 5),
            RichString {
                rich_texts: vec![RichText {
                    text: "LLO".to_string(),
                    style: Style {
                        fg: Color::Red,
                        ..Default::default()
                    }
                },]
            }
        );
        assert_eq!(
            rich_str.slice(2, 3),
            RichString {
                rich_texts: vec![RichText {
                    text: "L".to_string(),
                    style: Style {
                        fg: Color::Red,
                        ..Default::default()
                    }
                },]
            }
        );
    }
}
