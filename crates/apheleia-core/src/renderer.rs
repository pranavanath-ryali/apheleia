use std::io::{Error, Stdout, Write, stdout};

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
use log::info;

use crate::{
    buffer::Buffer,
    style::{Style, StyleFlags},
    types::Vec2,
};

pub struct Renderer {
    pub size: Vec2,
    pub stdout: Stdout,
}
impl Renderer {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            stdout: stdout(),
        }
    }

    pub fn init(&mut self) {
        _ = execute!(self.stdout, cursor::Hide);
        _ = execute!(self.stdout, EnterAlternateScreen);
        _ = enable_raw_mode();
    }

    pub fn render_flip(&mut self, buf: &mut Buffer) {
        _ = execute!(self.stdout, Clear(crossterm::terminal::ClearType::All));
        _ = execute!(self.stdout, cursor::Hide);

        for y in 0..self.size.y {
            let mut batch_text = String::new();
            let mut style = Style::default();
            let mut start_x = 0u16;

            for x in 0..self.size.x {
                let cell = buf.get_cell(Vec2 { x, y });

                if cell.style == style {
                    batch_text.push(cell.c);
                    continue;
                }

                _ = queue_batch(&mut self.stdout, start_x, y, &batch_text, style);

                batch_text.clear();
                batch_text.push(cell.c);
                style = cell.style;
                start_x = x;
            }

            _ = queue_batch(&mut self.stdout, start_x, y, &batch_text, style);
        }

        _ = self.stdout.flush();
        buf.clear_diff();
    }

    pub fn render(&mut self, buf: &mut Buffer) {
        info!("RENDERER's UPDATE CALLED");
        for (y, map) in buf.get_diffed_cells().iter() {
            let mut batch_text = String::new();
            let mut style = Style::default();
            let mut start_x = 0u16;
            let mut offset = 0u16;

            for (x, cell) in map.iter() {
                if *x != start_x + offset + 1 {
                    _ = queue_batch(&mut self.stdout, start_x, *y, &batch_text, style);

                    start_x = *x;
                    offset = 0;
                    style = cell.style;
                    batch_text.clear();
                    batch_text.push(cell.c);

                    continue;
                }

                if cell.style != style {
                    _ = queue_batch(&mut self.stdout, start_x, *y, &batch_text, style);

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

            _ = queue_batch(&mut self.stdout, start_x, *y, &batch_text, style);
        }

        _ = self.stdout.flush();
        buf.clear_diff();
        info!("RENDERER's UPDATE ENDED");
    }

    pub fn quit(&mut self) {
        _ = disable_raw_mode();
        _ = execute!(self.stdout, cursor::Show);
        _ = execute!(self.stdout, LeaveAlternateScreen);
    }
}

fn queue_batch(
    stdout: &mut Stdout,
    x: u16,
    y: u16,
    text: &String,
    style: Style,
) -> Result<(), Error> {
    if text.is_empty() {
        return Ok(());
    }

    info!("BATCH QUEUE INFO:");
    info!("x: {}; y: {}", x, y);
    info!("text: {}", text);
    info!("style: {:?}", style);

    queue!(stdout, SetAttribute(Attribute::Reset))?;

    queue!(stdout, MoveTo(x, y))?;
    queue!(stdout, SetForegroundColor(style.fg))?;
    queue!(stdout, SetBackgroundColor(style.bg))?;
    queue_flags(stdout, style.flags)?;
    queue!(stdout, Print(text.to_string()))?;

    Ok(())
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
