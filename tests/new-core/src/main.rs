use std::{io, thread, time::Duration};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use apheleia_core::{buffer::Buffer, style::Style, terminal::Terminal};
use smallvec::SmallVec;

fn main() -> io::Result<()> {
    let mut term = Terminal::new();
    term.init()?;

    let size = crossterm::terminal::size().unwrap();
    println!("Terminal Size: {:?}", size);

    let mut buffer = Buffer::new(size);

    let s = "➔";
    println!("Standard width: {}", UnicodeWidthStr::width(s));

    // Test if CJK width differs
    use unicode_width::UnicodeWidthChar;
    println!("CJK width: {}", UnicodeWidthChar::width_cjk('➔').unwrap());

    let teststr = "Ｈｅｌｌｏ, ｗｏｒｌｄ!";
    println!("{teststr}");
    println!("Size: {}", teststr.len());
    println!("Count: {}", teststr.chars().count());

    // term.render_clear(&mut buffer)?;
    // buffer.write("╴hi➔hello", (0, 0), 0, Style::default(), None);
    // term.render_update(&mut buffer)?;

    // buffer.fill_rect((0, 0), size, 0, Color::Black, None);
    //
    // term.render_clear(&mut buffer)?;
    //
    // let mut x = 0u16;
    // let mut y = 0u16;
    //
    // let mut direction_x = 1i32;
    // let mut direction_y = 1i32;
    //
    // let mut prev_position = (x, y);
    //
    // loop {
    //     thread::sleep(Duration::from_millis(30));
    //
    //     if x != 0 {
    //         buffer.clear_cell_z(prev_position, 1);
    //     }
    //     buffer.write_text(
    //         "󰝥",
    //         (x, y),
    //         1,
    //         Style {
    //             ..Default::default()
    //         },
    //         Some(150)
    //     );
    //
    //     if x >= size.0 - 1 {
    //         direction_x = -1;
    //     }
    //     if y >= size.1 - 1 {
    //         direction_y = -1;
    //     }
    //
    //     if x == 0 {
    //         direction_x = 1;
    //     }
    //     if y == 0 {
    //         direction_y = 1;
    //     }
    //
    //     prev_position = (x, y);
    //
    //     x = (x as i32 + direction_x) as u16;
    //     y = (y as i32 + direction_y) as u16;
    //     term.render_update(&mut buffer)?;
    // }
    //
    Ok(())
}
