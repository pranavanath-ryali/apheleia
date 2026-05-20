use rustc_hash::FxHashMap;

use crate::{NodeId, types::NodeData};

/// Stores NodeData for every NodeId.
#[derive(Default)]
pub struct NodeDataStore {
    id_to_data: FxHashMap<NodeId, NodeData>,
}
