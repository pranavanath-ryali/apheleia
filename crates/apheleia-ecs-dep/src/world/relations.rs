use log::info;
use tree_ds::prelude::{Node, NodeRemovalStrategy, Tree};

use crate::{types::NodeId, world::World};

impl World {
    /// Returns a mutable reference to the underlying node relationship tree.
    ///
    /// Provides direct access to the [`Tree<NodeId, NodeId>`] backing the
    /// world's parent-child structure
    #[inline]
    pub fn get_relations_mut(&mut self) -> &mut Tree<NodeId, NodeId> {
        &mut self.relations
    }

    /// Returns a shared reference to the underlying node relationship tree.
    #[inline]
    pub fn get_relations(&self) -> &Tree<NodeId, NodeId> {
        &self.relations
    }

    /// Relates `child` to `parent` in the node hierarchy, making `parent` the
    /// new parent of `child`.
    ///
    /// If `child` does not yet exist in the relationship tree, it is inserted
    /// fresh under `parent`. If `child` already exists (and thus may already
    /// have its own subtree of descendants), the entire subtree rooted at
    /// `child` is detached from its current location and reattached under
    /// `parent`, preserving all of `child`'s descendants.
    ///
    /// # Arguments
    ///
    /// * `child` - The [`NodeId`] of the child node.
    /// * `parent` - The [`NodeId`] of the parent node. Must already exist in
    ///   the relationship tree.
    ///
    /// # Panics
    ///
    /// Panics if `parent` does not exist in the relationship tree, or if the
    /// underlying tree operations (fetching the subtree, removing `child`, or
    /// reattaching the subtree) fail.
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
