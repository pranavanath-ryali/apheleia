use apheleia_core::{buffer::Buffer, node_buffer::NodeBuffer, renderer::Renderer, types::Vec2};

fn main() {
    let mut renderer = Renderer::default();
    let mut buffer = Buffer::new(Vec2 { x: 50, y: 50 });

    let mut node_buffer = NodeBuffer::new(Vec2 { x: 10, y: 10 });

    node_buffer.write_rich_string(Vec2 { x: 0, y: 0 }, "</reverse;bold/>Hello");

    renderer.init();
    buffer.render_buffer(Vec2 { x: 0, y: 0 }, &mut node_buffer);
    _ = renderer.render_flip(&mut buffer);

    loop {
        renderer.render(&mut buffer);
    }
    renderer.quit();
}
