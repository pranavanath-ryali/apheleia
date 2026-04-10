use std::io::{Error, Stdout, Write, stdout};

use crossterm::{
    cursor::{MoveTo, MoveToRow},
    execute, queue,
    style::{
        self, Attribute, Attributes, Print, SetAttribute, SetAttributes, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{Clear, disable_raw_mode, enable_raw_mode},
};

use crate::{
    buffer::Buffer,
    style::{Style, StyleFlags},
};

pub struct Renderer {
    pub size: (u16, u16),
    pub stdout: Stdout,
}
impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            size: (width, height),
            stdout: stdout(),
        }
    }

    pub fn init(&mut self) {
        _ = enable_raw_mode();
        _ = execute!(self.stdout, Clear(crossterm::terminal::ClearType::All));
    }

    pub fn render_flip(&mut self, buf: &mut Buffer) {
        _ = execute!(self.stdout, Clear(crossterm::terminal::ClearType::All));

        for y in 0..self.size.1 {
            let mut batch_text = String::new();
            let mut style = Style::default();
            let mut start_x = 0u16;

            _ = queue!(self.stdout, SetAttribute(Attribute::Reset));

            for x in 0..self.size.0 {
                let cell = buf.get_cell(x, y);

                if cell.style == style {
                    batch_text.push(cell.c);
                    continue;
                }

                _ = queue!(self.stdout, SetAttribute(Attribute::Reset));

                _ = queue!(self.stdout, MoveTo(start_x, y));
                _ = queue!(self.stdout, SetForegroundColor(style.fg));
                _ = queue!(self.stdout, SetBackgroundColor(style.bg));
                _ = queue_flags(&mut self.stdout, style.flags);
                _ = queue!(self.stdout, Print(batch_text.to_string()));

                batch_text.clear();
                batch_text.push(cell.c);
                style = cell.style;
                start_x = x;
            }

            _ = queue!(self.stdout, SetAttribute(Attribute::Reset));

            _ = queue!(self.stdout, MoveTo(start_x, y));
            _ = queue!(self.stdout, SetForegroundColor(style.fg));
            _ = queue!(self.stdout, SetBackgroundColor(style.bg));
            _ = queue_flags(&mut self.stdout, style.flags);
            _ = queue!(self.stdout, Print(batch_text.to_string()));
        }

        _ = self.stdout.flush();
        buf.clear_diff();
    }

    pub fn render(&mut self, buf: &mut Buffer) {
        for (y, map) in buf.get_diffed_cells().iter() {
            let mut batch_text = String::new();
            let mut style = Style::default();
            let mut start_x = 0u16;
            let mut offset = 0u16;

            _ = queue!(self.stdout, SetAttribute(Attribute::Reset));

            for (x, cell) in map.iter() {
                if *x != start_x + offset + 1 {
                    _ = queue!(self.stdout, SetAttribute(Attribute::Reset));
                    _ = queue!(self.stdout, MoveTo(start_x, *y));
                    _ = queue!(self.stdout, SetForegroundColor(style.fg));
                    _ = queue!(self.stdout, SetBackgroundColor(style.bg));
                    _ = queue_flags(&mut self.stdout, style.flags);
                    _ = queue!(self.stdout, Print(batch_text.to_string()));

                    start_x = *x;
                    offset = 0;
                    style = cell.style;
                    batch_text.clear();
                    batch_text.push(cell.c);

                    continue;
                }

                if cell.style != style {
                    _ = queue!(self.stdout, SetAttribute(Attribute::Reset));
                    _ = queue!(self.stdout, MoveTo(start_x, *y));
                    _ = queue!(self.stdout, SetForegroundColor(style.fg));
                    _ = queue!(self.stdout, SetBackgroundColor(style.bg));
                    _ = queue_flags(&mut self.stdout, style.flags);
                    _ = queue!(self.stdout, Print(batch_text.to_string()));

                    start_x = *x;
                    offset = 0;
                    style = cell.style;
                    batch_text.clear();
                    batch_text.push(cell.c);
                }

                offset += 1;
                batch_text.push(cell.c);
            }

            _ = queue!(self.stdout, SetAttribute(Attribute::Reset));
            _ = queue!(self.stdout, MoveTo(start_x, *y));
            _ = queue!(self.stdout, SetForegroundColor(style.fg));
            _ = queue!(self.stdout, SetBackgroundColor(style.bg));
            _ = queue_flags(&mut self.stdout, style.flags);
            _ = queue!(self.stdout, Print(batch_text.to_string()));
        }

        _ = self.stdout.flush();
    }

    pub fn quit(&mut self) {
        _ = disable_raw_mode();
    }
}

fn queue_flags(stdout: &mut Stdout, flags: StyleFlags) -> Result<(), Error> {
    let mut attr = Attributes::default();

    if flags.contains(StyleFlags::BOLD) {
        attr.set(Attribute::Bold);
    }
    if flags.contains(StyleFlags::ITALIC) {
        attr.set(Attribute::Italic);
    }
    if flags.contains(StyleFlags::DIM) {
        attr.set(Attribute::Dim);
    }
    if flags.contains(StyleFlags::REVERSE) {
        attr.set(Attribute::Reverse);
    }
    if flags.contains(StyleFlags::UNDER_CURLED) {
        attr.set(Attribute::Undercurled);
    }
    if flags.contains(StyleFlags::UNDER_LINED) {
        attr.set(Attribute::Underlined);
    }
    if flags.contains(StyleFlags::UNDER_DOTTED) {
        attr.set(Attribute::Underdotted);
    }
    if flags.contains(StyleFlags::UNDER_DASHED) {
        attr.set(Attribute::Underdashed);
    }
    if flags.contains(StyleFlags::DOUBLE_UNDERLINED) {
        attr.set(Attribute::DoubleUnderlined);
    }
    if flags.contains(StyleFlags::SLOW_BLINK) {
        attr.set(Attribute::SlowBlink);
    }

    queue!(stdout, SetAttributes(attr))
}

