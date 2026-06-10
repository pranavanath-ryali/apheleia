use apheleia_ecs_new::{
    NodeId,
    command::ContextCommand,
    types::NodeData,
    world::{self, World},
};

#[derive(Debug)]
pub struct CreateNode(pub NodeId);

impl CreateNode {
    pub fn new(id: NodeId) -> Box<Self> {
        Box::new(Self(id))
    }
}
impl ContextCommand for CreateNode {
    fn execute(self: Box<Self>, world: &mut apheleia_ecs_new::world::World) {}
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
