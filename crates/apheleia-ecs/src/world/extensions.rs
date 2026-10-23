use crate::{
    types::{NodeId, traits::Extension},
    world::World,
};

impl World {
    pub fn add_extension<T: Extension>(&mut self, id: NodeId, extension: T) {
        self.extension_store.add_extension(id, extension);
    }

    pub fn get_extension<T: Extension>(&self, id: NodeId) -> Option<&T> {
        self.extension_store.get_extension::<T>(id)
    }
    pub fn get_extension_mut<T: Extension>(&mut self, id: NodeId) -> Option<&mut T> {
        self.extension_store.get_extension_mut::<T>(id)
    }
}
