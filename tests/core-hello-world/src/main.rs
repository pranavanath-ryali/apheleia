use apheleia_core::{
    Color, buffer::Buffer, renderer::Renderer, style::{Style, StyleFlags}, terminal
};

fn main() {
    let size = terminal::size().unwrap();

    let mut buffer = Buffer::new(size.0, size.1);
    let mut renderer = Renderer::new();

    buffer.write_line(
        10,
        10,
        "Hello World!",
        Some(Style {
            fg: Color::Red,
            bg: Color::Blue,
            flags: StyleFlags::BOLD | StyleFlags::ITALIC,
            ..Default::default()
        }),
    );
    renderer.flip(&mut buffer);

    buffer.write_line(
        50,
        20,
        "WEEEEEEEEE",
        Some(Style {
            fg: Color::Blue,
            flags: StyleFlags::ITALIC | StyleFlags::UNDERCURLED,
            ..Default::default()
        }),
    );
    renderer.update(&mut buffer);
}
