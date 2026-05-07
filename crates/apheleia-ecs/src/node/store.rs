use apheleia_types::{
    MAX_NODES, NodeId, id_generator::{IdGenerator, IdGeneratorTrait}, node_data::NodeData, vec2::Vec2
};
use sparseset::SparseSet;

pub struct NodeDataStore {
    id_generator: IdGenerator<NodeId>,

    positions: SparseSet<Vec2>,
    sizes: SparseSet<Vec2>,
    global_positions: SparseSet<Option<Vec2>>,
    global_sizes: SparseSet<Option<Vec2>>,
}
impl NodeDataStore {
    pub fn new() -> Self {
        Self {
            id_generator: Default::default(),

            positions: SparseSet::with_capacity(MAX_NODES),
            sizes: SparseSet::with_capacity(MAX_NODES),
            global_positions: SparseSet::with_capacity(MAX_NODES),
            global_sizes: SparseSet::with_capacity(MAX_NODES),
        }
    }

    pub fn create_node(&mut self, data: &NodeData) -> NodeId {
        let id = self.id_generator.next();

        self.positions.insert(id, data.position);
        self.sizes.insert(id, data.size);
        self.global_positions.insert(id, None);
        self.global_sizes.insert(id, None);

        id
    }

    pub fn get_position(&self, id: NodeId) -> &Vec2 {
        self.positions.get(id).unwrap()
    }
    pub fn set_position(&mut self, id: NodeId, position: Vec2) {
        *self.positions.get_mut(id).unwrap() = position;
    }

    pub fn get_size(&self, id: NodeId) -> &Vec2 {
        self.sizes.get(id).unwrap()
    }
    pub fn set_size(&mut self, id: NodeId, size: Vec2) {
        *self.sizes.get_mut(id).unwrap() = size;
    }

    pub fn get_global_position(&self, id: NodeId) -> &Option<Vec2> {
        self.global_positions.get(id).unwrap()
    }
    pub fn set_global_position(&mut self, id: NodeId, global_position: Vec2) {
        *self.global_positions.get_mut(id).unwrap() = Some(global_position);
    }

    pub fn get_global_size(&self, id: NodeId) -> &Option<Vec2> {
        self.global_sizes.get(id).unwrap()
    }
    pub fn set_global_size_mut(&mut self, id: NodeId, global_size: Vec2) {
        *self.global_sizes.get_mut(id).unwrap() = Some(global_size);
    }
}
