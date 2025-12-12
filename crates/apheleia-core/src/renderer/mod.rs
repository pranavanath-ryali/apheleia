use std::io::{Stdout, Write, stdout};

use crossterm::{
    cursor, execute, queue,
    style::{
        Attribute, Color, Print, PrintStyledContent, SetAttribute, SetAttributes, SetBackgroundColor, SetForegroundColor, StyledContent, Stylize
    },
    terminal::Clear,
};

use crate::{
    buffer::Buffer,
    style::{Style, StyleFlags},
};

pub struct Renderer {
    stdout: Stdout,
}

impl Renderer {
    pub fn new() -> Self {
        let mut stdout = stdout();
        execute!(stdout, cursor::Hide);

        Self { stdout }
    }

    fn queue_flags(&mut self, flags: &StyleFlags) {
        if flags.contains(StyleFlags::BOLD) {
            queue!(self.stdout, SetAttribute(Attribute::Bold));
        }
        if flags.contains(StyleFlags::ITALIC) {
            queue!(self.stdout, SetAttribute(Attribute::Italic));
        }
        if flags.contains(StyleFlags::DIM) {
            queue!(self.stdout, SetAttribute(Attribute::Dim));
        }
        if flags.contains(StyleFlags::REVERSE) {
            queue!(self.stdout, SetAttribute(Attribute::Reverse));
        }
        if flags.contains(StyleFlags::UNDERCURLED) {
            queue!(self.stdout, SetAttribute(Attribute::Undercurled));
        }
        if flags.contains(StyleFlags::UNDERLINED) {
            queue!(self.stdout, SetAttribute(Attribute::Underlined));
        }
        if flags.contains(StyleFlags::UNDERDOTTED) {
            queue!(self.stdout, SetAttribute(Attribute::Underdotted));
        }
        if flags.contains(StyleFlags::UNDERDASHED) {
            queue!(self.stdout, SetAttribute(Attribute::Underdashed));
        }
        if flags.contains(StyleFlags::DOUBLE_UNDERLINED) {
            queue!(self.stdout, SetAttribute(Attribute::DoubleUnderlined));
        }
        if flags.contains(StyleFlags::SLOW_BLINK) {
            queue!(self.stdout, SetAttribute(Attribute::SlowBlink));
        }

        if flags.is_empty() {
            queue!(self.stdout, SetAttribute(Attribute::Reset));
        }
    }

    pub fn flip(&mut self, buf: &mut Buffer) {
        execute!(self.stdout, Clear(crossterm::terminal::ClearType::All));

        for y in 0..(buf.height) {
            for x in 0..(buf.width) {
                let cell = buf.get(x, y);

                queue!(self.stdout, cursor::MoveTo(x as u16, y as u16));
                self.queue_flags(&cell.style.flags);
                queue!(
                    self.stdout,
                    SetForegroundColor(cell.style.fg),
                    SetBackgroundColor(cell.style.bg),
                    Print(cell.c)
                );
            }
        }

        self.stdout.flush();
        buf.clear_update_list();
    }

    // FIXME: working of attributes
    pub fn update(&mut self, buf: &mut Buffer) {
        // for pos in buf.get_update_list() {
        //     let cell = buf.get(pos.0, pos.1);
        //
        //     if let Some(attr) = cell.style.attrs {
        //         queue!(
        //             self.stdout,
        //             cursor::MoveTo(pos.0 as u16, pos.1 as u16),
        //             PrintStyledContent(
        //                 cell.c
        //                     .with(cell.style.fg)
        //                     .on(cell.style.bg)
        //                     .attribute(attr)
        //             )
        //         );
        //     } else {
        //         queue!(
        //             self.stdout,
        //             cursor::MoveTo(pos.0 as u16, pos.1 as u16),
        //             PrintStyledContent(
        //                 cell.c
        //                     .with(cell.style.fg)
        //                     .on(cell.style.bg)
        //             )
        //         );
        //     }
        // }
        //
        // self.stdout.flush();
        // buf.clear_update_list();
    }

    pub fn quit(&mut self) {
        execute!(self.stdout, cursor::Show);
    }
}
