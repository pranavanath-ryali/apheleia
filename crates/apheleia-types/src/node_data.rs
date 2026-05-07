use crate::vec2::Vec2;

pub struct NodeData {
    pub position: Vec2,
    pub size: Vec2,
}
impl NodeData {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            position,
            size,
        }
    }
}
