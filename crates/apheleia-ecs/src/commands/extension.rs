use crate::{commands::WorldCommand, types::{NodeId, traits::Extension}};

pub struct AddExtension<T: Extension>(pub NodeId, pub T);
impl<T: Extension> AddExtension<T> {
    pub fn new(id: NodeId, ext: T) -> Box<Self> {
        Box::new(Self(id, ext))
    }
}
impl<T: Extension> WorldCommand for AddExtension<T> {
    fn execute(self: Box<Self>, world: &mut crate::world::World) {
        world.add_extension(self.0, self.1);
    }
}
