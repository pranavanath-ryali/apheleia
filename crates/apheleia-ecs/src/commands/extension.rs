use crate::{
    traits::{context_command::ContextCommand, extension::Extension}, types::NodeId, world::World,
};

/// A [`ContextCommand`] that attaches an [`Extension`] of type `E` to a specific node.
///
/// Wraps a [`NodeId`] and an extension value, applying the extension to the
/// corresponding node in the [`World`] when executed.
#[derive(Debug)]
pub struct AddExtensionToNode<E: Extension>(pub NodeId, pub E);

impl<E: Extension> AddExtensionToNode<E> {
    /// Creates a new boxed [`AddExtensionToNode`] command for the given node and extension.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the node to which the extension will be added.
    /// * `extension` - The extension instance to attach to the node.
    pub fn new(id: NodeId, extension: E) -> Box<Self> {
        Box::new(Self(id, extension))
    }
}

impl<E: Extension> ContextCommand for AddExtensionToNode<E> {
    /// Executes the command, adding the extension to the specified node in the [`World`].
    fn execute(self: Box<Self>, world: &mut World) {
        world.add_extension_to_node(self.0, self.1);
    }
}
