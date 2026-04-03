use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::resources::traits::Resource;

#[derive(Default)]
pub struct ResourceStore {
    resources_storage: HashMap<TypeId, Box<dyn Any>>,
}
impl ResourceStore {
    pub fn add_resource<T: Resource>(&mut self, res: Box<T>) {
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
