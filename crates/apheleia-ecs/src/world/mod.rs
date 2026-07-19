use std::collections::VecDeque;

use apheleia_core::types::Vec2;
use log::warn;
use tree_ds::prelude::{Node, Tree};

use crate::{
    constants::MAX_NODES,
    id_generator::IdGenerator,
    stores::{extension::ExtensionStore, nodedata::NodeDataStore, resource::ResourceStore, system::SystemStore},
    traits::context_command::ContextCommand,
    types::{NodeId, SystemRunStage},
};

pub mod commands;
pub mod extensions;
pub mod nodedata;
pub mod relations;
pub mod resources;
pub mod systems;

pub struct World {
    pub terminal_size: Vec2,
    pub running: bool,
    pub current_stage: SystemRunStage,

    nodeid_gen: IdGenerator<NodeId>,
    relations: Tree<NodeId, NodeId>,
    registered_nodes: VecDeque<NodeId>,

    nodedata_store: NodeDataStore,
    extension_store: ExtensionStore,
    resource_store: ResourceStore,
    system_store: SystemStore,

    commands: VecDeque<Box<dyn ContextCommand>>,
}
impl World {
    pub fn new(terminal_size: Vec2) -> Self {
        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        _ = relations.add_node(Node::new(0, None), None);

        warn!("[ECS] - Created new World");

        Self {
            terminal_size,
            running: true,
            current_stage: SystemRunStage::Event,

            nodeid_gen: IdGenerator::new(MAX_NODES),
            relations,
            registered_nodes: Default::default(),

            nodedata_store: Default::default(),
            extension_store: Default::default(),
            resource_store: Default::default(),
            system_store: Default::default(),

            commands: Default::default(),
        }
    }
}
