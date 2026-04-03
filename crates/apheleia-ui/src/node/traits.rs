use std::any::Any;

use crate::contexts::node::NodeContext;

pub trait NodeTrait: Any {
    fn initial_setup(&mut self, ctx: &mut NodeContext);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
