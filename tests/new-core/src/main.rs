use std::{io, thread, time::Duration};

use apheleia_core::{
    buffer::{Buffer, MultiLayerCellTrait},
    cell::{Cell, Color, Grapheme, Modifiers, Style},
    terminal::{self, Terminal},
};
use smallvec::SmallVec;

fn main() -> io::Result<()> {
    let mut term = Terminal::new();
    println!("Terminal Capabilites: {:#?}", term.capabilities);
    term.init()?;

    let size = crossterm::terminal::size().unwrap();
    println!("Terminal Size: {:?}", size);

    let mut buffer = Buffer::new(size);

    println!("Size of style: {}", size_of::<Style>());
    println!("Size of graphene: {}", size_of::<Grapheme>());
    println!("Size of cell: {}", size_of::<Cell>());
    println!(
        "Size of smallvec len - 2: {}",
        size_of::<SmallVec<[(u8, Cell); 2]>>()
    );
    println!(
        "Size of cell & smallvec len - 2: {}",
        size_of::<(Cell, SmallVec<[(u8, Cell); 2]>)>() * size.0 as usize * size.1 as usize
    );

    term.render_clear(&mut buffer)?;

    let mut x = 0u16;
    loop {
        thread::sleep(Duration::from_millis(60));

        if x != 0 {
            // buffer.clear_cell((x - 1, x - 1));
        }
        buffer.write_text(
            "0",
            (x, x),
            0,
            Style {
                ..Default::default()
            },
            None,
        );

        x += 1;
        term.render_update(&mut buffer)?;
    }

    Ok(())
}
