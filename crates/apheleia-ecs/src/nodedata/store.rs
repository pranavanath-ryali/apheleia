use log::info;
use rustc_hash::FxHashMap;

use crate::{nodedata::data::NodeData, types::NodeId};

/// Stores NodeData for every NodeId.
#[derive(Default)]
pub struct NodeDataStore {
    id_to_data: FxHashMap<NodeId, NodeData>,
}
impl NodeDataStore {
    pub fn set_data(&mut self, id: NodeId, data: NodeData) {
        info!("[ECS] Set data to Node {} - {:#?}", id, data);
        self.id_to_data
            .entry(id)
            .and_modify(|d| *d = data)
            .or_insert(data);
    }

    pub fn get_data(&self, id: NodeId) -> Option<&NodeData> {
        self.id_to_data.get(&id)
    }
    pub fn get_data_mut(&mut self, id: NodeId) -> Option<&mut NodeData> {
        self.id_to_data.get_mut(&id)
    }
}
