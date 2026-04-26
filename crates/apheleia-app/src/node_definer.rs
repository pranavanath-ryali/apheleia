use crate::contexts::node::NodeContext;

pub trait NodeDefiner {
    fn setup(&mut self, ctx: &mut NodeContext);
}
