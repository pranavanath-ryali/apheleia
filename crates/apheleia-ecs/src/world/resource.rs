use crate::{types::traits::Resource, world::World};

impl World {
    #[inline]
    pub fn resource_insert<T: Resource>(&mut self, res: T) {
        self.resource_store.add(res);
    }

    #[inline]
    pub fn resource_get<T: Resource>(&self) {
        self.resource_store.get::<T>();
    }

    #[inline]
    pub fn resource_get_mut<T: Resource>(&mut self) {
        self.resource_store.get_mut::<T>();
    }
}
