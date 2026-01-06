use apheleia_core::types::vector::Vector2;

use crate::node::node::NodeTrait;

pub struct NodeData {
    pub position: Vector2,
    pub size: Option<Vector2>,
}
impl NodeData {
    pub fn new() -> Self {
        NodeData { position: Vector2(0, 0), size: None }
    }
}
