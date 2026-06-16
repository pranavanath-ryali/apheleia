//! [`EventTracker`] related methods implemented for [`World`]

use super::*;

impl World {
    // ==================[EVENT_TRACKER FUNCTIONS]==================
    /// Add a local event with the given node, registring it in the [`EventTracker`]
    ///
    /// # Arguments
    ///
    /// * `node` - The [`NodeId`] of the node to associate the local event with
    /// * `event` - The [`EventId`] of the event to register
    ///
    /// # Example
    ///
    /// ```rust
    /// ```
    #[inline]
    pub fn add_local_event(&mut self, node: NodeId, event: EventId) {
        self.event_tracker.add_local_event(node, event);
    }
    #[inline]
    pub fn is_local_event(&self, node_id: NodeId, event_id: EventId) -> bool {
        self.event_tracker.is_local_event(node_id, event_id)
    }
    #[inline]
    pub fn clear_local_events(&mut self) {
        self.event_tracker.clear_local_events();
    }
    #[inline]
    pub fn get_nodes_with_event(
        &mut self,
        event_id: EventId,
    ) -> Option<&mut indexmap::IndexSet<usize>> {
        self.event_tracker.get_nodes_with_event(event_id)
    }
    #[inline]
    pub fn add_global_event(&mut self, tag: Tag, event_id: EventId) {
        self.event_tracker.add_global_event(tag, event_id);
    }
    #[inline]
    pub fn is_global_event(&self, tag: Tag, event_id: EventId) -> bool {
        self.event_tracker.is_global_event(tag, event_id)
    }
    #[inline]
    pub fn clear_global_events(&mut self) {
        self.event_tracker.clear_global_events();
    }
}
