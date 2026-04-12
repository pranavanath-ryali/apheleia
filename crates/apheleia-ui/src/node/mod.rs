use std::any::Any;

use crate::{contexts::node::NodeContext, node::traits::NodeTrait};

pub mod data;
pub(crate) mod store;
pub mod traits;

pub struct EmptyNode;
impl NodeTrait for EmptyNode {
    fn initial_setup(&mut self, _ctx: &mut NodeContext) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
