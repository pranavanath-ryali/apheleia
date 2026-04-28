use tree_ds::prelude::Tree;

use crate::{ContextCommand, NodeId, world_access::WorldAccess};

pub struct SystemContext {
    relations: Tree<NodeId, NodeId>,
    world: Box<dyn WorldAccess>,

    commands: Vec<Box<dyn ContextCommand>>,
}
impl SystemContext {
    pub fn new(relations: Tree<NodeId, NodeId>, world: Box<dyn WorldAccess>) -> Self {
        Self {
            relations,
            world,
            commands: vec![],
        }
    }
}
