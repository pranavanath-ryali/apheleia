//! [`EventTracker`] related methods implemented for [`World`]

use crate::events::EventTrait;

use super::*;

impl World {
    // ==================[EVENT_TRACKER FUNCTIONS]==================
    /// Add a local event with the given node, registring it in the [`EventTracker`]
    ///
    /// # Arguments
    ///
    /// * `node` - The [`NodeId`] of the node to associate the local event with
    /// * `event` - The event to register
    ///
    /// # Example
    ///
    /// ```rust
    /// ```
    #[inline]
    pub fn add_local_event<E: EventTrait>(&mut self, node: NodeId, event: E) {
        self.event_tracker.add_local_event(node, event);
    }
    #[inline]
    pub fn is_local_event<E: EventTrait>(&self, node_id: NodeId, event: E) -> bool {
        self.event_tracker.is_local_event(node_id, event)
    }
    #[inline]
    pub fn clear_local_events(&mut self) {
        self.event_tracker.clear_local_events();
    }
    #[inline]
    pub fn get_nodes_with_event<E: EventTrait>(
        &mut self,
        event: E,
    ) -> Option<&mut indexmap::IndexSet<usize>> {
        self.event_tracker.get_nodes_with_event::<E>()
    }

    #[inline]
    pub fn add_global_event<E: EventTrait>(&mut self, tag: Tag, event: E) {
        self.event_tracker.add_global_event(tag, event);
    }
    #[inline]
    pub fn is_global_event<E: EventTrait>(&self, tag: Tag, event: E) -> bool {
        self.event_tracker.is_global_event(tag, event)
    }
    #[inline]
    pub fn clear_global_events(&mut self) {
        self.event_tracker.clear_global_events();
    }
}
