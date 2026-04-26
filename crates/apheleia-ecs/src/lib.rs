use apheleia_types::NodeId;

use crate::{
    extensions::{store::ExtensionStore, traits::Extension},
    node::{NodeData, store::NodeDataStore},
    resources::{store::ResourceStore, traits::Resource},
};

pub mod extensions;
pub mod node;
pub mod resources;

#[derive(Default)]
pub struct World {
    data_store: NodeDataStore,
    extension_store: ExtensionStore,
    resource_store: ResourceStore,
}
impl World {
    pub fn create_node(&mut self, class: Option<&str>, data: NodeData) -> NodeId {
        self.data_store.create_node(class, data)
    }

    pub fn add_resource<R: Resource + 'static>(&mut self, resource: R) {
        self.resource_store.add_resource(Box::new(resource));
    }
}
