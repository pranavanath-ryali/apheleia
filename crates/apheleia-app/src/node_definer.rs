use crate::contexts::node::NodeContext;

pub trait NodeDefiner {
    fn setup(&self, ctx: &mut NodeContext);
}
