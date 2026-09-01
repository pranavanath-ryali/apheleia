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
    
    buffer.fill_rect((0, 0), size, 0, Color::Black, None);

    term.render_clear(&mut buffer)?;

    let mut x = 0u16;
    let mut y = 0u16;

    let mut direction_x = 1i32;
    let mut direction_y = 1i32;

    let mut prev_position = (x, y);

    loop {
        thread::sleep(Duration::from_millis(30));

        if x != 0 {
            buffer.clear_cell_z(prev_position, 1);
        }
        buffer.write_text(
            "󰝥",
            (x, y),
            1,
            Style {
                ..Default::default()
            },
            Some(150)
        );

        if x >= size.0 - 1 {
            direction_x = -1;
        }
        if y >= size.1 - 1 {
            direction_y = -1;
        }

        if x == 0 {
            direction_x = 1;
        }
        if y == 0 {
            direction_y = 1;
        }

        prev_position = (x, y);

        x = (x as i32 + direction_x) as u16;
        y = (y as i32 + direction_y) as u16;
        term.render_update(&mut buffer)?;
    }

    Ok(())
}
