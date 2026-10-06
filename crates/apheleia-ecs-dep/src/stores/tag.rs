use std::{any::TypeId, fmt::Debug};

use log::info;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::{traits::tag::TagTrait, types::NodeId};

/// A registry mapping tag types to the nodes tagged with them.
///
/// Tags are lightweight, type-based markers (identified purely by [`TypeId`],
/// with no data stored per-tag) used to categorize nodes for later lookup
#[derive(Default)]
pub struct TagRegistry {
    /// Maps each tag's [`TypeId`] to the list of nodes tagged with it.
    map: FxHashMap<TypeId, SmallVec<[NodeId; 8]>>,
}

impl Debug for TagRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TagRegistry")
            .finish()
    }
}

impl TagRegistry {
    /// Tags `node` with the given tag value's type.
    ///
    /// Only the type of `tag` is recorded (via [`TypeId::of`]); the value
    /// itself is not stored, and is only used here for logging.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to tag.
    /// * `tag` - A value whose type determines the tag being applied.
    pub fn tag_node<T: TagTrait>(&mut self, node: NodeId, tag: T) {
        let tag_type = TypeId::of::<T>();
        self.map
            .entry(tag_type)
            .and_modify(|v| {
                if !v.contains(&node) {
                    v.push(node);
                }
            })
            .or_insert({
                let mut v = SmallVec::<[NodeId; _]>::new();
                v.push(node);
                v
            });
        info!("[ECS] Tagged NodeId: {} with Tag: {:#?}", node, tag);
    }

    /// Tags `node` with the tag identified by the given [`TypeId`].
    ///
    /// Equivalent to [`TagRegistry::tag_node`], but for use when only a
    /// type-erased [`TypeId`] is available rather than a concrete tag value.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to tag.
    /// * `tag` - The [`TypeId`] of the tag to apply.
    pub fn tag_node_by_id(&mut self, node: NodeId, tag: TypeId) {
        self.map
            .entry(tag)
            .and_modify(|v| {
                if !v.contains(&node) {
                    v.push(node);
                }
            })
            .or_insert({
                let mut v = SmallVec::<[NodeId; _]>::new();
                v.push(node);
                v
            });
        info!("[ECS] Tagged NodeId: {} with TagType: {:#?}", node, tag);
    }

    /// Returns all nodes tagged with type `T`, if any have been tagged.
    ///
    /// Returns `None` if no node has ever been tagged with this type (as
    /// opposed to returning an empty slice).
    pub fn get_nodes_with_tag<T: TagTrait>(&self) -> Option<&SmallVec<[NodeId; 8]>> {
        let tag = TypeId::of::<T>();
        self.map.get(&tag)
    }
}
