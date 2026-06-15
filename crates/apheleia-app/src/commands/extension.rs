use apheleia_ecs::{
    NodeId,
    command::ContextCommand,
    extensions::Extension,
    world::{self, World},
};

#[derive(Debug)]
pub struct AddExtensionToNode<E: Extension>(pub NodeId, pub E);
impl<E: Extension> AddExtensionToNode<E> {
    pub fn new(id: NodeId, extension: E) -> Box<Self> {
        Box::new(Self(id, extension))
    }
}
impl<E: Extension> ContextCommand for AddExtensionToNode<E> {
    fn execute(self: Box<Self>, world: &mut World) {
        world.add_extension_to_node(self.0, self.1);
    }
}
