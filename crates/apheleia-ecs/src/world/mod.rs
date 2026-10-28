pub mod commands;
pub mod extensions;
pub mod node;
pub mod resource;
pub mod nodedata;

use apheleia_types::id_gen::IdGenerator;
use tree_ds::prelude::{Node, Tree};

use crate::{
    commands::WorldCommand,
    stores::{
        extensions::ExtensionStore, nodedata::NodeDataStore, resource::ResourceStore,
        systems::SystemStore,
    },
    types::NodeId,
};

pub struct World {
    nodeid_gen: IdGenerator<NodeId>,
    relations: Tree<NodeId, NodeId>,

    nodedata: NodeDataStore,
    resource_store: ResourceStore,
    extension_store: ExtensionStore,
    system_store: SystemStore,

    commands: Vec<Box<dyn WorldCommand>>,
}
impl Default for World {
    fn default() -> Self {
        let mut relations = Tree::<NodeId, NodeId>::new(None);
        relations.add_node(Node::new(0, None), None).ok();

        Self {
            nodeid_gen: IdGenerator::new(1), // 0 is reserved for terminal 'root'
            relations,

            nodedata: Default::default(),
            resource_store: Default::default(),
            extension_store: Default::default(),
            system_store: Default::default(),

            commands: Default::default(),
        }
    }
}
