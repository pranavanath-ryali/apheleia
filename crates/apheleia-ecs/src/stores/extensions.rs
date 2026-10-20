use std::any::Any;

use rustc_hash::FxHashMap;
use sparseset::SparseSet;

use crate::types::{NodeId, traits::Extension};

trait ExtensionContainer<T: Extension>: Any {
    fn insert(&mut self, id: NodeId, extension: T);
    fn remove(&mut self, id: NodeId);
    fn get(&self, id: NodeId) -> Option<&T>;
    fn get_mut(&mut self, id: NodeId) -> Option<&mut T>;
}

struct ExtensionPool<T: Extension> {
    extensions: SparseSet<T>
}
impl<T: Extension> Default for ExtensionPool<T> {
    fn default() -> Self {
        Self { extensions: SparseSet::with_capacity(NodeId::MAX as usize) }
    }
}

impl<T: Extension> ExtensionContainer<T> for ExtensionPool<T> {
    fn insert(&mut self, id: NodeId, extension: T) {
        self.extensions.insert(id as usize, extension);
    }
    fn remove(&mut self, id: NodeId) {
        self.extensions.remove(id as usize);
    }

    fn get(&self, id: NodeId) -> Option<&T> {
        self.extensions.get(id as usize)
    }
    fn get_mut(&mut self, id: NodeId) -> Option<&mut T> {
        self.extensions.get_mut(id as usize)
    }
}

pub(crate) struct ExtensionStore {
    pools: FxHashMap<usize, Box<dyn Any>>
}
impl Default for ExtensionStore {
    fn default() -> Self {
        Self { pools: Default::default() }
    }
}
impl ExtensionStore {
    pub fn register_extension<T: Extension>(&mut self) {
        let id = T::id();

        self.pools.entry(id).or_insert(
            Box::new(ExtensionPool::<T>::default())
        );
    }

    pub fn add_extension<T: Extension>(&mut self, node_id: NodeId, extension: T) {
        let pool = self.pools.get_mut(&T::id()).expect("ERROR AT ADDING EXTENSION. EXTENSION WASN'T REGISTERED").downcast_mut::<ExtensionPool<T>>().unwrap();
        pool.insert(node_id, extension);
    }

    pub fn get_extension<T: Extension>(&self, node_id: NodeId) -> Option<&T> {
        let pool = self.pools.get(&T::id()).expect("ERROR AT ADDING EXTENSION. EXTENSION WASN'T REGISTERED").downcast_ref::<ExtensionPool<T>>().unwrap();
        pool.get(node_id)
    }
    pub fn get_extension_mut<T: Extension>(&mut self, node_id: NodeId) -> Option<&mut T> {
        let pool = self.pools.get_mut(&T::id()).expect("ERROR AT ADDING EXTENSION. EXTENSION WASN'T REGISTERED").downcast_mut::<ExtensionPool<T>>().unwrap();
        pool.get_mut(node_id)
    }

    pub fn remove_extension<T: Extension>(&mut self, node_id: NodeId) {
        let pool = self.pools.get_mut(&T::id()).expect("ERROR AT ADDING EXTENSION. EXTENSION WASN'T REGISTERED").downcast_mut::<ExtensionPool<T>>().unwrap();
    }
}
