use apheleia_core::buffer::NodeBuffer;

pub trait NodeTrait {
    fn update(&mut self);
    fn render(&self, buf: &mut NodeBuffer);
}
