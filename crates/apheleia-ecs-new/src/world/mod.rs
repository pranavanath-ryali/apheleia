use crate::{NodeId, extensions::store::ExtensionStore, id_generator::IdGenerator, nodedata_store::NodeDataStore};

pub struct World {
    nodeid_gen: IdGenerator<NodeId>,

    nodedata_store: NodeDataStore,
    extension_store: ExtensionStore,
}
impl Default for World {
    fn default() -> Self {
        Self {
            nodeid_gen: IdGenerator::new(0),

            nodedata_store: NodeDataStore::default(),
            extension_store: ExtensionStore::default(),
        }
    }
}
