use std::{mem::take, ptr};

use crate::{
    NodeId,
    constants::MAX_NODES,
    extensions::{Extension, store::ExtensionStore},
    id_generator::IdGenerator,
    nodedata_store::NodeDataStore,
    resources::{Resource, store::ResourceStore},
    systems::{stages::SystemRunStage, store::SystemStore, system::IntoSystem},
    types::NodeData,
};

pub struct World {
    nodeid_gen: IdGenerator<NodeId>,

    nodedata_store: NodeDataStore,
    extension_store: ExtensionStore,
    resource_store: ResourceStore,
    system_store: SystemStore,
}
impl Default for World {
    fn default() -> Self {
        Self {
            nodeid_gen: IdGenerator::new(MAX_NODES),

            nodedata_store: NodeDataStore::default(),
            extension_store: ExtensionStore::default(),
            resource_store: ResourceStore::default(),
            system_store: Default::default(),
        }
    }
}
impl World {
    // [`NodeDataStore`] functions
    pub fn set_data(&mut self, id: NodeId, data: NodeData) {
        self.nodedata_store.set_data(id, data);
    }

    // [`ResourceStore`] functions
    /// Add a resource.
    pub fn add_resource<R: Resource>(&mut self, resource: R) {
        self.resource_store.add_resource(Box::new(resource));
    }

    // Get reference of resource that was previously added to [`World`]
    pub fn get_resource<R: Resource + 'static>(&self) -> Option<&R> {
        self.resource_store.get_resource::<R>()
    }
    // Get mutable access of resource that was previously added to [`World`]
    pub fn get_resource_mut<R: Resource + 'static>(&mut self) -> Option<&mut R> {
        self.resource_store.get_resource_mut::<R>()
    }

    // [`ExtensionStore`] functions
    /// Add an extension and bind it to the given [`NodeId`]
    pub fn add_extension_to_node<E: Extension>(&mut self, node_id: NodeId, extension: E) {
        self.extension_store
            .add_extension_to_node(node_id, extension);
    }

    /// Get reference [`Extension`] binded to given [`NodeId`]
    pub fn get_extension<E: Extension>(&self, node_id: NodeId) -> Option<&E> {
        self.extension_store.get_extension::<E>(node_id)
    }
    /// Get mutable reference [`Extension`] binded to given [`NodeId`]
    pub fn get_extension_mut<E: Extension>(&mut self, node_id: NodeId) -> Option<&mut E> {
        self.extension_store.get_extension_mut::<E>(node_id)
    }

    // [`SystemStore`] functions
    /// Convert function to a [`System`] and register
    pub fn add_system<Params: 'static>(
        &mut self,
        stage: SystemRunStage,
        priority: u8,
        system: impl IntoSystem<Params>,
    ) {
        self.system_store.add_system(stage, priority, system);
    }
    // /// Run all [`System`]s registered for that stage and run in order of priority
    pub fn run_systems_on_stage(&mut self, stage: SystemRunStage) {
        let mut system_store = take(&mut self.system_store);
        system_store.run_systems_for_stage(stage, ptr::from_mut(self));
        self.system_store = system_store;
    }
}

// #[cfg(test)]
// mod Test {
//     use crate::systems::{self, system_param::SystemParam};
//
//     use super::*;
//
//     struct Res<R> {
//         value: *mut R,
//     }
//     impl<R: Resource + 'static> SystemParam for Res<R> {
//         type Item<'w> = Res<R>;
//
//         fn fetch<'w>(world: UnsafeWorldCellMut<'w>) -> Option<Self::Item<'w>> {
//             Some(Res {
//                 value: (unsafe { world.get_world_mut() })
//                     .get_resource_mut::<R>()
//                     .unwrap(),
//             })
//         }
//     }
//
//     struct TestRes;
//     impl Resource for TestRes {}
//     fn test_system(res: Res<TestRes>, res2: Res<TestRes>) {}
//
//     #[test]
//     fn test_world() {
//         use crate::constants::PRE_STAGE;
//
//         let mut world = World::default();
//
//         world.add_system(SystemRunStage::Render, PRE_STAGE, test_system);
//     }
// }
