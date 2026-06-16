use std::any::Any;

use log::info;
use sparseset::SparseSet;

use crate::{constants::MAX_NODES, extensions::Extension, types::ExtensionId};

pub trait ExtensionContainer<T: Extension>: Any {
    fn insert(&mut self, id: ExtensionId, extension: T);
    fn get(&self, id: ExtensionId) -> Option<&T>;
    fn get_mut(&mut self, id: ExtensionId) -> Option<&mut T>;
}

pub struct ExtensionContainerSingle<T: Extension> {
    extensions: SparseSet<T>,
}
impl<T: Extension> ExtensionContainerSingle<T> {
    pub fn new() -> Self {
        Self { extensions: SparseSet::with_capacity(MAX_NODES) }
    }
}
impl<T: Extension> ExtensionContainer<T> for ExtensionContainerSingle<T> {
    fn insert(&mut self, id: ExtensionId, extension: T) {
        info!("[ECS] Addind Extension {:#?}", extension);
        assert!(self.extensions.insert(id, extension), "Extension already exists with ID: {}", id);
    }

    fn get(&self, id: ExtensionId) -> Option<&T> {
        self.extensions.get(id)
    }
    fn get_mut(&mut self, id: ExtensionId) -> Option<&mut T> {
        self.extensions.get_mut(id)
    }
}
