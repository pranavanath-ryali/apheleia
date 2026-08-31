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
       None 
    );
    buffer.write(
        "HAHAHHA",
        (4, 0),
        0,
        Style {
            fg: Color::Cyan,
            ..Default::default()
        },

       None 
    );
    buffer.write("WORLD", (50, 10), 0, Style::default(), None);
    buffer.write("     ", (10, 10), 0, Style { bg: Color::Red, ..Default::default() }, None);

    buffer.write("HELLO", (10, 10), 1, Style { fg: Color::Black,  modifiers: Modifiers::BOLD, ..Default::default() }, None);
    buffer.write("WEE", (12, 10), 3, Style { modifiers: Modifiers::ITALIC, ..Default::default() }, None);

    term.render_clear(&mut buffer)?;

    Ok(())
}
