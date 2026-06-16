use rustc_hash::FxHashMap;
use std::any::{Any, TypeId};

use crate::{
    constants::MAX_EXTENSIONS,
    extensions::{
        Extension,
        container::{ExtensionContainer, ExtensionContainerSingle},
    },
    id_generator::IdGenerator,
    types::{ExtensionId, NodeId},
};

pub(crate) struct ExtensionStore {
    id_generator: IdGenerator<ExtensionId>,

    node_to_ext: FxHashMap<NodeId, FxHashMap<TypeId, ExtensionId>>,
    exttype_to_node: FxHashMap<TypeId, Vec<NodeId>>,
    exttype_to_container: FxHashMap<TypeId, Box<dyn Any>>,
}
impl Default for ExtensionStore {
    fn default() -> Self {
        Self {
            id_generator: IdGenerator::new(MAX_EXTENSIONS),

            exttype_to_node: Default::default(),
            node_to_ext: Default::default(),
            exttype_to_container: Default::default(),
        }
    }
}
impl ExtensionStore {
    pub fn add_extension_to_node<T: Extension>(&mut self, node_id: NodeId, extension: T) {
        let ext_id = self.id_generator.next();
        let type_id = TypeId::of::<T>();

        if let Some(container) = self.exttype_to_container.get_mut(&type_id) {
            container
                .downcast_mut::<ExtensionContainerSingle<T>>()
                .unwrap()
                .insert(ext_id, extension);
        } else {
            let mut container = Box::new(ExtensionContainerSingle::<T>::new());
            container.insert(ext_id, extension);
            self.exttype_to_container.insert(type_id, container);
        }

        self.node_to_ext
            .entry(node_id)
            .and_modify(|v| {
                v.entry(type_id)
                    .and_modify(|_| panic!("Extension already exists for node"))
                    .or_insert(ext_id);
            })
            .or_insert_with(|| {
                let mut map: FxHashMap<TypeId, ExtensionId> = Default::default();
                map.insert(type_id, ext_id);
                map
            });
        self.exttype_to_node
            .entry(type_id)
            .and_modify(|v| v.push(node_id))
            .or_insert(vec![node_id]);
    }

    pub fn get_extension<T: Extension>(&self, node_id: NodeId) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        if let Some(container) = self.exttype_to_container.get(&type_id)
            && let Some(typeid_to_extid) = self.node_to_ext.get(&node_id)
            && let Some(ext_id) = typeid_to_extid.get(&type_id)
        {
            let ext = container
                .downcast_ref::<ExtensionContainerSingle<T>>()
                .unwrap()
                .get(*ext_id);
            return ext;
        }
        None
    }

    pub fn get_extension_mut<T: Extension>(&mut self, node_id: NodeId) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();

        if let Some(container) = self.exttype_to_container.get_mut(&type_id)
            && let Some(typeid_to_extid) = self.node_to_ext.get(&node_id)
            && let Some(ext_id) = typeid_to_extid.get(&type_id)
        {
            return container
                .downcast_mut::<ExtensionContainerSingle<T>>()
                .unwrap()
                .get_mut(*ext_id);
        }
        None
    }

    pub fn get_nodes_with_extension<T: Extension>(&self) -> Vec<NodeId> {
        let type_id = TypeId::of::<T>();
        let mut ids: Vec<NodeId> = vec![];

        if let Some(nodes) = self.exttype_to_node.get(&type_id) {
            nodes.iter().for_each(|id| ids.push(*id));
        }
        ids
    }
}

// #[cfg(test)]
// mod test_extensions {
//     use super::*;

//     #[derive(Debug)]
//     struct TestExtension {
//         value: u16,
//     }
//     impl Extension for TestExtension {}

//     // #[test]
//     fn test_extension_store() {
//         let mut store = ExtensionStore::default();

//         store.add_extension_to_node(10, TestExtension { value: 5 });
//         store.

//         assert_eq!(store.get_extension::<TestExtension>(10).unwrap().value, 5);
//     }
// }
