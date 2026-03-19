pub mod builder;
pub mod contexts;
pub mod id_generator;
pub mod node;
pub mod rootnode;
pub mod types;

pub type NodeId = usize;
pub type ExtensionId = usize;

pub const FAKE_NODEID: NodeId = 0;
pub const MAX_NODES: NodeId = 1000;

pub use crossterm::event::*;

use crate::node::node::NodeTrait;

pub struct EmptyNode;
impl NodeTrait for EmptyNode {
    fn initial_setup(&mut self, _ctx: &mut contexts::Context) {}
    fn event(&mut self, _ctx: &mut contexts::Context) {}
    fn update(&mut self, _ctx: &mut contexts::Context) {}
    fn render(&self, _buf: &mut apheleia_core::buffer::Buffer, _ctx: &mut contexts::Context) {}
}
