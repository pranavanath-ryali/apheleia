use std::io::stdout;

use apheleia_core::{
    Color,
    buffer::Buffer,
    renderer::Renderer,
    style::{Style, StyleFlags},
    terminal,
};

fn main() {
    let size = terminal::size().unwrap();

    let mut buffer = Buffer::new(size.0, size.1);
    let mut renderer = Renderer {
        width: size.0,
        height: size.1,
        stdout: stdout(),
    };
    renderer.clear(&mut buffer);

    buffer.write_line(
        0,
        0,
        "<magenta>H<italic;darkgrey>e<cyan>l<normal;darkgreen>l<dim;darkgreen>o",
        None,
    );
    buffer.write_line(0, 1, "<blink;b;i>Hello World", None);

    renderer.render(&mut buffer);
    renderer.quit();
}
