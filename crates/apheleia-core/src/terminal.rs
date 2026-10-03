use std::{
    env,
    io::{self, Stdout, Write, stdout},
    mem::take,
};

use crossterm::{
    cursor::{self, MoveTo},
    execute, queue,
    style::{
        self, Color, Print, SetAttribute, SetAttributes, SetBackgroundColor, SetForegroundColor,
    },
    terminal::Clear,
};
use tracing::info;

use crate::{
    buffer::Buffer,
    cell::layered::MultiLayerCellTrait,
    style::{Style, modifiers::Modifiers},
};

static mut QUEUE_COUNT: u32 = 0;

#[derive(Debug)]
pub enum ColorSpace {
    Monochrome,
    Ansi,      // 4 bit
    HighColor, // 8 bit
    TrueColor, // 24 bit
}

#[derive(Debug)]
pub struct TerminalCapabilities {
    pub color_space: ColorSpace,
}

pub struct Terminal {
    stdout: Stdout, // TODO: Temporary, eventually abstract this into the backend

    pub capabilities: TerminalCapabilities,
}
impl Terminal {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            stdout: stdout(),
            capabilities: get_capabilites(),
        }
    }

    pub fn init(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::Hide)?;
        // TODO: Use alternate screen

        Ok(())
    }

    pub fn render_update(&mut self, buffer: &mut Buffer) -> io::Result<()> {
        info!(target: "CORE", "RENDER UPDATE - Started");

        buffer.changed_cells.sort_unstable_by_key(|v| (v.1, v.0));
        buffer.changed_cells.dedup();

        let mut batch_text = String::new();
        let mut current_style = Style::default();
        let mut current_pos = (0u16, 0u16);
        let mut offset_x = 0u16;

        unsafe {
            QUEUE_COUNT = 0;
        }
        let changed_cells = take(&mut buffer.changed_cells);
        for (x, y) in changed_cells {
            if current_pos.1 != y || current_pos.0 + offset_x != x {
                queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

                batch_text.clear();
                current_pos = (x, y);
                current_style = Style::default();
                offset_x = 0;
            }

            let (cell, z_cells) = buffer.get_cell_mut((current_pos.0 + offset_x, current_pos.1));
            let result_cell = z_cells.result();

            if let Some(result_cell) = result_cell {
                if result_cell.transparent {
                    queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

                    batch_text.clear();
                    current_pos = (x, y);
                    offset_x = 0;
                    current_style = Style::default();

                    batch_text.push(' ');
                    offset_x += 1;

                    continue;
                }

                if result_cell == *cell {
                    queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

                    batch_text.clear();
                    current_pos = (x, y);
                    current_style = result_cell.style;
                    offset_x = 0;

                    batch_text.push(result_cell.c);
                    continue;
                }

                if result_cell.style != current_style {
                    queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

                    batch_text.clear();
                    current_pos = (x, y);
                    current_style = result_cell.style;
                    offset_x = 1;

                    batch_text.push(result_cell.c);

                    *cell = result_cell;
                    continue;
                }

                batch_text.push(result_cell.c);
                *cell = result_cell;
            } else {
                queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

                batch_text.clear();
                current_pos = (x, y);
                offset_x = 0;
                current_style = Style::default();

                batch_text.push(' ');
                offset_x += 1;

                continue;
            }
            offset_x += 1;
        }

        queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

        queue!(&mut self.stdout, SetForegroundColor(Color::Reset))?;
        queue!(&mut self.stdout, SetBackgroundColor(Color::Reset))?;

        self.stdout.flush()?;

        info!(target: "CORE", "RENDER UPDATE - Ended. Flushed {} render queue calls", unsafe {QUEUE_COUNT});
        Ok(())
    }

    pub fn render_clear(&mut self, buffer: &mut Buffer) -> io::Result<()> {
        info!(target: "CORE", "RENDER CLEAR - Started");

        execute!(self.stdout, Clear(crossterm::terminal::ClearType::All))?;

        let mut batch_text = String::new();
        let mut current_style = Style::default();
        let mut current_pos = (0u16, 0u16);

        unsafe {
            QUEUE_COUNT = 0;
        }
        for y in 0..buffer.size.1 {
            for x in 0..buffer.size.0 {
                let (cell, z_cells) = buffer.get_cell_mut((x, y));
                let result_cell = z_cells.result();

                if let Some(result_cell) = result_cell {
                    *cell = result_cell.clone();

                    if result_cell.transparent {
                        queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

                        batch_text.clear();
                        current_pos = (x.saturating_add(1), y);
                        current_style = Style::default();
                        continue;
                    }

                    if result_cell.style != current_style {
                        queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

                        batch_text.clear();
                        current_style = result_cell.style;
                        current_pos = (x, y);
                    }
                    batch_text.push(result_cell.c);
                } else {
                    queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

                    batch_text.clear();
                    current_pos = (x.saturating_add(1), y);
                    current_style = Style::default();
                    continue;
                }
            }

            queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

            batch_text.clear();
            current_pos = (0, y);
            current_style = Style::default();
        }

        self.stdout.flush()?;

        buffer.clear_changed();

        info!(target: "CORE", "RENDER CLEAR - Ended. Flushed {} render queue calls", unsafe {QUEUE_COUNT});
        Ok(())
    }
}

