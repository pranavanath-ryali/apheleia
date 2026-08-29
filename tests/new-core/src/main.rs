use std::io;

use apheleia_core::{
    buffer::Buffer,
    cell::{Color, Modifiers, Style},
    terminal::{self, Terminal},
};

fn main() -> io::Result<()> {
    let mut term = Terminal::new();
    println!("Terminal Capabilites: {:#?}", term.capabilities);
    term.init()?;

    let size = crossterm::terminal::size().unwrap();
    println!("Terminal Size: {:?}", size);

    let mut buffer = Buffer::new(size);

    buffer.write(
        "Hello",
        (0, 0),
        0,
        Style {
            fg: Color::Red,
            ..Default::default()
        },
    );
    buffer.write(
        "HAHAHHA",
        (5, 0),
        0,
        Style {
            fg: Color::Cyan,
            ..Default::default()
        },
    );
    buffer.write("WORLD", (50, 10), 0, Style::default());

    term.render_clear(&mut buffer)?;

    Ok(())
}
