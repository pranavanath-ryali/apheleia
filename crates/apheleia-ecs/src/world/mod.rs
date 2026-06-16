pub mod events;
pub mod tags;
pub mod resources;
pub mod extensions;
pub mod systems;
pub mod commands;
pub mod buffers;
pub mod nodedata;

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
impl World {
    // ==================[RELATIONS FUNCTIONS]==================
    /// Returns mutable reference to current relations of type [`Tree`]
    #[inline]
    pub fn get_relations_mut(&mut self) -> &mut Tree<NodeId, NodeId> {
        &mut self.relations
    }
    /// Returns reference to current relations of type [`Tree`]
    #[inline]
    pub fn get_relations(&self) -> &Tree<NodeId, NodeId> {
        &self.relations
    }

    pub fn relate_node_with_parent(&mut self, child: NodeId, parent: NodeId) {
        assert!(self.relations.get_node_by_id(&parent).is_some());

        if self.relations.get_node_by_id(&child).is_none() {
            self.relations
                .add_node(Node::new(child, None), Some(&parent))
                .unwrap();

            info!(
                "[ECS] Child NodeID: {} related with Parent NodeID: {}",
                child, parent
            );
            return;
        }

        // Retain children, and move the subtree along with the child if it has any
        assert!(self.relations.get_node_by_id(&child).is_some());
        let subtree = self
            .relations
            .get_subtree(&child, None)
            .expect("Couldn't get subtree");
        self.relations
            .remove_node(&child, NodeRemovalStrategy::RemoveNodeAndChildren)
            .expect("Couldn't remove node from relations");
        self.relations
            .add_subtree(&parent, subtree)
            .expect("Couldn't add subtree to relations");
        info!(
            "[ECS] Moved Child NodeID and all its children: {} to parent NodeID: {}",
            child, parent
        );
    }

}
