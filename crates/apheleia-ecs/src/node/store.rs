use apheleia_types::{
    NodeId,
    id_generator::{IdGenerator, IdGeneratorTrait},
};
use rustc_hash::FxHashMap;

use crate::node::NodeData;

#[derive(Default)]
pub struct NodeDataStore {
    id_generator: IdGenerator<NodeId>,

    id_data: FxHashMap<NodeId, NodeData>,
}
impl NodeDataStore {
    pub fn create_node(&mut self, data: NodeData) -> NodeId {
        let id = self.id_generator.next();
        self.id_data.insert(id, data);
        id
    }

    pub fn get_data(&self, id: NodeId) -> Option<&NodeData> {
        self.id_data.get(&id)
    }
    pub fn get_data_mut(&mut self, id: NodeId) -> Option<&mut NodeData> {
        self.id_data.get_mut(&id)
    }
}
