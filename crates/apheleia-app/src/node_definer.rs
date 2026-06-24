use std::fmt::Debug;

use crate::context::node::NodeContext;

pub trait NodeDefiner: Debug {
    fn setup(self: Box<Self>, ctx: &mut NodeContext);
}

#[derive(Debug)]
pub struct EmptyNode;
impl NodeDefiner for EmptyNode {
    fn setup(self: Box<Self>, ctx: &mut NodeContext) {}
}

impl Default for Box<dyn NodeDefiner> {
    fn default() -> Self {
        Box::new(EmptyNode)
    }
}
