use apheleia_core::buffer::Buffer;

use crate::contexts::{self, Context};

pub mod data;
pub mod node_storage;

pub trait NodeTrait {
    fn initial_setup(&mut self, ctx: &mut Context);

    fn event(&mut self, ctx: &mut Context);
    fn update(&mut self, ctx: &mut Context);
    fn render(&self, buf: &mut Buffer, ctx: &mut Context);
}

pub struct EmptyNode;
impl NodeTrait for EmptyNode {
    fn initial_setup(&mut self, _ctx: &mut contexts::Context) {}
    fn event(&mut self, _ctx: &mut contexts::Context) {}
    fn update(&mut self, _ctx: &mut contexts::Context) {}
    fn render(&self, _buf: &mut apheleia_core::buffer::Buffer, _ctx: &mut contexts::Context) {}
}
