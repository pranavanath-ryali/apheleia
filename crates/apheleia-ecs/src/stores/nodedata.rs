use rustc_hash::FxHashMap;

use crate::{nodedata::NodeData, types::NodeId};

#[derive(Default)]
pub struct NodeDataStore {
    data: Vec<NodeData>
}
impl NodeDataStore {
    pub fn write_data(&mut self, id: NodeId, data: NodeData) {
        self.id_to_data
            .entry(id)
            .and_modify(|d| *d = data.clone())
            .or_insert(data);
    }
}
