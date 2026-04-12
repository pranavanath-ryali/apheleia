use std::collections::HashMap;

use rustc_hash::FxHashMap;

use crate::{
    node::{NodeTrait, data::NodeData},
    types::NodeId,
};

#[derive(Default)]
pub struct NodeStore {
    id_nodes: FxHashMap<NodeId, Box<dyn NodeTrait>>,
    id_data: FxHashMap<NodeId, NodeData>,
    class_id: HashMap<String, NodeId>,
}
impl NodeStore {
    pub fn add_node(
        &mut self,
        id: NodeId,
        class: Option<String>,
        node: Box<dyn NodeTrait>,
        data: NodeData,
    ) {
        self.id_nodes.insert(id, node);
        self.id_data.insert(id, data);
        if let Some(class) = class {
            self.class_id.insert(class, id);
        }
    }

    pub fn get_node_as<T: NodeTrait>(&self, id: NodeId) -> Option<&T> {
        if let Some(node) = self.id_nodes.get(&id) {
            return node.as_any().downcast_ref::<T>();
        }
        None
    }
    pub fn get_node_mut_as<T: NodeTrait>(&mut self, id: NodeId) -> Option<&mut T> {
        if let Some(node) = self.id_nodes.get_mut(&id) {
            return node.as_any_mut().downcast_mut::<T>();
        }
        None
    }

    pub fn get_node(&self, id: NodeId) -> Option<&dyn NodeTrait> {
        if let Some(node) = self.id_nodes.get(&id) {
            return Some(node.as_ref());
        }
        None
    }
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Box<dyn NodeTrait>> {
        if let Some(node) = self.id_nodes.get_mut(&id) {
            return Some(node);
        }
        None
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
}
