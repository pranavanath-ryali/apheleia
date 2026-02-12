use apheleia_core::types::vector::Vector2;

use crate::{NodeId, contexts::ContextCommand, rootnode::{self, RootNodeData}, types::EventType};

pub struct Command_SetSizeForId(pub NodeId, pub Vector2);
pub struct Command_SetPositionForId(pub NodeId, pub Vector2);
pub struct Command_RegisterForUpdate;
pub struct Command_RegisterForEvent(pub EventType);

impl ContextCommand for Command_SetSizeForId {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data.id_data.get_mut(&self.0).unwrap().set_size(self.1);
    }
}
impl ContextCommand for Command_SetPositionForId {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data.id_data.get_mut(&self.0).unwrap().set_position(self.1);
    }
}
