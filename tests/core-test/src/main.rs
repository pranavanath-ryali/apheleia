use apheleia_core::{
    buffer::Buffer,
    node_buffer::NodeBuffer,
    renderer::Renderer,
    style::Style,
    terminal::{self, size},
    types::Vec2,
};

fn main() {
    let mut renderer = Renderer::default();
    let size = terminal::size().unwrap();
    let mut buffer = Buffer::new(Vec2 {
        x: size.0 as u32,
        y: size.1 as u32,
    });

    let mut node_buffer = NodeBuffer::new(Vec2 { x: 0, y: 0 }, Vec2 { x: 20, y: 10 });
    node_buffer.write_string(
        Vec2 { x: 0, y: 0 },
        &" ".repeat(20),
        Some(Style {
            bg: apheleia_core::Color::Green,
            ..Default::default()
        }),
    );

    node_buffer.write_string(
        Vec2 { x: 3, y: 0 },
        "         ",
        Some(Style {
            bg: apheleia_core::Color::Red,
            ..Default::default()
        }),
    );

    let mut other_buffer = NodeBuffer::new(Vec2 { x: 5, y: 0 }, Vec2 { x: 20, y: 10 });
    other_buffer.write_string(Vec2 { x: 0, y: 0 }, "BRUH WHY DOES IT", None);

    buffer.render_buffer(&mut node_buffer);
    buffer.render_buffer(&mut other_buffer);

    renderer.render_flip(&mut buffer);
}
