pub(crate) mod store;

use apheleia_types::Vec2;

#[derive(Clone, Copy)]
pub struct NodeData {
    pub global_positon: Option<Vec2>,
    pub position: Vec2,
    pub size: Option<Vec2>,
    pub global_size: Option<Vec2>,
}
impl Default for NodeData {
    fn default() -> Self {
        NodeData {
            global_positon: None,
            position: Vec2::zero(),
            size: None,
            global_size: None,
        }
    }
}
impl NodeData {
    pub fn new(position: Vec2, size: Option<Vec2>) -> Self {
        NodeData {
            global_positon: None,
            position,
            size,
            global_size: None,
        }
    }

    pub fn set_global_position(&mut self, position: Vec2) {
        self.global_positon = Some(position);
    }
    pub fn get_global_position(&self) -> Option<Vec2> {
        self.global_positon
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.size = Some(size);
    }
    pub fn get_size(&self) -> Option<Vec2> {
        self.size
    }

    pub fn get_global_size(&self) -> Option<Vec2> {
        self.global_size
    }
    pub fn set_global_size(&mut self, size: Option<Vec2>) {
        self.global_size = size;
    }
}
