use apheleia_ecs::{
    NodeId,
    command::ContextCommand,
    types::NodeData,
    utils::{calculate_global_position, calculate_global_size},
    world::{self, World},
};
use log::info;
use tree_ds::prelude::Node;

#[derive(Debug)]
pub struct CreateNode(pub NodeId);

impl CreateNode {
    pub fn new(id: NodeId) -> Box<Self> {
        Box::new(Self(id))
    }
}
impl ContextCommand for CreateNode {
    fn execute(self: Box<Self>, world: &mut apheleia_ecs::world::World) {}
}

#[derive(Debug)]
pub struct SetDataForNode(pub NodeId, pub NodeData);

impl SetDataForNode {
    pub fn new(id: NodeId, data: NodeData) -> Box<Self> {
        Box::new(Self(id, data))
    }
}
impl ContextCommand for SetDataForNode {
    fn execute(self: Box<Self>, world: &mut World) {
        world.set_data(self.0, self.1);
    }
}

#[derive(Debug)]
pub struct RelateNodeWithParent {
    pub node: NodeId,
    pub parent: NodeId,
}
impl RelateNodeWithParent {
    pub fn new(id: NodeId, parent: NodeId) -> Box<Self> {
        Box::new(Self { node: id, parent })
    }
}
impl ContextCommand for RelateNodeWithParent {
    fn execute(self: Box<Self>, world: &mut World) {
        // 1. If Node doesn't exist -> Create Node
        // 2. if node exists -> rewrite its relation

        let relations = world.get_relations_mut();
        if relations.get_node_by_id(&self.node).is_none() {
            relations
                .add_node(Node::new(self.node, None), Some(&self.parent))
                .unwrap_or_else(|_| {
                    panic!(
                        "Couldn't relate NodeId: {} with parent: {}",
                        self.node, self.parent
                    )
                });
            info!(
                "Node Id: {} related with parent Node Id: {}",
                self.node, self.parent
            );
        }

        // TODO: Write implementation to rewrite relation if node exists
    }
}

#[derive(Debug)]
pub struct CalculateGlobalPositionForNode(pub NodeId);
impl CalculateGlobalPositionForNode {
    pub fn new(id: NodeId) -> Box<Self> {
        Box::new(Self(id))
    }
}
impl ContextCommand for CalculateGlobalPositionForNode {
    fn execute(self: Box<Self>, world: &mut World) {
        world.get_nodedata_mut(self.0).unwrap().global_position =
            Some(calculate_global_position(world, self.0));
    }
}

#[derive(Debug)]
pub struct CalculateGlobalSizeForNode(pub NodeId);
impl CalculateGlobalSizeForNode {
    pub fn new(id: NodeId) -> Box<Self> {
        Box::new(Self(id))
    }
}
impl ContextCommand for CalculateGlobalSizeForNode {
    fn execute(self: Box<Self>, world: &mut World) {
        world.get_nodedata_mut(self.0).unwrap().global_size =
            Some(calculate_global_size(world, self.0));
    }
}
