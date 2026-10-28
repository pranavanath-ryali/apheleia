use std::{io, thread::sleep, time::Duration};

use apheleia_core::{buffer::Buffer, terminal::Terminal};
use apheleia_ecs::{types::traits::Extension, world::World};

struct TestExt {
    x: u16,
}
impl Extension for TestExt {}

fn main() -> io::Result<()> {
    let mut term = Terminal::new();
    term.init()?;

    let size = crossterm::terminal::size().unwrap();
    println!("Terminal Size: {:?}", size);

    let mut buffer = Buffer::new(size);

    let mut world = World::default();
    world.register_extension::<TestExt>();

    world.add_extension(0, TestExt { x: 10 });
    buffer.write(
        &format!("{:?}", world.get_extension::<TestExt>(0).unwrap().x),
        (0, 0),
        0,
        None,
    );

    term.render_clear(&mut buffer)?;
    loop {
        sleep(Duration::from_micros(500));
        term.render_update(&mut buffer)?;
    }

    Ok(())
}
