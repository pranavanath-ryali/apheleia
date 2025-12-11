use std::io::{Stdout, Write, stdout};

use crossterm::{
    cursor, execute, queue,
    style::{
        Attribute, Color, PrintStyledContent, SetAttributes, SetBackgroundColor,
        SetForegroundColor, StyledContent, Stylize,
    },
    terminal::Clear,
};

use crate::{buffer::Buffer, style::Style};

pub struct Renderer {
    stdout: Stdout,
}

impl Renderer {
    pub fn new() -> Self {
        let mut stdout = stdout();
        execute!(stdout, cursor::Hide);

        Self { stdout }
    }

    pub fn flip(&mut self, buf: &mut Buffer) {
        execute!(self.stdout, Clear(crossterm::terminal::ClearType::All));

        for y in 0..(buf.height) {
            for x in 0..(buf.width) {
                let cell = buf.get(x, y);
                queue!(
                    self.stdout,
                    cursor::MoveTo(x as u16, y as u16),
                    PrintStyledContent(
                        cell.c
                            .with(cell.style.fg)
                            .on(cell.style.bg)
                            .attribute(cell.style.attrs.unwrap_or_else(|| { Attribute::Reset }))
                    )
                );
            }
        }

        self.stdout.flush();
        buf.clear_update_list();
    }
    
    // FIXME: working of attributes
    pub fn update(&mut self, buf: &mut Buffer) {
        for pos in buf.get_update_list() {
            let cell = buf.get(pos.0, pos.1);

            if let Some(attr) = cell.style.attrs {
                queue!(
                    self.stdout,
                    cursor::MoveTo(pos.0 as u16, pos.1 as u16),
                    PrintStyledContent(
                        cell.c
                            .with(cell.style.fg)
                            .on(cell.style.bg)
                            .attribute(attr)
                    )
                );
            } else {
                queue!(
                    self.stdout,
                    cursor::MoveTo(pos.0 as u16, pos.1 as u16),
                    PrintStyledContent(
                        cell.c
                            .with(cell.style.fg)
                            .on(cell.style.bg)
                    )
                );
            }
        }

        self.stdout.flush();
        buf.clear_update_list();
    }

    pub fn quit(&mut self) {
        execute!(self.stdout, cursor::Show);
    }
}
