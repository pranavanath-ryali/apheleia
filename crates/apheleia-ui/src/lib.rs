pub mod builder;
pub mod contexts;
pub mod node;
pub mod rootnode;
pub mod types;
pub mod utils;

pub type NodeId = usize;

pub const FAKE_NODEID: NodeId = 0;
pub const MAX_NODES: NodeId = 1000;

pub use crossterm::event::*;

use crate::node::node::NodeTrait;

pub struct EmptyNode;
impl NodeTrait for EmptyNode {
    fn initial_setup(&mut self, ctx: &mut contexts::Context) {
        todo!()
    }

    fn event(&mut self, ctx: &mut contexts::Context) {
        todo!()
    }

    fn update(&mut self, ctx: &mut contexts::Context) {
        todo!()
    }

    fn render(&self, buf: &mut apheleia_core::buffer::Buffer, ctx: &mut contexts::Context) {
        todo!()
    }
}
