use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub trait Resource: Any {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Default)]
pub struct ResourceStore {
    extensions_storage: HashMap<TypeId, Box<dyn Any>>,
}
impl ResourceStore {
    pub fn add_resource<T: Resource>(&mut self, extension: Box<T>) {
        self.extensions_storage
            .entry(TypeId::of::<T>())
            .or_insert(extension);
    }

    pub fn get_resource<T: Resource>(&self) -> Option<&T> {
        self.extensions_storage
            .get(&TypeId::of::<T>())
            .unwrap()
            .downcast_ref::<T>()
    }
    pub fn get_resource_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.extensions_storage
            .get_mut(&TypeId::of::<T>())
            .unwrap()
            .downcast_mut::<T>()
    }
}
