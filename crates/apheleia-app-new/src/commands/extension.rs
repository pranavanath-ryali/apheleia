use apheleia_ecs_new::{NodeId, extensions::Extension};

use crate::commands::ContextCommand;

#[derive(Debug)]
pub struct AddExtensionToNode<E: Extension>(pub NodeId, pub E);
impl<E: Extension> AddExtensionToNode<E> {
    pub fn new(id: NodeId, extension: E) -> Box<Self> {
        Box::new(Self(id, extension))
    }
}
impl<E: Extension> ContextCommand for AddExtensionToNode<E> {
    fn execute(self: Box<Self>, app: &mut crate::app::App) {
        app.get_world_mut().add_extension_to_node(self.0, self.1);
    }
}
