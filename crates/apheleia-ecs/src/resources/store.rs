use std::any::{Any, TypeId};

use log::info;
use rustc_hash::FxHashMap;

use crate::resources::Resource;

#[derive(Default)]
pub struct ResourceStore {
    resources: FxHashMap<TypeId, Box<dyn Any>>,
}
impl ResourceStore {
    pub fn add_resource<T: Resource>(&mut self, res: Box<T>) {
        assert!(
            !self.resources.contains_key(&TypeId::of::<T>()),
            "The given resource is already added"
        );
        info!("[ECS] Added resource: {:#?}", res);
        self.resources.entry(TypeId::of::<T>()).or_insert(res);
    }

    pub fn get_resource<T: Resource>(&self) -> Option<&T> {
        if let Some(resource) = self.resources.get(&TypeId::of::<T>()) {
            return Some(
                resource
                    .downcast_ref::<T>()
                    .expect("Couldn't downcast Any to resource T"),
            );
        }
        None
    }
    pub fn get_resource_mut<T: Resource>(&mut self) -> Option<&mut T> {
        if let Some(resource) = self.resources.get_mut(&TypeId::of::<T>()) {
            return Some(
                resource
                    .downcast_mut::<T>()
                    .expect("Couldn't downcast Any to resource T"),
            );
        }
        None
    }
}
