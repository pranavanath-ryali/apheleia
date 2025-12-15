use apheleia_core::{Color, style::Style};
use apheleia_ui::{commands::{self, InitialCallContext}, contexts::RenderContext, node::node::NodeTrait};

pub struct Block;
impl NodeTrait for Block {
    fn initial_setup(&mut self, _ctx: &mut InitialCallContext) {}

    fn event(&mut self) {}
    fn update(&mut self) {}

    fn render(&self, ctx: &mut RenderContext, buf: &mut apheleia_core::buffer::Buffer) {
        buf.write_line(0, 0, "+", None);
        buf.write_line(0, ctx.size.1 - 1, "+", None);
        buf.write_line(ctx.size.0 - 1, 0, "+", None);
        buf.write_line(ctx.size.0 - 1, ctx.size.1 - 1, "+", None);

        buf.write_line(1, 1, &format!("X: {}", ctx.position.0), Some(Style { fg: Color::Blue, ..Default::default() }));
        buf.write_line(1, 2, &format!("Y: {}", ctx.position.1), Some(Style { fg: Color::Blue, ..Default::default() }));
        buf.write_line(1, 3, &format!("WIDTH: {}", ctx.size.0), Some(Style { fg: Color::Red, ..Default::default() }));
        buf.write_line(1, 4, &format!("HEIGHT: {}", ctx.size.1), Some(Style { fg: Color::Red, ..Default::default() }));
    }
}
