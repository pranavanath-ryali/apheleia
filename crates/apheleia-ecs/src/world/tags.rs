//! [`TagRegistry`] related methods implemented into [`World`]

use super::*;

impl World {
    /// Tags a node with the given [`Tag`] and register it to the [`TagRegistry`]
    ///
    /// # Arguments
    ///
    /// * `node` - The [`NodeId`] of the node to be tagged
    /// * `tag` - The Tag that is associated with the node
    ///
    /// # Example
    ///
    /// ```rust
    /// struct MyButtonTag;
    /// impl TagTrait for MyButtonTag {}
    ///
    /// let mut world = World::default();
    /// let button_node = world.create_node();
    ///
    /// world.tag_node(MyButtonTag, button_node);
    /// ```
    #[inline]
    pub fn tag_node<T: TagTrait + 'static>(&mut self, node: NodeId, tag: T) {
        self.tag_registry.tag_node(node, tag);
    }

    /// Returns all nodes associated with the given [`Tag`]
    ///
    /// Returns `None` if no nodes have been tagged with the given [`Tag`]
    ///
    /// # Arguments
    ///
    /// * `tag` - The [`Tag`] to look up
    ///
    /// # Example
    ///
    /// ```rust
    /// struct MyButtonTag;
    /// impl TagTrait for MyButtonTag {}
    ///
    /// if let Some(nodes) = world.get_nodes_tagged::<MyButtonTag>() {
    ///     for id in nodes {
    ///         println!("{}", id);
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn get_nodes_tagged<T: TagTrait + 'static>(&self) -> Option<&SmallVec<[usize; 8]>> {
        self.tag_registry.get_nodes::<T>()
    }
}
