use rustc_hash::FxHashMap;

use crate::{nodedata::NodeData, types::NodeId};

#[derive(Default)]
pub struct NodeDataStore {
    data: FxHashMap<NodeId, NodeData>
}
impl NodeDataStore {
    pub fn write_data(&mut self, id: NodeId, data: NodeData) {
        self.data
            .entry(id)
            .and_modify(|d| *d = data.clone())
            .or_insert(data);
    }
}
