use apheleia_core::types::vector::Vector2;

pub struct NodeData {
    pub global_positon: Option<Vector2>,
    pub position: Vector2,

    pub size: Option<Vector2>,

    pub dirty: Dirty
}
impl NodeData {
    pub fn new(position: Vector2) -> Self {
        NodeData {
            global_positon: None,
            position,
            size: None,

            dirty: Dirty::default()
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

#[derive(Clone, Copy)]
pub enum DirtyRenderLevel {
    None,

    SimpleDirty, // Rerender node alone. Leave already defined attributes unless specified
    SubtreeDirty, // Rerender entire subtree which includes the node and including all its children
}
pub struct Dirty {
    pub render: DirtyRenderLevel 
}
impl Default for Dirty {
    fn default() -> Self {
        Dirty { render: DirtyRenderLevel::None }
    }
}
