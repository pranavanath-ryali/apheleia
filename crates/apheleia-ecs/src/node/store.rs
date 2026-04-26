use std::collections::HashMap;

use rustc_hash::FxHashMap;

use crate::node::NodeData;
use apheleia_types::{IdGenerator, IdGeneratorTrait, NodeId};

#[derive(Default)]
pub struct NodeDataStore {
    id_generator: IdGenerator<NodeId>,

    id_data: FxHashMap<NodeId, NodeData>,
    class_id: HashMap<String, NodeId>,
}
impl NodeDataStore {
    pub fn create_node(&mut self, class: &str, data: NodeData) -> NodeId {
        let id = self.id_generator.next();

        self.id_data.insert(id, data);
        self.class_id.insert(class.to_string(), id);

        id
    }

    pub fn get_data(&self, id: NodeId) -> Option<&NodeData> {
        self.id_data.get(&id)
    }
    pub fn get_data_mut(&mut self, id: NodeId) -> Option<&mut NodeData> {
        self.id_data.get_mut(&id)
    }

    pub fn get_id_by_class(&self, class: &str) -> Option<&NodeId> {
        self.class_id.get(class)
    }
}
