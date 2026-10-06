use apheleia_types::sparse_set::SlotMap;

use crate::{nodedata::NodeData, types::NodeId};

pub struct NodeDataStore {
    data: SlotMap<NodeData>,
}
impl Default for NodeDataStore {
    fn default() -> Self {
        Self {
            data: SlotMap::new(1),
        }
    }
}
impl NodeDataStore {
    #[inline]
    pub fn write_data(&mut self, id: NodeId, data: NodeData) {
        self.data.insert(id as u32, data);
    }

    #[inline]
    pub fn get(&self, id: NodeId) -> Option<&NodeData> {
        self.data.get(id as u32)
    }

    #[inline]
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut NodeData> {
        self.data.get_mut(id as u32)
    }
}
