use std::io;

use apheleia_core::{
    buffer::Buffer,
    style::{Style, modifiers::Modifiers},
    terminal::Terminal,
};

fn main() -> io::Result<()> {
    let mut term = Terminal::new();
    term.init()?;

    let size = crossterm::terminal::size().unwrap();
    println!("Terminal Size: {:?}", size);

    let mut buffer = Buffer::new(size);

    term.render_clear(&mut buffer)?;

    buffer.write(
        "HELLO WORLD",
        (0, 0),
        0,
        Some(Style {
            fg: apheleia_core::style::color::Color::Rgba {
                r: 128,
                g: 97,
                b: 46,
                a: 120,
            },
            bg: apheleia_core::style::color::Color::Default,
            modifiers: Modifiers::BOLD | Modifiers::ITALIC,
        }),
    );
    buffer.write(
        "    ",
        (0, 0),
        -1,
        Some(Style {
            fg: apheleia_core::style::color::Color::Default,
            bg: apheleia_core::style::color::Color::Rgba {
                r: 0,
                g: 126,
                b: 126,
                a: 255,
            },
            modifiers: Modifiers::BOLD | Modifiers::ITALIC,
        }),
    );
    buffer.write(
        " ",
        (0, 0),
        -1,
        Some(Style {
            fg: apheleia_core::style::color::Color::Default,
            bg: apheleia_core::style::color::Color::Rgba {
                r: 255,
                g: 255,
                b: 255,
                a: 100,
            },
            modifiers: Modifiers::BOLD | Modifiers::ITALIC,
        }),
    );

    term.render_update(&mut buffer)?;

    Ok(())
}
