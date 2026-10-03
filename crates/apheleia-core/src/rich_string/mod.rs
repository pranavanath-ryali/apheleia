// use crate::style::Style;
//
// #[derive(Debug)]
// struct Span {
//     pub start: usize,
//     pub end: Option<usize>,
//     pub style: Style,
// }
//
// #[derive(Debug)]
// pub struct RichString {
//     pub text: String,
//     spans: Vec<Span>,
// }
// impl RichString {
//     pub fn new(text: &str) -> Self {
//         // </.../> overlay style
//         // <...> scoped style
//         // {...} forced style
//
//         let mut text: &str = text;
//         let mut raw_text: String = String::new();
//         let mut spans: Vec<Span> = Default::default();
//         let mut run = false;
//         while let Some(start_index) = text.find("</") {
//             run = true;
//             let end_index = text
//                 .find("/>")
//                 .filter(|&v| v > start_index)
//                 .expect("Expected closing delimiter for scoped style markup. Couldn't find '/>'");
//             let style = Style::from_markup(&text[(start_index + 2)..end_index]);
//             spans.push(Span {
//                 start: start_index,
//                 end: None,
//                 style,
//             });
//
//             text = &text[(end_index + 2)..];
//             if let Some(next_start) = text.find("</") {
//                 raw_text.push_str(&text[0..next_start]);
//                 continue;
//             }
//
//             raw_text.push_str(text);
//             break;
//         }
//
//         if !run {
//             raw_text = String::from(text);
//         }
//
//         Self {
//             text: raw_text,
//             spans,
//         }
//     }
//
//     pub fn len(&self) -> usize {
//         self.text.len()
//     }
//     pub fn is_empty(&self) -> bool {
//         self.text.is_empty()
//     }
//
//     pub fn iter<'a>(&'a self) -> RichStringIter<'a> {
//         RichStringIter::new(&self.text, &self.spans)
//     }
// }
//
// pub struct RichStringIter<'a> {
//     text: &'a str,
//     spans: &'a [Span],
//
//     cursor: usize,
// }
// impl<'a> RichStringIter<'a> {
//     pub(crate) fn new(text: &'a str, spans: &'a [Span]) -> Self {
//         Self {
//             text,
//             spans,
//
//             cursor: 0,
//         }
//     }
// }
// impl<'a> Iterator for RichStringIter<'a> {
//     type Item = (char, Style);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some((_, ch)) = self.text[self.cursor..].char_indices().next() {
//             let mut style = Style::default();
//             for span in self.spans {
//                 if self.cursor >= span.start {
//                     if let Some(end) = span.end
//                         && end < self.cursor
//                     {
//                         style.update(&span.style);
//                     } else {
//                         style.update(&span.style);
//                     }
//                 }
//             }
//             self.cursor += ch.len_utf8();
//             return Some((ch, style));
//         }
//         None
//     }
// }
