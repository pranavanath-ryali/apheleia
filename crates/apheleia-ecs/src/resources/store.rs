use std::any::{Any, TypeId};

use apheleia_types::Resource;
use rustc_hash::FxHashMap;

#[derive(Default)]
pub struct ResourceStore {
    resources_storage: FxHashMap<TypeId, Box<dyn Any>>,
}
impl ResourceStore {
    pub fn add_resource<T: Resource>(&mut self, res: Box<T>) {
        assert!(self.resources_storage.contains_key(&TypeId::of::<T>()), "The given resource is already added");
        self.resources_storage
            .entry(TypeId::of::<T>())
            .or_insert(res);
    }

    pub fn get_resource<T: Resource>(&self) -> Option<&T> {
        self.resources_storage
            .get(&TypeId::of::<T>())
            .unwrap()
            .downcast_ref::<T>()
    }
    pub fn get_resource_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.resources_storage
            .get_mut(&TypeId::of::<T>())
            .unwrap()
            .downcast_mut::<T>()
    }
}
