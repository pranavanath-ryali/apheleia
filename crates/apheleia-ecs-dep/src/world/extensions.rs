use crate::{traits::extension::Extension, types::NodeId, world::World};

impl World {
    /// Add an extension and bind it to the given [`NodeId`]
    ///
    /// # Arguments
    ///
    /// * `node` - The [`NodeId`] of the node to add the [`Extension`] to
    /// * `extension` - The [`Extension`] itself that should be added and stored in [`World`]
    ///
    /// # Example
    ///
    /// ```rust
    /// #[derive(Debug)]
    /// pub struct MyExtension {
    ///     value: i32
    /// }
    /// impl Extension for MyExtension {}
    ///
    /// let node = world.create_node();
    ///
    /// world.add_extension_to_node(node, MyExtension { value: 123 });
    /// ```
    #[inline]
    pub fn add_extension_to_node<E: Extension>(&mut self, node: NodeId, extension: E) {
        self.extension_store
            .add_extension_to_node(node, extension);
    }

    /// Get reference to [`Extension`] that is associated to given node
    ///
    /// # Arguments
    ///
    /// * `node` - The [`NodeId`] for the node
    ///
    /// # Example
    ///
    /// ```rust
    /// #[derive(Debug)]
    /// pub struct MyExtension {
    ///     value: i32
    /// }
    /// impl Extension for MyExtension {}
    ///
    /// let node = world.create_node();
    ///
    /// world.add_extension_to_node(node, MyExtension { value: 123 });
    ///
    /// assert_eq!(world.get_extension::<MyExtension>(node).unwrap().value, 123);
    /// ```
    #[inline]
    pub fn get_extension<E: Extension>(&self, node: NodeId) -> Option<&E> {
        self.extension_store.get_extension::<E>(node)
    }
    /// Get mutable reference to [`Extension`] that is associated to given node
    ///
    /// # Arguments
    ///
    /// * `node` - The [`NodeId`] for the node
    ///
    /// # Example
    ///
    /// ```rust
    /// #[derive(Debug)]
    /// pub struct MyExtension {
    ///     value: i32
    /// }
    /// impl Extension for MyExtension {}
    ///
    /// let node = world.create_node();
    ///
    /// world.add_extension_to_node(node, MyExtension { value: 123 });
    ///
    /// assert_eq!(world.get_extension_mut::<MyExtension>(node).unwrap().value, 123);
    /// ```
    #[inline]
    pub fn get_extension_mut<E: Extension>(&mut self, node: NodeId) -> Option<&mut E> {
        self.extension_store.get_extension_mut::<E>(node)
    }

    /// Get all [`NodeId`]s with [`Extension`]
    ///
    /// # Example
    ///
    /// ```rust
    /// #[derive(Debug)]
    /// pub struct MyExtension {
    ///     value: i32
    /// }
    /// impl Extension for MyExtension {}
    ///
    /// let node = world.create_node();
    ///
    /// world.add_extension_to_node(node, MyExtension { value: 123 });
    ///
    /// for id in world.get_nodes_with_extension::<MyExtension>() {
    ///     println!("NodeId: {}", id);
    /// }
    /// ```
    #[inline]
    pub fn get_nodes_with_extension<E: Extension>(&self) -> Vec<NodeId> {
        self.extension_store.get_nodes_with_extension::<E>()
    }

}
