use std::any::{Any, TypeId};

use rustc_hash::FxHashMap;

use crate::types::traits::Resource;

#[derive(Default)]
pub struct ResourceStore {
    resources: FxHashMap<TypeId, Box<dyn Any>>,
}
impl ResourceStore {
    pub fn add<T: Resource>(&mut self, res: T) {
        let type_id = TypeId::of::<T>();
        self.resources.entry(type_id).or_insert(Box::new(res));
    }

    pub fn get<T: Resource>(&self) -> Option<&T> {
        let Some(res) = self.resources.get(&TypeId::of::<T>()) else {
            return None;
        };

        res.downcast_ref::<T>()
    }
    pub fn get_mut<T: Resource>(&mut self) -> Option<&mut T> {
        let Some(res) = self.resources.get_mut(&TypeId::of::<T>()) else {
            return None;
        };

        res.downcast_mut::<T>()
    }
}
