use crate::{commands::WorldCommand, nodedata::NodeData, types::NodeId};

pub struct SetNodeData(pub NodeId, pub NodeData);
impl SetNodeData {
    pub fn new(id: NodeId, data: NodeData) -> Box<Self> {
        Box::new(Self(id, data))
    }
}
impl WorldCommand for SetNodeData {
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        world.nodedata_write(self.0, self.1);
    }
}
