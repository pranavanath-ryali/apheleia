use crate::{contexts, node::node::NodeTrait};

pub mod data;
pub mod node;

pub struct EmptyNode;
impl NodeTrait for EmptyNode {
    fn initial_setup(&mut self, _ctx: &mut contexts::Context) {}
    fn event(&mut self, _ctx: &mut contexts::Context) {}
    fn update(&mut self, _ctx: &mut contexts::Context) {}
    fn render(&self, _buf: &mut apheleia_core::buffer::Buffer, _ctx: &mut contexts::Context) {}
}
