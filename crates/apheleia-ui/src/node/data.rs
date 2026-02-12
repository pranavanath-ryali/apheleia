use apheleia_core::types::vector::Vector2;

use crate::types::{Layout, TBLRValues};

pub struct NodeData {
    pub global_positon: Option<Vector2>,
    pub position: Vector2,
    pub size: Option<Vector2>,

    pub layout: Option<Layout>,

    pub dirty: Dirty,
}
impl Default for NodeData {
    fn default() -> Self {
        NodeData {
            global_positon: None,
            position: Vector2(0, 0),
            size: None,
            layout: None,
            dirty: Dirty::default(),
        }
    }
}
impl NodeData {
    pub fn new(position: Vector2, size: Option<Vector2>) -> Self {
        NodeData {
            global_positon: None,
            position,
            size,

            layout: None,

            dirty: Dirty::default(),
        }
    }

    pub fn set_global_position(&mut self, position: Vector2) {
        self.global_positon = Some(position);
    }
    pub fn get_global_position(&self) -> &Option<Vector2> {
        &self.global_positon
    }

    pub fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }
    pub fn get_position(&self) -> &Vector2 {
        &self.position
    }

    pub fn set_size(&mut self, size: Vector2) {
        self.size = Some(size);
    }
    pub fn get_size(&self) -> &Option<Vector2> {
        &self.size
    }
}

#[derive(Clone, Copy)]
pub enum DirtyRenderLevel {
    SimpleDirty, // Rerender node alone. Leave already defined attributes unless specified
    SubtreeDirty, // Rerender entire subtree which includes the node and including all its children
}
