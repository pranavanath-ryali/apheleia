use tree_ds::prelude::{Node, Tree};

use crate::{
    dirty_tracker::DirtyTracker, extensions::store::ExtensionStore, node::store::NodeStore,
    resources::store::ResourceStore, systems::SystemStore, types::NodeId,
};

pub struct World {
    pub relations: Tree<NodeId, NodeId>,

    pub node_storage: NodeStore,
    pub extension_store: ExtensionStore,
    pub dirty_tracker: DirtyTracker,
    pub system_store: SystemStore,
    pub resource_store: ResourceStore,
}
impl Default for World {
    fn default() -> Self {
        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        _ = relations.add_node(Node::new(0, None), None);

        Self {
            relations,

            node_storage: NodeStore::default(),
            extension_store: ExtensionStore::default(),
            dirty_tracker: DirtyTracker::default(),
            system_store: SystemStore::default(),
            resource_store: ResourceStore::default(),
        }
    }
}
