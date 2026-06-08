use crate::context::node::NodeContext;

pub trait NodeDefiner {
    fn setup(&mut self, ctx: &mut NodeContext);
}

pub struct EmptyNode;
impl NodeDefiner for EmptyNode {
    fn setup(&mut self, ctx: &mut NodeContext) {}
}
