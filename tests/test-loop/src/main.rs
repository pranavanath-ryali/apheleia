use std::{io, thread::sleep, time::Duration};

use apheleia_core::{buffer::Buffer, terminal::Terminal};
use apheleia_ecs::world::World;

fn main() -> io::Result<()> {
    let mut term = Terminal::new();
    term.init()?;

    let size = crossterm::terminal::size().unwrap();
    println!("Terminal Size: {:?}", size);

    let mut buffer = Buffer::new(size);

    term.render_clear(&mut buffer)?;

    let mut world = World::default();

    loop {
        sleep(Duration::new(1, 0));
    }


    Ok(())
}
