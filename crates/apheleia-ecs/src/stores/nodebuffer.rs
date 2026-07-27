use apheleia_core::{node_buffer::NodeBuffer, types::Vec2};
use log::info;
use rustc_hash::FxHashMap;

use crate::types::NodeId;

/// A [`Resource`] storing a [`NodeBuffer`] for each node that has one.
///
/// Buffers are created lazily and on-demand: a [`NodeBuffer`] is only
/// allocated for a node once valid global position and size information is
/// available, and is skipped (with a log message) if the node's size has a
/// zero dimension.
#[derive(Debug, Default)]
pub struct NodeBufferStore {
    /// Maps each node's [`NodeId`] to its associated [`NodeBuffer`], if one
    /// has been created.
    id_to_buffer: FxHashMap<NodeId, NodeBuffer>,

    first_access_per_tick: FxHashMap<NodeId, bool>,
}

impl NodeBufferStore {
    pub fn clear_first_access(&mut self) {
        self.first_access_per_tick.clear();
    }

    /// Returns a mutable reference to the [`NodeBuffer`] associated with the
    /// given node, if one exists.
    ///
    /// Returns `None` if no buffer has been created for this node yet (e.g.
    /// via [`NodeBufferStore::create_or_get_buffer`]).
    pub fn get_buffer(&mut self, id: NodeId) -> Option<&mut NodeBuffer> {
        self.id_to_buffer.get_mut(&id)
    }

    /// Returns the existing [`NodeBuffer`] for the given node, creating one
    /// first if it doesn't already exist.
    ///
    /// A new buffer is only created if both `global_position` and
    /// `global_size` are provided and `global_size` has non-zero width and
    /// height. If the buffer already exists, it is returned unchanged
    /// regardless of the position/size arguments passed in.
    ///
    /// # Arguments
    ///
    /// * `global_position` - The global position to use if a new buffer needs
    ///   to be created.
    /// * `global_size` - The global size to use if a new buffer needs to be
    ///   created. Must have non-zero `x` and `y` components for creation to
    ///   proceed.
    /// * `id` - The identifier of the node to create or fetch a buffer for.
    pub fn create_or_get_buffer(
        &mut self,
        global_position: Option<Vec2>,
        global_size: Option<Vec2>,
        id: NodeId,
    ) -> Option<&mut NodeBuffer> {
        if let Some(size) = global_size
            && let Some(global_position) = global_position
            && (size.x != 0 && size.y != 0)
        {
            return Some(
                self.id_to_buffer
                    .entry(id)
                    .and_modify(|buf| {
                        if *self
                            .first_access_per_tick
                            .entry(id)
                            .and_modify(|b| {
                                if *b {
                                    *b = false;
                                }
                            })
                            .or_insert(true)
                        {
                            buf.clear();
                        }
                    })
                    .or_insert_with(|| {
                        info!(
                            "[ECS] Creating new NodeBuffer of size: {:?} for node: {}",
                            size, id
                        );
                        NodeBuffer::new(global_position, size)
                    }),
            );
        }
        info!(
            "[ECS] Skipped creating NodeBuffer since one of the dimension for global_size is 0 for NodeID: {}",
            id
        );
        None
    }

    pub fn clear_all_buffers(&mut self) {
        self.id_to_buffer.clear();
        self.first_access_per_tick.clear();
    }
}
