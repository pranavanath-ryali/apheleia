use crate::{
    types::{NodeId, traits::Extension},
    world::World,
};

impl World {
    #[inline]
    pub fn register_extension<T: Extension>(&mut self) {
        self.extension_store.register_extension::<T>();
    }

    #[inline]
    pub fn add_extension<T: Extension>(&mut self, id: NodeId, extension: T) {
        self.extension_store.add_extension(id, extension);
    }

    #[inline]
    pub fn get_extension<T: Extension>(&self, id: NodeId) -> Option<&T> {
        self.extension_store.get_extension::<T>(id)
    }
    #[inline]
    pub fn get_extension_mut<T: Extension>(&mut self, id: NodeId) -> Option<&mut T> {
        self.extension_store.get_extension_mut::<T>(id)
    }

    #[inline]
    pub fn remove_extension<T: Extension>(&mut self, id: NodeId) {
        self.extension_store.remove_extension::<T>(id);
    }
}
