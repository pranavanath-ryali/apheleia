use apheleia_core::types::vector::Vector2;

use crate::node::node::NodeTrait;

pub struct NodeData {
    pub global_positon: Option<Vector2>,
    pub position: Vector2,

    pub size: Option<Vector2>,
}
impl NodeData {
    pub fn new(position: Vector2) -> Self {
        NodeData {
            global_positon: None,
            position,
            size: None,
        }
    }

    pub fn set_global_position(&mut self, position: Vector2) {
        self.global_positon = Some(position);
    }
    pub fn get_global_position(&self) -> &Option<Vector2> {
        &self.global_positon
    }

    pub fn set_size(&mut self, size: Vector2) {
        self.size = Some(size);
    }
    pub fn get_size(&self) -> &Option<Vector2> {
        &self.size
    }
}
