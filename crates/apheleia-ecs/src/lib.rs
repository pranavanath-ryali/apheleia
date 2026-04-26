use crate::{extensions::store::ExtensionStore, node::store::NodeDataStore};

pub mod extensions;
pub mod node;

#[derive(Default)]
pub struct World {
    data_store: NodeDataStore,
    extension_store: ExtensionStore,
}
impl World {}
