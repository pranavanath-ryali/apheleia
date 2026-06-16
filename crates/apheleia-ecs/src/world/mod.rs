pub mod buffers;
pub mod commands;
pub mod events;
pub mod extensions;
pub mod nodedata;
pub mod relations;
pub mod resources;
pub mod systems;
pub mod tags;

use std::{collections::VecDeque, mem::take};

use apheleia_core::buffer::Buffer;
use log::{info, warn};
use smallvec::SmallVec;
use tree_ds::prelude::{Node, NodeRemovalStrategy, Tree};

use crate::{
    buffer_store::BufferStore,
    commands::ContextCommand,
    constants::MAX_NODES,
    events::EventTrait,
    events::tracker::EventTracker,
    extensions::{Extension, store::ExtensionStore},
    id_generator::IdGenerator,
    nodedata::{data::NodeData, store::NodeDataStore},
    resources::{Resource, store::ResourceStore},
    systems::{
        stages::SystemRunStage,
        store::SystemStore,
        system::{IntoSystem, System},
    },
    tags::{TagTrait, registry::TagRegistry},
    types::NodeId,
};
use indexmap::IndexSet;

pub struct World {
    pub running: bool,

    pub nodeid_gen: IdGenerator<NodeId>,
    relations: Tree<NodeId, NodeId>,

    pub current_stage: SystemRunStage,

    registered_nodes: VecDeque<NodeId>,
    tag_registry: TagRegistry,

    event_tracker: EventTracker,
    nodedata_store: NodeDataStore,
    extension_store: ExtensionStore,
    resource_store: ResourceStore,
    system_store: SystemStore,
    buffer_store: BufferStore,

    commands: VecDeque<Box<dyn ContextCommand>>,
}
impl Default for World {
    fn default() -> Self {
        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        _ = relations.add_node(Node::new(0, None), None);

        warn!("[ECS] - Created new World");

        Self {
            running: true,
            nodeid_gen: IdGenerator::new(MAX_NODES),
            relations,

            current_stage: SystemRunStage::Event,

            registered_nodes: Default::default(),
            tag_registry: Default::default(),

            event_tracker: Default::default(),
            nodedata_store: Default::default(),
            extension_store: Default::default(),
            resource_store: Default::default(),
            system_store: Default::default(),
            buffer_store: Default::default(),

            commands: Default::default(),
        }
    }
}
