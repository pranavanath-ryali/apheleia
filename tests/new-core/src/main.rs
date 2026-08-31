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
            fg: Color::White,
            bg: Color::Rgb {
                r: 123,
                g: 53,
                b: 87,
            },
            ..Default::default()
        },
        Some(150),
    );
    buffer.write(
        "HI",
        (0, 0),
        1,
        Style {
            bg: Color::Rgb { r: 0, g: 150, b: 60 },
            ..Default::default()
        },
        Some(150),
    );
    // buffer.write(
    //     "Z",
    //     (0, 0),
    //     2,
    //     Style {
    //         bg: Color::Rgb {
    //             r: 150,
    //             g: 15,
    //             b: 15,
    //         },
    //         ..Default::default()
    //     },
    //     Some(20),
    // );

    // buffer.write(
    //     "Hello",
    //     (0, 1),
    //     0,
    //     Style {
    //         fg: Color::White,
    //         bg: Color::Rgb {
    //             r: 100,
    //             g: 0,
    //             b: 255,
    //         },
    //         ..Default::default()
    //     },
    //     None,
    // );
    // buffer.write(
    //     "Z",
    //     (0, 1),
    //     2,
    //     Style {
    //         bg: Color::Rgb {
    //             r: 150,
    //             g: 15,
    //             b: 15,
    //         },
    //         ..Default::default()
    //     },
    //     Some(155),
    // );
    //
    //
    // buffer.write(
    //     "Hello",
    //     (0, 2),
    //     0,
    //     Style {
    //         fg: Color::White,
    //         bg: Color::Rgb {
    //             r: 100,
    //             g: 0,
    //             b: 255,
    //         },
    //         ..Default::default()
    //     },
    //     Some(20),
    // );
    // buffer.write(
    //     "HI",
    //     (0, 2),
    //     1,
    //     Style {
    //         bg: Color::Rgb { r: 0, g: 255, b: 0 },
    //         ..Default::default()
    //     },
    //     Some(155),
    // );
    // buffer.write(
    //     "Z",
    //     (0, 2),
    //     2,
    //     Style {
    //         bg: Color::Rgb {
    //             r: 150,
    //             g: 15,
    //             b: 15,
    //         },
    //         ..Default::default()
    //     },
    //     Some(155),
    // );

    // buffer.write("WORLD", (50, 10), 0, Style::default(), None);
    // buffer.write("     ", (10, 10), 0, Style { bg: Color::Red, ..Default::default() }, None);
    //
    // buffer.write("HELLO", (10, 10), 1, Style { fg: Color::Black,  modifiers: Modifiers::BOLD, ..Default::default() }, None);
    // buffer.write("WEE", (12, 10), 3, Style { modifiers: Modifiers::ITALIC, ..Default::default() }, None);

    term.render_clear(&mut buffer)?;

    Ok(())
}
