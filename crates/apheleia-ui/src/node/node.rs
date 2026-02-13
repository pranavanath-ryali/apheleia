use crate::contexts::Context;
use apheleia_core::buffer::Buffer;

pub trait NodeTrait {
    fn initial_setup(&mut self, ctx: &mut Context);

    fn event(&mut self, ctx: &mut Context);
    fn update(&mut self, ctx: &mut Context);
    fn render(&self, buf: &mut Buffer, ctx: &Context);
}
