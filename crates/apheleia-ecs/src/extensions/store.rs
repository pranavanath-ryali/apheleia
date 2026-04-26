use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use apheleia_types::{ExtensionId, IdGenerator, NodeId};
use rustc_hash::FxHashMap;

use crate::extensions::traits::Extension;

#[derive(Default)]
pub struct ExtensionStore {
    id_generator: IdGenerator<ExtensionId>,

    nodeid_extid: FxHashMap<NodeId, HashMap<TypeId, ExtensionId>>,
    extensions_storage: FxHashMap<ExtensionId, Box<dyn Any>>,
}
impl ExtensionStore {
    // pub fn get_id(&mut self) -> ExtensionId {
    //     self.id_generator.next()
    // }

    // pub fn add_extension_to_node(
    //     &mut self,
    //     node_id: NodeId,
    //     extension: Box<dyn Extension>,
    // ) -> Result<(), String> {
    //     let ext_id = self.id_generator.next();
    //     let type_id = &(*extension).type_id();
    //     self.add_extension(ext_id, extension);

    //     if !self.extensions_storage.contains_key(&ext_id) {
    //         return Err(format!(
    //             "Extension of ID: {} doesn't exist or not in ExtensionStore",
    //             ext_id
    //         )
    //         .to_string());
    //     }

    //     self.nodeid_extid
    //         .entry(node_id)
    //         .and_modify(|v| {
    //             // TODO: Return error if a node is already binded with an extension of this type
    //             v.entry(*type_id).or_insert(ext_id);
    //         })
    //         .or_insert_with(|| {
    //             let mut map = HashMap::new();
    //             map.insert(*type_id, ext_id);
    //             map
    //         });
    //     Ok(())
    // }

    // pub fn add_extension(&mut self, ext_id: ExtensionId, extension: Box<dyn Extension>) {
    //     self.extensions_storage.insert(ext_id, extension);
    // }

    // pub fn bind_extension<T: Extension>(
    //     &mut self,
    //     node_id: NodeId,
    //     ext_id: ExtensionId,
    // ) -> Result<(), String> {
    //     if !self.extensions_storage.contains_key(&ext_id) {
    //         return Err(format!(
    //             "Extension of ID: {} doesn't exist or not in ExtensionStore",
    //             ext_id
    //         )
    //         .to_string());
    //     }

    //     self.nodeid_extid
    //         .entry(node_id)
    //         .and_modify(|v| {
    //             // TODO: Return error if a node is already binded with an extension of this type
    //             v.entry(TypeId::of::<T>()).or_insert(ext_id);
    //         })
    //         .or_insert_with(|| {
    //             let mut map = HashMap::new();
    //             map.insert(TypeId::of::<T>(), ext_id);
    //             map
    //         });
    //     Ok(())
    // }

    // pub fn get_extension<T: Extension>(&self, node_id: NodeId) -> &T {
    //     let ext_id = self
    //         .nodeid_extid
    //         .get(&node_id)
    //         .unwrap()
    //         .get(&TypeId::of::<T>())
    //         .unwrap();

    //     self.extensions_storage
    //         .get(ext_id)
    //         .unwrap()
    //         .downcast_ref::<T>()
    //         .unwrap()
    // }
    // pub fn get_extension_mut<T: Extension>(&mut self, node_id: NodeId) -> &mut T {
    //     let ext_id = self
    //         .nodeid_extid
    //         .get(&node_id)
    //         .unwrap()
    //         .get(&TypeId::of::<T>())
    //         .unwrap();

    //     self.extensions_storage
    //         .get_mut(ext_id)
    //         .unwrap()
    //         .downcast_mut::<T>()
    //         .unwrap()
    // }
}
