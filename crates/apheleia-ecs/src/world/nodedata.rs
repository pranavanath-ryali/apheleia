use crate::{nodedata::NodeData, types::NodeId, world::World};

impl World {
    #[inline]
    pub fn nodedata_write(&mut self, id: NodeId, data: NodeData) {
        self.nodedata.write_data(id, data);
    }

    #[inline]
    pub fn nodedata_remove(&mut self, id: NodeId) {
        self.nodedata.remove(id);
    }
}
