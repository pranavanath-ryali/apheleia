use std::io;

use apheleia_core::{buffer::Buffer, cell::{Modifiers, Style}, terminal::{self, Terminal}};

fn main() -> io::Result<()> {
    let mut term = Terminal::new();
    println!("Terminal Capabilites: {:#?}", term.capabilities);

    let size = crossterm::terminal::size().unwrap();
    println!("Terminal Size: {:?}", size);

    let mut buffer = Buffer::new(size);

    buffer.write("Hello", (0, 0), 0, Style {
        modifiers: Modifiers::NONE
    });
    buffer.write("WORLD", (50, 10), 0, Style::default());
    
    println!("Cell 50, 10: {:?}", buffer.get_result_cell((50, 10)));

    term.render_clear(&mut buffer)?;

    Ok(())
}
