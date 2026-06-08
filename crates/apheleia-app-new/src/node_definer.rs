use std::fmt::Debug;

use crate::context::node::NodeContext;

pub trait NodeDefiner: Debug {
    fn setup(&mut self, ctx: &mut NodeContext);
}

#[derive(Debug)]
pub struct EmptyNode;
impl NodeDefiner for EmptyNode {
    fn setup(&mut self, ctx: &mut NodeContext) {}
}
