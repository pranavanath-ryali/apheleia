use std::io::{Stdout, Write, stdout};

use crossterm::{
    cursor::{self, MoveTo, Show},
    execute, queue,
    style::{Attribute, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::Clear,
};

use crate::{
    buffer::{Buffer, Line},
    style::StyleFlags,
};

// pub struct Renderer {
//     pub stdout: Stdout,
// }

// impl Default for Renderer {
//     fn default() -> Self {
//         let mut stdout = stdout();
//         execute!(stdout, cursor::Hide);

//         Self { stdout }
//     }
// }

// impl Renderer {
//     fn queue_flags(&mut self, flags: &StyleFlags) {
//         if flags.contains(StyleFlags::Bold) {
//             queue!(self.stdout, SetAttribute(Attribute::Bold));
//         }
//         if flags.contains(StyleFlags::Italic) {
//             queue!(self.stdout, SetAttribute(Attribute::Italic));
//         }
//         if flags.contains(StyleFlags::Dim) {
//             queue!(self.stdout, SetAttribute(Attribute::Dim));
//         }
//         if flags.contains(StyleFlags::Reverse) {
//             queue!(self.stdout, SetAttribute(Attribute::Reverse));
//         }
//         if flags.contains(StyleFlags::UnderCurled) {
//             queue!(self.stdout, SetAttribute(Attribute::Undercurled));
//         }
//         if flags.contains(StyleFlags::UnderLined) {
//             queue!(self.stdout, SetAttribute(Attribute::Underlined));
//         }
//         if flags.contains(StyleFlags::UnderDotted) {
//             queue!(self.stdout, SetAttribute(Attribute::Underdotted));
//         }
//         if flags.contains(StyleFlags::UnderDashed) {
//             queue!(self.stdout, SetAttribute(Attribute::Underdashed));
//         }
//         if flags.contains(StyleFlags::DoubleUnderLined) {
//             queue!(self.stdout, SetAttribute(Attribute::DoubleUnderlined));
//         }
//         if flags.contains(StyleFlags::SlowBlink) {
//             queue!(self.stdout, SetAttribute(Attribute::SlowBlink));
//         }

//         if flags.is_empty() {
//             queue!(self.stdout, SetAttribute(Attribute::Reset));
//         }
//     }

//     fn queue_write(&mut self, line: &Line) {
//         queue!(
//             self.stdout,
//             cursor::MoveTo(line.position.0, line.position.1)
//         );
//         self.queue_flags(&line.style.flags);
//         queue!(
//             self.stdout,
//             SetForegroundColor(line.style.fg),
//             SetBackgroundColor(line.style.bg),
//             Print(&line.text)
//         );
//     }

//     pub fn clear(&mut self, buf: &mut Buffer) {
//         execute!(self.stdout, Clear(crossterm::terminal::ClearType::All));
//         self.stdout.flush();

//         buf.clear_update_list();
//     }

//     pub fn update(&mut self, buf: &mut Buffer) {
//         for line in buf.get_update_list() {
//             self.queue_write(line);
//         }
//         self.stdout.flush();
//         buf.clear_update_list();
//     }

//     pub fn quit(&mut self) {
//         execute!(self.stdout, cursor::Show);
//     }
// }

pub struct Renderer {
    pub width: u16,
    pub height: u16,
    pub stdout: Stdout,
}

impl Renderer {
    pub fn clear(&mut self, buf: &mut Buffer) {
        _ = execute!(self.stdout, Clear(crossterm::terminal::ClearType::All));
        _ = self.stdout.flush();

        buf.clear_update_list();
    }

    fn handle_tag(&mut self, tag: &str) {}

    pub fn render(&mut self, buf: &mut Buffer) {
        for line in buf.get_update_list() {
            let mut s: String = "".to_string();
            let mut is_inside_tag = false;

            let mut first = true;
            let mut current_pos = line.position;
            let mut offset: (u16, u16) = (0, 0);

            _ = queue!(
                self.stdout,
                SetBackgroundColor(crossterm::style::Color::Blue)
            );

            for c in line.text.chars() {
                match c {
                    '<' => {
                        if s.len() != 0 {
                            _ = queue!(self.stdout, MoveTo(current_pos.0, current_pos.1));
                            _ = queue!(self.stdout, Print(&s));
                            s.clear();

                            current_pos.0 += offset.0;
                            current_pos.1 += offset.1;
                            offset.0 = 0;
                            offset.1 = 0;
                        }
                        is_inside_tag = true;
                    }
                    '>' => {
                        if is_inside_tag {
                            is_inside_tag = false;
                            self.handle_tag(&s.to_string());
                            s.clear();
                        }
                    }
                    _ => {
                        if !is_inside_tag {
                            s.push(c);
                            offset.0 += 1;
                        }
                    }
                }
            }
            if s.len() != 0 {
                _ = queue!(self.stdout, MoveTo(current_pos.0, current_pos.1));
                _ = queue!(self.stdout, Print(&s));
                s.clear();
            }
        }
        _ = self.stdout.flush();
        buf.clear_update_list();
    }

    pub fn quit(&mut self) {
        _ = execute!(self.stdout, Show);
    }
}
