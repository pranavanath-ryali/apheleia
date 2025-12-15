use apheleia_ui::{commands::{self, InitialCallContext}, node::node::NodeTrait};

pub struct Block;
impl NodeTrait for Block {
    fn initial_setup(&mut self, _ctx: &mut InitialCallContext) {}

    fn event(&mut self) {}
    fn update(&mut self) {}

    fn render(&self, buf: &mut apheleia_core::buffer::Buffer) {
        buf.write_line(0, 0, "+", None);
        // buf.write_line(0, self., "+", None);
        // buf.write_line(0, 0, "+", None);
        // buf.write_line(0, 0, "+", None);
    }
}
