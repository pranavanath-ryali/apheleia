use std::collections::HashMap;

use crate::{
    node::{data::NodeData, node::NodeTrait},
    types::NodeId,
};

#[derive(Default)]
pub struct NodeStorage {
    id_nodes: HashMap<NodeId, Box<dyn NodeTrait>>,
    id_data: HashMap<NodeId, NodeData>,
    class_id: HashMap<String, NodeId>,
}
impl NodeStorage {
    pub fn add_node(&mut self, id: NodeId, class: &str, node: Box<dyn NodeTrait>, data: NodeData) {
        self.id_nodes.insert(id, node);
        self.id_data.insert(id, data);
        self.class_id.insert(class.to_string(), id);
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Box<dyn NodeTrait>> {
        self.id_nodes.get(&id)
    }
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Box<dyn NodeTrait>> {
        self.id_nodes.get_mut(&id)
    }

    pub fn get_data(&self, id: NodeId) -> Option<&NodeData> {
        self.id_data.get(&id)
    }
    pub fn get_data_mut(&mut self, id: NodeId) -> Option<&mut NodeData> {
        self.id_data.get_mut(&id)
    }

    pub fn get_id(&self, class: &str) -> Option<&NodeId> {
        self.class_id.get(class)
    }

    pub fn iter_id_data(&self) -> std::collections::hash_map::Iter<'_, usize, NodeData> {
        self.id_data.iter()
    }
    pub fn iter_id_data_mut(&mut self) -> std::collections::hash_map::IterMut<'_, usize, NodeData> {
        self.id_data.iter_mut()
    }

    pub fn iter_id_node(
        &self,
    ) -> std::collections::hash_map::Iter<'_, usize, Box<dyn NodeTrait + 'static>> {
        self.id_nodes.iter()
    }
    pub fn iter_id_node_mut(
        &mut self,
    ) -> std::collections::hash_map::IterMut<'_, usize, Box<dyn NodeTrait + 'static>> {
        self.id_nodes.iter_mut()
    }
}
