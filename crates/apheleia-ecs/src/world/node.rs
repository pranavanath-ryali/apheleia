use std::collections::linked_list;

use crate::{types::NodeId, world::World};

impl World {
    #[inline]
    pub fn node_create(&mut self) -> NodeId {
        self.nodeid_gen.next()
    }

    #[inline]
    pub fn node_delete(&mut self, id: NodeId) {
        self.nodeid_gen.free(id);
    }
}
