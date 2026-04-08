use apheleia_core::{buffer::Buffer, renderer::Renderer, rich_strings::RichString, terminal};

fn main() {
    let size = terminal::size().unwrap();

    let mut buffer = Buffer::new(size.0, size.1);
    let mut renderer = Renderer::new(size.0, size.1);
    renderer.init();

    buffer.write_rich_string(5, 5, RichString::new("H<fg:red;italic>el<under_lined>lo"));

    renderer.render_flip(&mut buffer);
    renderer.quit();
}
