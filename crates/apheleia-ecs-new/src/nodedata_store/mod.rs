use std::mem::{replace, swap};

use rustc_hash::FxHashMap;

use crate::{NodeId, types::NodeData};

/// Stores NodeData for every NodeId.
#[derive(Default)]
pub struct NodeDataStore {
    id_to_data: FxHashMap<NodeId, NodeData>,
}
impl NodeDataStore {
    pub fn set_data(&mut self, id: NodeId, data: NodeData) {
        self.id_to_data
            .entry(id)
            .and_modify(|d| *d = data)
            .or_insert(data);
    }

    pub fn get_data(&self, id: NodeId) -> Option<&NodeData> {
        self.id_to_data.get(&id)
    }
}
