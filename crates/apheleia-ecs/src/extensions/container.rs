use std::any::Any;

use apheleia_types::{Extension, ExtensionId, MAX_NODES};
use sparseset::SparseSet;

pub trait ExtensionContainer<T: Extension>: Any {
    fn insert(&mut self, id: ExtensionId, extension: T);
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
        assert!(self.extensions.insert(id, extension), "Extension already exists with ID: {}", id);
    }
}
