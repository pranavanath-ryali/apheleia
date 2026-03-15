use std::collections::HashMap;

use crate::{
    NodeId,
    node::{data::NodeData, node::NodeTrait},
};

pub struct NodeStorage {
    id_nodes: HashMap<NodeId, Box<dyn NodeTrait>>,
    id_data: HashMap<NodeId, NodeData>,
    class_id: HashMap<String, NodeId>,
}
