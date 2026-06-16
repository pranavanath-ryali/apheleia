use super::*;

impl World {
    /// Get mutable reference to current relations of type [`Tree<NodeId, NodeId>`]
    #[inline]
    pub fn get_relations_mut(&mut self) -> &mut Tree<NodeId, NodeId> {
        &mut self.relations
    }

    /// Get reference to current relations of type [`Tree`]
    #[inline]
    pub fn get_relations(&self) -> &Tree<NodeId, NodeId> {
        &self.relations
    }

    /// Add a relation between a child node and parent node from their respective [`NodeId`]s
    ///
    /// # Arguments
    ///
    /// * `child` - The [`NodeId`] of the child node
    /// * `parent` - The [`NodeId`] of the parent node
    ///
    /// # Example
    ///
    /// ```rust
    /// let parent = world.create_node();
    /// let child = world.create_node();
    ///
    /// world.relate_node_with_parent(child, parent);
    /// ```
    pub fn relate_node_with_parent(&mut self, child: NodeId, parent: NodeId) {
        assert!(self.relations.get_node_by_id(&parent).is_some());

        if self.relations.get_node_by_id(&child).is_none() {
            self.relations
                .add_node(Node::new(child, None), Some(&parent))
                .unwrap();

            info!(
                "[ECS] Child NodeID: {} related with Parent NodeID: {}",
                child, parent
            );
            return;
        }

        // Retain children, and move the subtree along with the child if it has any
        assert!(self.relations.get_node_by_id(&child).is_some());
        let subtree = self
            .relations
            .get_subtree(&child, None)
            .expect("Couldn't get subtree");
        self.relations
            .remove_node(&child, NodeRemovalStrategy::RemoveNodeAndChildren)
            .expect("Couldn't remove node from relations");
        self.relations
            .add_subtree(&parent, subtree)
            .expect("Couldn't add subtree to relations");
        info!(
            "[ECS] Moved Child NodeID and all its children: {} to parent NodeID: {}",
            child, parent
        );
    }
}
