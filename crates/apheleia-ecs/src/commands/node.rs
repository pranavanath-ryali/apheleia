use crate::{
    commands::ContextCommand,
    nodedata::data::NodeData,
    types::NodeId,
    utils::{calculate_global_position, calculate_global_size},
};

#[derive(Debug)]
pub struct SetDataForNode(pub NodeId, pub NodeData);
#[derive(Debug)]
pub struct RelateChildWithParent {
    pub parent: NodeId,
    pub child: NodeId,
}
#[derive(Debug)]
pub struct CalculateGlobalPositionForNode(pub NodeId);
#[derive(Debug)]
pub struct CalculateGlobalSizeForNode(pub NodeId);

impl SetDataForNode {
    pub fn new(id: NodeId, data: NodeData) -> Box<Self> {
        Box::new(Self(id, data))
    }
}
impl ContextCommand for SetDataForNode {
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        world.set_data(self.0, self.1);
    }
}

impl RelateChildWithParent {
    pub fn new(child: NodeId, parent: NodeId) -> Box<Self> {
        Box::new(Self { child, parent })
    }
}
impl ContextCommand for RelateChildWithParent {
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        world.relate_node_with_parent(self.child, self.parent);
    }
}

impl CalculateGlobalPositionForNode {
    pub fn new(id: NodeId) -> Box<Self> {
        Box::new(Self(id))
    }
}
impl ContextCommand for CalculateGlobalPositionForNode {
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        let position = calculate_global_position(world, self.0);
        world
            .get_nodedata_mut(self.0)
            .expect("Unexpected error trying to unwrap nodedata")
            .global_position = Some(position);
    }
}

impl CalculateGlobalSizeForNode {
    pub fn new(id: NodeId) -> Box<Self> {
        Box::new(Self(id))
    }
}
impl ContextCommand for CalculateGlobalSizeForNode {
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        let size = calculate_global_size(world, self.0);
        world
            .get_nodedata_mut(self.0)
            .expect("Unexpected error trying to unwrap nodedata")
            .global_size = Some(size);
    }
}
