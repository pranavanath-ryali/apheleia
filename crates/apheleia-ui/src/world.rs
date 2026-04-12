use tree_ds::prelude::Tree;

use crate::{
    dirty_tracker::DirtyTracker, extensions::store::ExtensionStore, node::store::NodeStore,
    resources::store::ResourceStore, systems::store::SystemStore, types::NodeId,
};

pub struct SystemView<'a> {
    pub relations: &'a Tree<NodeId, NodeId>,

    pub node_storage: &'a NodeStore,
    pub extension_store: &'a mut ExtensionStore,
    pub resource_store: &'a mut ResourceStore,
}

pub struct WorldViewForCommands<'a> {
    pub relations: &'a mut Tree<NodeId, NodeId>,

    pub node_storage: &'a mut NodeStore,
    pub systems_store: &'a mut SystemStore,
    pub extension_store: &'a mut ExtensionStore,
    pub dirty_tracker: &'a mut DirtyTracker,
    pub resource_store: &'a mut ResourceStore,
}

// pub struct World {
//     pub relations: Tree<NodeId, NodeId>,

//     pub node_storage: Box<NodeStore>,
//     pub extension_store: Box<ExtensionStore>,
//     pub dirty_tracker: Box<DirtyTracker>,
//     pub system_store: Box<SystemStore>,
//     pub resource_store: Box<ResourceStore>,
// }
// impl Default for World {
// fn default() -> Self {
//     let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
//     _ = relations.add_node(Node::new(0, None), None);

//     Self {
//         relations,

//         node_storage: Box::new(NodeStore::default()),
//         extension_store: Box::new(ExtensionStore::default()),
//         dirty_tracker: Box::new(DirtyTracker::default()),
//         system_store: Box::new(SystemStore::default()),
//         resource_store: Box::new(ResourceStore::default()),
//     }
// }
// }
