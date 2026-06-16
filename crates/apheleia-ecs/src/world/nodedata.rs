use super::*;

impl World {
    /// Generate a unique [`NodeId`], register it and return for future use
    pub fn create_node(&mut self) -> NodeId {
        let id = self.nodeid_gen.next();
        self.registered_nodes.push_back(id);
        id
    }

    /// Get all [`NodeId`]s currently alive and registered
    #[inline]
    pub fn get_registered_nodes(&self) -> &VecDeque<NodeId> {
        &self.registered_nodes
    }

    /// Set [`NodeData`] for given [`NodeId`]
    ///
    /// # Arguments
    ///
    /// * `id` - The [`NodeId`] for the node
    /// * `data` - The [`NodeData`] itself to associate with the node
    ///
    /// # Example
    ///
    /// ```rust
    /// let node = world.create_node();
    /// world.set_data(node, NodeData::default());
    /// ```
    #[inline]
    pub fn set_data(&mut self, id: NodeId, data: NodeData) {
        self.nodedata_store.set_data(id, data);
    }

    /// Get reference to [`NodeData`] for given [`NodeId`]
    ///
    /// # Arguments
    ///
    /// * `id` - The [`NodeId`] to look for
    #[inline]
    pub fn get_nodedata(&self, id: NodeId) -> Option<&NodeData> {
        self.nodedata_store.get_data(id)
    }

    /// Get mutable reference to [`NodeData`] for given [`NodeId`]
    ///
    /// # Arguments
    ///
    /// * `id` - The [`NodeId`] to look for
    #[inline]
    pub fn get_nodedata_mut(&mut self, id: NodeId) -> Option<&mut NodeData> {
        self.nodedata_store.get_data_mut(id)
    }
}