// pub struct Renderer {
//     pub width: u16,
//     pub height: u16,
//     pub stdout: Stdout,
// }

// impl Renderer {
//     pub fn clear(&mut self, buf: &mut Buffer) {
//         _ = execute!(self.stdout, Clear(crossterm::terminal::ClearType::All));
//         _ = self.stdout.flush();

//         buf.clear_update_list();
//     }

//     fn handle_markup(&mut self, tag: &str) {
//         // Syntax: <fg: red; bg: blue; underline; bold; italic>
//         // Some properties have values
//         // Others just need to exist

//         let mut pairs: Vec<(String, String)> = vec![];
//         for mut c in (&tag[1..tag.len() - 1]).split(';') {
//             c = c.trim();
//             let mut words = c.split(':').fuse();
//             pairs.push((
//                 words
//                     .next()
//                     .unwrap()
//                     .trim()
//                     .to_ascii_lowercase()
//                     .to_string(),
//                 words
//                     .next()
//                     .unwrap_or("")
//                     .trim()
//                     .to_ascii_lowercase()
//                     .to_string(),
//             ));
//         }

//         fn get_color(value: &str) -> Color {
//             let color: Color;
//             match value {
//                 "red" => color = Color::Red,
//                 "darkred" => color = Color::DarkRed,

//                 "blue" => color = Color::Blue,
//                 "darkblue" => color = Color::DarkBlue,

//                 "magenta" => color = Color::Magenta,
//                 "darkmagenta" => color = Color::DarkMagenta,

//                 "yellow" => color = Color::Yellow,
//                 "darkyellow" => color = Color::DarkYellow,

//                 "cyan" => color = Color::Cyan,
//                 "darkcyan" => color = Color::DarkCyan,

//                 "green" => color = Color::Green,
//                 "darkgreen" => color = Color::DarkGreen,

//                 "grey" => color = Color::Grey,
//                 "darkgrey" => color = Color::DarkGrey,

//                 "white" => color = Color::White,
//                 "black" => color = Color::Black,

//                 _ => color = Color::Reset,
//             }
//             color
//         }

//         for (prop, value) in pairs {
//             match prop.as_str() {
//                 "fg" => {
//                     _ = queue!(self.stdout, SetForegroundColor(get_color(value.as_str())));
//                 }
//                 "bg" => {
//                     _ = queue!(self.stdout, SetBackgroundColor(get_color(value.as_str())));
//                 }
//                 "r" => _ = queue!(self.stdout, SetAttribute(Attribute::Reset)),

//                 "normal" => _ = queue!(self.stdout, SetAttribute(Attribute::NormalIntensity)),
//                 "italic" | "i" => _ = queue!(self.stdout, SetAttribute(Attribute::Italic)),
//                 "bold" | "b" => _ = queue!(self.stdout, SetAttribute(Attribute::Bold)),

//                 "dim" => _ = queue!(self.stdout, SetAttribute(Attribute::Dim)),

//                 "reverse" => _ = queue!(self.stdout, SetAttribute(Attribute::Reverse)),

//                 "underlined" => _ = queue!(self.stdout, SetAttribute(Attribute::Underlined)),
//                 "undercurled" => _ = queue!(self.stdout, SetAttribute(Attribute::Undercurled)),
//                 "underdashed" => _ = queue!(self.stdout, SetAttribute(Attribute::Underdashed)),
//                 "underdotted" => _ = queue!(self.stdout, SetAttribute(Attribute::Underdotted)),
//                 "doubleunderlined" => {
//                     _ = queue!(self.stdout, SetAttribute(Attribute::DoubleUnderlined))
//                 }

//                 "blink" => _ = queue!(self.stdout, SetAttribute(Attribute::SlowBlink)),

//                 _ => {
//                     let color = get_color(prop.as_str());
//                     if color != Color::Reset {
//                         _ = queue!(self.stdout, SetForegroundColor(color));
//                     }
//                 }
//             }
//         }
//     }

//     pub fn render(&mut self, buf: &mut Buffer) {
//         for line in buf.get_update_list() {
//             let mut s: String = "".to_string();
//             let mut is_inside_tag = false;
//             let mut current_pos = line.position;
//             let mut offset: (u16, u16) = (0, 0);

//             for c in line.text.chars() {
//                 match c {
//                     '<' => {
//                         _ = queue!(self.stdout, MoveTo(current_pos.0, current_pos.1));
//                         _ = queue!(self.stdout, Print(&s));
//                         s.clear();

//                         s.push(c);

//                         current_pos.0 += offset.0;
//                         current_pos.1 += offset.1;
//                         offset.0 = 0;
//                         offset.1 = 0;
//                         is_inside_tag = true;
//                     }
//                     '>' => {
//                         if is_inside_tag {
//                             s.push(c);
//                             is_inside_tag = false;
//                             self.handle_markup(&s.to_string());
//                             s.clear();
//                         }
//                     }
//                     _ => {
//                         s.push(c);
//                         if !is_inside_tag {
//                             offset.0 += 1;
//                         }
//                     }
//                 }
//             }
//             if s.len() != 0 {
//                 _ = queue!(self.stdout, MoveTo(current_pos.0, current_pos.1));
//                 _ = queue!(self.stdout, Print(&s));
//                 _ = queue!(self.stdout, SetAttribute(Attribute::Reset));
//                 s.clear();
//             }
//         }
//         _ = self.stdout.flush();
//         buf.clear_update_list();
//     }

//     pub fn quit(&mut self) {
//         _ = execute!(self.stdout, Show);
//     }
// }
