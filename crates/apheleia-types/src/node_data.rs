use std::fmt;

use crate::vec2::Vec2;

/// The minimum data needed for a node.
/// [`NodeData`] is separated from generic [`Extension`] because the framework is designed around
/// position and size (more specifically, global_position, and global_size) as the minimum for a Node
/// # Examples
/// ```rust
/// use crate::vec2::Vec2;
/// use crate::node_data::NodeData;
///
/// let data = NodeData { position: Vec2 { x: 10, y: 5 }, size: Vec2 { x: 0, y: 0 } };
///
/// assert_eq!(data.position, Vec2 { x: 10, y: 5 });
/// assert_eq!(data.size, Vec2 { x: 0, y: 0 });
/// ```
#[derive(fmt::Debug, PartialEq)]
pub struct NodeData {
    pub position: Vec2,
    pub size: Vec2,
}
impl NodeData {
    /// Creates a new [`NodeData`] with given position and size
    /// # Arguments
    ///
    /// * `position`: A [`Vec2`] representing the position of the node in 2D space. Can be zero.
    /// The engine precomputes `global_position` from `position` and uses that at runtime.
    /// * `size`: A [`Vec2`] representing the node's self defined size. Can be zero. The engine
    /// precomputes `global_size` from `size` and uses that at runtime. The engine skips the
    /// render process for the node if any one dimension of `global_size` is 0.
    ///
    /// # Examples
    /// ```rust
    /// let node = NodeData::new(Vec2 { x: 10, y: 5 }, Vec2 { x: 5, y: 3 });
    /// ```
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self { position, size }
    }
}
