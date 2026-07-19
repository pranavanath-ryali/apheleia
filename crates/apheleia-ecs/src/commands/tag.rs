use std::any::TypeId;

use crate::{stores::tag::TagRegistry, traits::context_command::ContextCommand, types::NodeId};

/// A [`ContextCommand`] that tags a node with a given [`TypeId`].
///
/// Wraps a [`NodeId`] and a [`TypeId`], associating the tag with the node
/// in the [`World`] when executed. This is typically used to mark nodes as
/// belonging to a particular type-based category for later lookup.
#[derive(Debug)]
pub struct TagNode(pub NodeId, pub TypeId);

impl TagNode {
    /// Creates a new boxed [`TagNode`] command for the given node and tag.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the node to tag.
    /// * `tag` - The [`TypeId`] to associate with the node.
    pub fn new(id: NodeId, tag: TypeId) -> Box<Self> {
        Box::new(TagNode(id, tag))
    }
}

impl ContextCommand for TagNode {
    /// Executes the command, tagging the specified node with the given
    /// [`TypeId`] in the [`World`].
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        world.get_resource_mut::<TagRegistry>().unwrap().tag_node_by_id(self.0, self.1);
    }
}
