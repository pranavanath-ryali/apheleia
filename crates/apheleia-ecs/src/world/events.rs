//! [`EventTracker`] related methods implemented for [`World`]

use super::*;

impl World {
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
    /// let node = world.create_node();
    ///
    /// world.add_event(node, RenderDirty);
    /// ```
    #[inline]
    pub fn add_event<E: EventTrait>(&mut self, node: NodeId, event: E) {
        self.event_tracker.add_local_event(node, event);
    }

    /// Check if a node is marked for a certain `event`
    ///
    /// # Arguments
    ///
    /// * `node` - The [`NodeId`] of the node
    /// * `event` - The event to check if `node` is marked for
    ///
    /// # Example
    ///
    /// ```rust
    /// let node = world.create_node();
    ///
    /// world.add_event(node, RenderDirty);
    ///
    /// assert!(world.is_event(node, RenderDirty));
    /// ```
    #[inline]
    pub fn is_event<E: EventTrait>(&self, node: NodeId, event: E) -> bool {
        self.event_tracker.is_local_event(node, event)
    }

    /// Clear all local events currently tracked in [`EventTracker`]
    #[inline]
    pub fn clear_events(&mut self) {
        self.event_tracker.clear_local_events();
    }

    /// Get all [`NodeId`]s that are marked for a certain `event`
    ///
    /// # Example
    ///
    /// ```rust
    /// for id in world.get_nodes_with_event::<RenderDirty>() {
    ///     println!("NodeId: {} is marked RenderDirty", id);
    /// }
    /// ```
    #[inline]
    pub fn get_nodes_with_event<E: EventTrait>(
        &mut self,
    ) -> Option<&mut IndexSet<usize>> {
        self.event_tracker.get_nodes_with_event::<E>()
    }

    /// Add a global event with a given `tag`, registring it in the [`EventTracker`]
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag that is marked with the given event
    /// * `event` - The event to register
    ///
    /// # Example
    ///
    /// ```rust
    /// #[derive(Debug)]
    /// pub struct MyButtonTag;
    /// impl TagTrait for MyButtonTag {}
    ///
    /// world.add_global_event(MyButtonTag, RenderDirty);
    /// ```
    #[inline]
    pub fn add_global_event<T: TagTrait, E: EventTrait>(&mut self, tag: T, event: E) {
        self.event_tracker.add_global_event(tag, event);
    }

    /// Check if a `tag` is marked for a certain `event`
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag to check for
    /// * `event` - The event to check if `tag` is marked for
    ///
    /// # Example
    ///
    /// ```rust
    /// #[derive(Debug)]
    /// pub struct MyButtonTag;
    /// impl TagTrait for MyButtonTag {}
    ///
    /// world.add_global_event(MyButtonTag, RenderDirty);
    ///
    /// assert!(world.is_global_event(MyButtonTag, RenderDirty));
    /// ```
    #[inline]
    pub fn is_global_event<T: TagTrait, E: EventTrait>(&self, tag: T, event: E) -> bool {
        self.event_tracker.is_global_event(tag, event)
    }

    /// Clear all global events currently tracked in [`EventTracker`]
    #[inline]
    pub fn clear_global_events(&mut self) {
        self.event_tracker.clear_global_events();
    }
}
