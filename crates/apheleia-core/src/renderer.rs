use std::io::{self, Error, Stdout, Write, stdout};
use crossterm::{
    cursor::{self, MoveTo},
    execute, queue,
    style::{
        Attribute, Attributes, Print, SetAttribute, SetAttributes, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{
        Clear, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use log::warn;

use crate::{
    buffer::Buffer,
    style::{Style, StyleFlags}, types::Vec2,
};

pub struct Renderer {
    pub stdout: Stdout,
}
impl Default for Renderer {
    fn default() -> Self {
        Self { stdout: stdout() }
    }
}
impl Renderer {
    pub fn init(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::Hide)?;
        execute!(self.stdout, EnterAlternateScreen)?;
        enable_raw_mode()?;

        Ok(())
    }

    pub fn render_flip(&mut self, buf: &mut Buffer) -> io::Result<()> {
        execute!(self.stdout, Clear(crossterm::terminal::ClearType::All))?;
        execute!(self.stdout, cursor::Hide)?;

        for y in 0..buf.size.y {
            let mut batch_text = String::new();
            let mut style = Style::default();
            let mut start_x = 0u32;

            for x in 0..buf.size.x {
                let cell = buf.get_cell(Vec2 { x, y });

                if cell.style == style {
                    batch_text.push(cell.c);
                    continue;
                }

                queue_batch(&mut self.stdout, start_x, y, &batch_text, style)?;

                batch_text.clear();
                batch_text.push(cell.c);
                style = cell.style;
                start_x = x;
            }

            queue_batch(&mut self.stdout, start_x, y, &batch_text, style)?;
        }

        self.stdout.flush()?;
        buf.clear_diff();

        Ok(())
    }

    pub fn render(&mut self, buf: &mut Buffer) -> io::Result<()> {
        for (y, map) in buf.get_diffed_cells().iter() {
            let mut batch_text = String::new();
            let mut style = Style::default();
            let mut start_x = 0u32;
            let mut offset = 0u32;

            for (x, cell) in map.iter() {
                if *x != start_x + offset + 1 {
                    queue_batch(&mut self.stdout, start_x, *y, &batch_text, style)?;

                    start_x = *x;
                    offset = 0;
                    style = cell.style;
                    batch_text.clear();
                    batch_text.push(cell.c);

                    continue;
                }

                if cell.style != style {
                    queue_batch(&mut self.stdout, start_x, *y, &batch_text, style)?;

                    start_x = *x;
                    offset = 0;
                    style = cell.style;
                    batch_text.clear();
                    batch_text.push(cell.c);

                    continue;
                }

                offset += 1;
                batch_text.push(cell.c);
            }

            queue_batch(&mut self.stdout, start_x, *y, &batch_text, style)?;
        }

        self.stdout.flush()?;
        buf.clear_diff();

        Ok(())
    }

    pub fn quit(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(self.stdout, cursor::Show)?;
        execute!(self.stdout, LeaveAlternateScreen)?;

        Ok(())
    }
}

fn queue_batch(
    stdout: &mut Stdout,
    x: u32,
    y: u32,
    text: &String,
    style: Style,
) -> Result<(), Error> {
    if text.is_empty() {
        return Ok(());
    }

    warn!("[CORE] Batched X: {}; Y: {}; TEXT: '{}'", x, y, text);

    queue!(stdout, SetAttribute(Attribute::Reset))?;

    queue!(stdout, MoveTo(x as u16, y as u16))?;
    queue!(stdout, SetForegroundColor(style.fg))?;
    queue!(stdout, SetBackgroundColor(style.bg))?;
    queue_flags(stdout, style.flags)?;
    queue!(stdout, Print(text.to_string()))?;

    Ok(())
}

fn queue_flags(stdout: &mut Stdout, flags: StyleFlags) -> io::Result<()> {
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

    queue!(stdout, SetAttributes(attr))?;
    Ok(())
}
