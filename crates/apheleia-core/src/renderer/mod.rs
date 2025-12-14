use std::io::{Stdout, Write, stdout};

use crossterm::{
    cursor, execute, queue,
    style::{Attribute, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::Clear,
};

use crate::{
    buffer::{Buffer, Cell, Line},
    style::StyleFlags,
};

pub struct Renderer {
    pub stdout: Stdout,
}

impl Default for Renderer {
    fn default() -> Self {
        let mut stdout = stdout();
        execute!(stdout, cursor::Hide);

        Self { stdout }
    }
}

impl Renderer {
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

    fn queue_write_cell(&mut self, x: u16, y: u16, cell: &Cell) {
        queue!(self.stdout, cursor::MoveTo(x, y));
        self.queue_flags(&cell.style.flags);
        queue!(
            self.stdout,
            SetForegroundColor(cell.style.fg),
            SetBackgroundColor(cell.style.bg),
            Print(cell.ch)
        );
    }

    fn queue_write(&mut self, line: &Line) {
        queue!(
            self.stdout,
            cursor::MoveTo(line.position.0, line.position.1)
        );
        self.queue_flags(&line.style.flags);
        queue!(
            self.stdout,
            SetForegroundColor(line.style.fg),
            SetBackgroundColor(line.style.bg),
            Print(&line.text)
        );
    }

    pub fn flip(&mut self, buf: &mut Buffer) {
        execute!(self.stdout, Clear(crossterm::terminal::ClearType::All));

        for y in 0..(buf.height) {
            for x in 0..(buf.width) {
                self.queue_write_cell(x as u16, y as u16, &buf.get(x, y));
            }
        }

        self.stdout.flush();
        buf.clear_update_list();
    }

    pub fn update(&mut self, buf: &mut Buffer) {
        for line in buf.get_update_list() {
            self.queue_write(line);
        }
        self.stdout.flush();
        buf.clear_update_list();
    }

    pub fn quit(&mut self) {
        execute!(self.stdout, cursor::Show);
    }
}
