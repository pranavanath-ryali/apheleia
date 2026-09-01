use std::{
    env,
    io::{self, Stdout, Write, stdout},
    mem::take,
    ops::ControlFlow::Continue,
};

use crossterm::{
    cursor::{self, MoveTo},
    execute, queue,
    style::{
        self, Attributes, Color, Print, SetAttribute, SetAttributes, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::Clear,
};

use crate::{
    buffer::{Buffer, MultiLayerCellTrait},
    cell::{Modifiers, Style},
};

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
        buffer.changed_cells.sort_unstable_by_key(|v| (v.1, v.0));
        buffer.changed_cells.dedup();

        let mut batch_text = String::new();
        let mut current_style = Style::default();
        let mut current_pos = (0u16, 0u16);
        let mut offset_x = 0u16;

        let changed_cells = take(&mut buffer.changed_cells);
        for (x, y) in changed_cells {
            if current_pos.1 != y || current_pos.0 + offset_x != x {
                queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

                batch_text.clear();
                current_pos = (x, y);
                current_style = Style::default();
                offset_x = 0;
            }

            // println!("{:?}; Cell: {:?}", (current_pos.0 + offset_x, current_pos.1), result_cell);
            let (cell, z_cells) = buffer.get_cell_mut((current_pos.0 + offset_x, current_pos.1));
            let result_cell = z_cells.result();

            match result_cell {
                crate::cell::Cell::Transparent => {
                    queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

                    batch_text.clear();
                    current_pos = (x, y);
                    offset_x = 0;
                    current_style = Style::default();

                    batch_text.push(' ');
                    offset_x += 1;

                    continue;
                }
                crate::cell::Cell::Opaque { grapheme, style } => {
                    if result_cell == *cell {
                        queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

                        batch_text.clear();
                        current_pos = (x, y);
                        current_style = style;
                        offset_x = 0;

                        match grapheme {
                            crate::cell::Grapheme::Char(ch) => batch_text.push(ch),
                            crate::cell::Grapheme::Width(_) => todo!(),
                        }
                        continue;
                    }

                    if style != current_style {
                        queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

                        batch_text.clear();
                        current_pos = (x, y);
                        current_style = style;
                        offset_x = 1;
                        *cell = result_cell;

                        match grapheme {
                            crate::cell::Grapheme::Char(ch) => batch_text.push(ch),
                            crate::cell::Grapheme::Width(_) => todo!(),
                        }

                        continue;
                    }

                    match grapheme {
                        crate::cell::Grapheme::Char(ch) => batch_text.push(ch),
                        crate::cell::Grapheme::Width(_) => todo!(),
                    }
                }
                crate::cell::Cell::Translucent {
                    grapheme,
                    style,
                    alpha,
                } => panic!("Result cell cannot be a Translucent"),
            }

            *cell = result_cell;
            offset_x += 1;
        }

        queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

        queue!(&mut self.stdout, SetForegroundColor(Color::Reset))?;
        queue!(&mut self.stdout, SetBackgroundColor(Color::Reset))?;

        self.stdout.flush()?;

        Ok(())
    }

    pub fn render_clear(&mut self, buffer: &mut Buffer) -> io::Result<()> {
        execute!(self.stdout, Clear(crossterm::terminal::ClearType::All))?;

        let mut batch_text = String::new();
        let mut current_style = Style::default();
        let mut current_pos = (0u16, 0u16);
        for y in 0..buffer.size.1 {
            for x in 0..buffer.size.0 {
                let (cell, z_cells) = buffer.get_cell_mut((x, y));
                *cell = z_cells.result();

                match cell {
                    crate::cell::Cell::Transparent => {
                        queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

                        batch_text.clear();
                        current_pos = (x.saturating_add(1), y);
                        current_style = Style::default();
                        continue;
                    }
                    crate::cell::Cell::Opaque { grapheme, style } => {
                        if *style != current_style {
                            queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

                            batch_text.clear();
                            current_style = *style;
                            current_pos = (x, y);
                        }

                        match grapheme {
                            // crate::cell::Grapheme::Ascii(_) => todo!(),
                            crate::cell::Grapheme::Char(c) => {
                                batch_text.push(*c);
                            }
                            crate::cell::Grapheme::Width(_) => todo!(),
                            // crate::cell::Grapheme::Extended => todo!(),
                        }
                    }
                    crate::cell::Cell::Translucent {
                        grapheme: _,
                        style: _,
                        alpha: _,
                    } => {
                        panic!("Final result of cell is not supposed to be Translucent");
                    }
                }
            }

            queue_batch(&mut self.stdout, &batch_text, current_pos, current_style)?;

            batch_text.clear();
            current_pos = (0, y);
            current_style = Style::default();
        }

        self.stdout.flush()?;

        buffer.clear_changed();
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

    // println!("BATCHED: {}; POS: {:?}, STYLE: {:?}", text, position, style);

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
        crate::cell::Color::Reset => Color::Reset,

        crate::cell::Color::Black => Color::Black,
        crate::cell::Color::DarkGrey => Color::DarkGrey,
        crate::cell::Color::DarkRed => Color::DarkRed,
        crate::cell::Color::Red => Color::Red,
        crate::cell::Color::DarkGreen => Color::DarkGreen,
        crate::cell::Color::Green => Color::Green,
        crate::cell::Color::DarkYellow => Color::DarkYellow,
        crate::cell::Color::Yellow => Color::Yellow,
        crate::cell::Color::DarkBlue => Color::DarkBlue,
        crate::cell::Color::Blue => Color::Blue,
        crate::cell::Color::DarkMagenta => Color::DarkMagenta,
        crate::cell::Color::Magenta => Color::Magenta,
        crate::cell::Color::DarkCyan => Color::DarkCyan,
        crate::cell::Color::Cyan => Color::Cyan,
        crate::cell::Color::Grey => Color::Grey,
        crate::cell::Color::White => Color::White,
        crate::cell::Color::Ansi(v) => Color::AnsiValue(v),
        crate::cell::Color::Rgb { r, g, b } => Color::Rgb { r, g, b },
    };

    let bg: crossterm::style::Color = match style.bg {
        crate::cell::Color::Reset => Color::Reset,

        crate::cell::Color::Black => Color::Black,
        crate::cell::Color::DarkGrey => Color::DarkGrey,
        crate::cell::Color::DarkRed => Color::DarkRed,
        crate::cell::Color::Red => Color::Red,
        crate::cell::Color::DarkGreen => Color::DarkGreen,
        crate::cell::Color::Green => Color::Green,
        crate::cell::Color::DarkYellow => Color::DarkYellow,
        crate::cell::Color::Yellow => Color::Yellow,
        crate::cell::Color::DarkBlue => Color::DarkBlue,
        crate::cell::Color::Blue => Color::Blue,
        crate::cell::Color::DarkMagenta => Color::DarkMagenta,
        crate::cell::Color::Magenta => Color::Magenta,
        crate::cell::Color::DarkCyan => Color::DarkCyan,
        crate::cell::Color::Cyan => Color::Cyan,
        crate::cell::Color::Grey => Color::Grey,
        crate::cell::Color::White => Color::White,
        crate::cell::Color::Ansi(v) => Color::AnsiValue(v),
        crate::cell::Color::Rgb { r, g, b } => Color::Rgb { r, g, b },
    };

    queue!(stdout, SetAttribute(style::Attribute::Reset))?;
    queue!(stdout, SetAttributes(attr))?;
    queue!(stdout, MoveTo(position.0, position.1))?;
    queue!(stdout, SetForegroundColor(fg))?;
    queue!(stdout, SetBackgroundColor(bg))?;
    queue!(stdout, Print(text))?;

    Ok(())
}

fn get_capabilites() -> TerminalCapabilities {
    let mut color_space: ColorSpace = ColorSpace::Monochrome;
    match env::var("TERM") {
        Ok(value) => {
            if value.eq("vt100") || value.eq("dumb") {
                color_space = ColorSpace::Monochrome;
            } else if value.eq("xterm") {
                color_space = ColorSpace::Ansi;
            } else if value.eq("xterm-256color") || value.eq("screen-256color") {
                color_space = ColorSpace::HighColor;
            }
        }
        Err(_) => (),
    };
    match env::var("COLORTERM") {
        Ok(value) => {
            if value.eq("truecolor") {
                color_space = ColorSpace::TrueColor;
            }
        }
        Err(_) => (),
    }

    TerminalCapabilities { color_space }
}
