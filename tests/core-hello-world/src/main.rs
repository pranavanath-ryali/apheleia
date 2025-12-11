use apheleia_core::{Attribute, Color, buffer::Buffer, renderer::Renderer, style::Style, terminal};

fn main() {
    let size = terminal::size().unwrap();

    let mut buffer = Buffer::new(size.0, size.1);
    let mut renderer = Renderer::new();

    buffer.write_line(0, 0, "Hello World!", Some(Style {
        fg: Color::Blue,
        attrs: Some(Attribute::Bold),
        ..Default::default()
    }));
    renderer.flip(&mut buffer);
}
