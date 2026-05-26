pub mod world_cell;

use crate::{
    NodeId,
    constants::MAX_NODES,
    extensions::store::ExtensionStore,
    id_generator::IdGenerator,
    nodedata_store::NodeDataStore,
    resources::{Resource, store::ResourceStore},
    systems::store::store::SystemStore,
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
    /// Add a resource to [`World`].
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
}
