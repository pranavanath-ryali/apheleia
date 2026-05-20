use std::any::{Any, TypeId};

use rustc_hash::FxHashMap;

use crate::{ExtensionId, NodeId, extensions::{Extension, container::{ExtensionContainer, ExtensionContainerSingle}}, id_generator::IdGenerator};

pub(crate) struct ExtensionStore {
    id_generator: IdGenerator<ExtensionId>,

    node_to_ext: FxHashMap<NodeId, FxHashMap<TypeId, ExtensionId>>,
    containers: FxHashMap<TypeId, Box<dyn Any>>,
}
impl Default for ExtensionStore {
    fn default() -> Self {
        Self {
            id_generator: IdGenerator::new(0),

            node_to_ext: Default::default(),
            containers: Default::default(),
        }
    }
}
impl ExtensionStore {
    pub fn add_extension_to_node<T: Extension>(&mut self, node_id: NodeId, extension: T) {
        let ext_id = self.id_generator.next();
        let type_id = TypeId::of::<T>();

        self.containers
            .entry(type_id)
            .and_modify(|container| {
                container
                    .downcast_mut::<ExtensionContainerSingle<T>>()
                    .unwrap()
                    .insert(ext_id, extension);
            })
            .or_insert_with(|| Box::new(ExtensionContainerSingle::<T>::new()));
    }

    pub fn get_extension<T: Extension>(&self, node_id: NodeId) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        if let Some(container) = self.containers.get(&type_id) {
            return container
                .downcast_ref::<ExtensionContainerSingle<T>>()
                .unwrap()
                .get(node_id);
        }
        None
    }

    pub fn get_extension_mut<T: Extension>(&mut self, node_id: NodeId) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();

        if let Some(container) = self.containers.get_mut(&type_id) {
            return container
                .downcast_mut::<ExtensionContainerSingle<T>>()
                .unwrap()
                .get_mut(node_id);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestExtension {
        value: u16,
    }
    impl Extension for TestExtension {}

    #[test]
    fn test_extension_store() {
        let mut store = ExtensionStore::default();

        store.add_extension_to_node(10, TestExtension { value: 5 });

        assert_eq!(store.get_extension::<TestExtension>(10).unwrap().value, 5);
    }
}