fn queue_batch(
    stdout: &mut Stdout,
    text: &str,
    position: (u16, u16),
    style: Style,
) -> io::Result<()> {
    if text.is_empty() {
        return Ok(());
    }

    let mut attr: crossterm::style::Attributes = crossterm::style::Attributes::none();
    if !style.modifiers.eq(&Modifiers::NONE) {
        if style.modifiers.contains(Modifiers::BOLD) {
            attr.set(style::Attribute::Bold);
        }
        if style.modifiers.contains(Modifiers::ITALIC) {
            attr.set(style::Attribute::Italic);
        }
        if style.modifiers.contains(Modifiers::DOUBLE_UNDERLINE) {
            attr.set(style::Attribute::DoubleUnderlined);
        }
        if style.modifiers.contains(Modifiers::UNDERLINE) {
            attr.set(style::Attribute::Underlined);
        }
        if style.modifiers.contains(Modifiers::REVERSE) {
            attr.set(style::Attribute::Reverse);
        }
        if style.modifiers.contains(Modifiers::BLINK) {
            attr.set(style::Attribute::SlowBlink);
        }
        if style.modifiers.contains(Modifiers::CONCEAL) {
            attr.set(style::Attribute::Hidden);
        }
        if style.modifiers.contains(Modifiers::STRIKETHROUGH) {
            attr.set(style::Attribute::CrossedOut);
        }
    }

    let fg: crossterm::style::Color = match style.fg {
        crate::style::color::Color::Default => Color::Reset,

        crate::style::color::Color::Black => Color::Black,
        crate::style::color::Color::DarkGrey => Color::DarkGrey,
        crate::style::color::Color::DarkRed => Color::DarkRed,
        crate::style::color::Color::Red => Color::Red,
        crate::style::color::Color::DarkGreen => Color::DarkGreen,
        crate::style::color::Color::Green => Color::Green,
        crate::style::color::Color::DarkYellow => Color::DarkYellow,
        crate::style::color::Color::Yellow => Color::Yellow,
        crate::style::color::Color::DarkBlue => Color::DarkBlue,
        crate::style::color::Color::Blue => Color::Blue,
        crate::style::color::Color::DarkMagenta => Color::DarkMagenta,
        crate::style::color::Color::Magenta => Color::Magenta,
        crate::style::color::Color::DarkCyan => Color::DarkCyan,
        crate::style::color::Color::Cyan => Color::Cyan,
        crate::style::color::Color::Grey => Color::Grey,
        crate::style::color::Color::White => Color::White,
        crate::style::color::Color::Ansi(v) => Color::AnsiValue(v),
        crate::style::color::Color::Rgba { r, g, b, a: _a } => Color::Rgb { r, g, b },
    };

    let bg: crossterm::style::Color = match style.bg {
        crate::style::color::Color::Default => Color::Reset,

        crate::style::color::Color::Black => Color::Black,
        crate::style::color::Color::DarkGrey => Color::DarkGrey,
        crate::style::color::Color::DarkRed => Color::DarkRed,
        crate::style::color::Color::Red => Color::Red,
        crate::style::color::Color::DarkGreen => Color::DarkGreen,
        crate::style::color::Color::Green => Color::Green,
        crate::style::color::Color::DarkYellow => Color::DarkYellow,
        crate::style::color::Color::Yellow => Color::Yellow,
        crate::style::color::Color::DarkBlue => Color::DarkBlue,
        crate::style::color::Color::Blue => Color::Blue,
        crate::style::color::Color::DarkMagenta => Color::DarkMagenta,
        crate::style::color::Color::Magenta => Color::Magenta,
        crate::style::color::Color::DarkCyan => Color::DarkCyan,
        crate::style::color::Color::Cyan => Color::Cyan,
        crate::style::color::Color::Grey => Color::Grey,
        crate::style::color::Color::White => Color::White,
        crate::style::color::Color::Ansi(v) => Color::AnsiValue(v),
        crate::style::color::Color::Rgba { r, g, b, a: _a } => Color::Rgb { r, g, b },
    };

    queue!(stdout, SetAttribute(style::Attribute::Reset))?;
    queue!(stdout, SetAttributes(attr))?;
    queue!(stdout, MoveTo(position.0, position.1))?;
    queue!(stdout, SetForegroundColor(fg))?;
    queue!(stdout, SetBackgroundColor(bg))?;
    queue!(stdout, Print(text))?;

    info!(target: "TERMINAL", "Queued FG: {:?}; BG: {:?}; POSITION: {:?}; ATTRS: {:?}; \nTEXT: {}", fg, bg, position, attr, text);
    unsafe {
        QUEUE_COUNT += 1;
    }

    Ok(())
}

fn get_capabilites() -> TerminalCapabilities {
    let mut color_space: ColorSpace = ColorSpace::Monochrome;
    if let Ok(value) = env::var("TERM") {
        if value.eq("vt100") || value.eq("dumb") {
            color_space = ColorSpace::Monochrome;
        } else if value.eq("xterm") {
            color_space = ColorSpace::Ansi;
        } else if value.eq("xterm-256color") || value.eq("screen-256color") {
            color_space = ColorSpace::HighColor;
        }
    };

    if let Ok(value) = env::var("COLORTERM")
        && value.eq("truecolor")
    {
        color_space = ColorSpace::TrueColor;
    }

    let capabilities = TerminalCapabilities { color_space };

    info!(target: "CORE", "Found Terminal capabilities: {:#?}", capabilities);

    capabilities
}
