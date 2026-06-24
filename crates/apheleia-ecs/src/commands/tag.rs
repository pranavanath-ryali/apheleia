use std::any::TypeId;

use crate::{commands::ContextCommand, types::NodeId};

#[derive(Debug)]
pub struct TagNode(pub NodeId, pub TypeId);
impl TagNode {
    pub fn new(id: NodeId, tag: TypeId) -> Box<Self> {
        Box::new(TagNode(id, tag))
    }
}

impl ContextCommand for TagNode {
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        world.tag_node_typeid(self.0, self.1);
    }
}
