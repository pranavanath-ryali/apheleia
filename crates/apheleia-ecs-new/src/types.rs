use apheleia_core::types::Vec2;

/// A [`NodeData`] for a specific node stores its position and size relative to the parent and the
/// absolute position and size.
/// This is split from extensions because the framework works on atleast a position and
/// size for a given node.
pub struct NodeData {
    position: Vec2,
    size: Vec2,

    global_position: Option<Vec2>,
    global_size: Option<Vec2>,
}
impl Default for NodeData {
    fn default() -> Self {
        Self {
            position: Vec2::zero(),
            size: Vec2::zero(),

            global_position: None,
            global_size: None
        }
    }
}
