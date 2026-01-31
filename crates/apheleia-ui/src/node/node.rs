use apheleia_core::buffer::Buffer;

use crate::{contexts::Context, node::data::NodeData};

pub trait NodeTrait
{
    fn initial_setup(&mut self, ctx: &mut Context, data: &NodeData);

    fn event(&mut self, ctx: &mut Context, data: &NodeData);
    fn update(&mut self, ctx: &mut Context, data: &NodeData);
    fn render(&self, buf: &mut Buffer, ctx: &Context, data: &NodeData);
}
