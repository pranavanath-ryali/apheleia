use std::{collections::VecDeque, mem::take, ptr};

use log::{info, warn};
use tree_ds::prelude::{Node, Tree};

use crate::{
    NodeId,
    command::ContextCommand,
    constants::MAX_NODES,
    extensions::{Extension, store::ExtensionStore},
    id_generator::IdGenerator,
    nodedata_store::NodeDataStore,
    resources::{Resource, store::ResourceStore},
    systems::{
        stages::SystemRunStage,
        store::SystemStore,
        system::{IntoSystem, System},
    },
    tag::tag_registry::TagRegistry,
    types::NodeData,
};

pub struct World {
    pub nodeid_gen: IdGenerator<NodeId>,
    relations: Tree<NodeId, NodeId>,

    registered_nodes: VecDeque<NodeId>,
    tag_registry: TagRegistry,

    nodedata_store: NodeDataStore,
    extension_store: ExtensionStore,
    resource_store: ResourceStore,
    system_store: SystemStore,

    commands: VecDeque<Box<dyn ContextCommand>>,
}
impl Default for World {
    fn default() -> Self {
        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        _ = relations.add_node(Node::new(0, None), None);

        Self {
            nodeid_gen: IdGenerator::new(MAX_NODES),
            relations,

            registered_nodes: Default::default(),
            tag_registry: Default::default(),

            nodedata_store: Default::default(),
            extension_store: Default::default(),
            resource_store: Default::default(),
            system_store: Default::default(),

            commands: Default::default(),
        }
    }
}
impl World {
    /// Tags a node
    #[inline]
    pub fn tag_node(&mut self, tag: usize, node: NodeId) {
        self.tag_registry.tag_node(tag, node);
    }
    /// Gets all nodes given tag
    #[inline]
    pub fn get_nodes_tagged(&self, tag: usize) -> Option<&smallvec::SmallVec<[usize; 4]>> {
        self.tag_registry.get_nodes(tag)
    }

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

    pub fn create_node(&mut self) -> NodeId {
        let id = self.nodeid_gen.next();
        self.registered_nodes.push_back(id);
        id
    }
    #[inline]
    pub fn get_registered_nodes(&self) -> &VecDeque<NodeId> {
        &self.registered_nodes
    }

    // [`NodeDataStore`] functions
    /// Get [`NodeData`] for given [`NodeId`]
    #[inline]
    pub fn get_nodedata(&self, id: NodeId) -> Option<&NodeData> {
        self.nodedata_store.get_data(id)
    }
    #[inline]
    pub fn set_data(&mut self, id: NodeId, data: NodeData) {
        self.nodedata_store.set_data(id, data);
    }
    // TODO: Write get functions

    // [`ResourceStore`] functions
    /// Add and register given [`Resource`].
    #[inline]
    pub fn add_resource<R: Resource>(&mut self, resource: R) {
        self.resource_store.add_resource(Box::new(resource));
    }
    /// Add and register boxed [`Resource`]
    #[inline]
    pub fn add_resource_boxed<R: Resource>(&mut self, resource: Box<R>) {
        self.resource_store.add_resource(resource);
    }

    /// Get reference of resource that was previously added to [`World`]
    #[inline]
    pub fn get_resource<R: Resource + 'static>(&self) -> Option<&R> {
        self.resource_store.get_resource::<R>()
    }
    /// Get mutable access of resource that was previously added to [`World`]
    #[inline]
    pub fn get_resource_mut<R: Resource + 'static>(&mut self) -> Option<&mut R> {
        self.resource_store.get_resource_mut::<R>()
    }

    // [`ExtensionStore`] functions
    /// Add an extension and bind it to the given [`NodeId`]
    #[inline]
    pub fn add_extension_to_node<E: Extension>(&mut self, node_id: NodeId, extension: E) {
        info!("ECS - Adding extension: {:#?} to node: {}", extension, node_id);
        self.extension_store
            .add_extension_to_node(node_id, extension);
    }

    /// Get reference [`Extension`] binded to given [`NodeId`]
    #[inline]
    pub fn get_extension<E: Extension>(&self, node_id: NodeId) -> Option<&E> {
        self.extension_store.get_extension::<E>(node_id)
    }
    /// Get mutable reference [`Extension`] binded to given [`NodeId`]
    #[inline]
    pub fn get_extension_mut<E: Extension>(&mut self, node_id: NodeId) -> Option<&mut E> {
        self.extension_store.get_extension_mut::<E>(node_id)
    }

    /// Get all [`NodeId`]s with [`Extension`]
    #[inline]
    pub fn get_nodes_with_extension<E: Extension>(&self) -> Vec<NodeId> {
        self.extension_store.get_nodes_with_extension::<E>()
    }

    // [`SystemStore`] functions
    /// Convert given function to a [`System`] and register
    #[inline]
    pub fn add_system<Params: 'static>(
        &mut self,
        stage: SystemRunStage,
        priority: u8,
        system: impl IntoSystem<Params>,
    ) {
        self.system_store.add_system(stage, priority, system);
    }
    /// Register and add [`System`]
    #[inline]
    pub fn add_system_boxed(
        &mut self,
        stage: SystemRunStage,
        priority: u8,
        system: Box<dyn System>,
    ) {
        self.system_store.add_system_boxed(stage, priority, system);
    }
    /// Run all [`System`]s registered for that stage and run in order of priority
    #[inline]
    pub fn run_systems_on_stage(&mut self, stage: SystemRunStage) {
        let ptr = self as *mut World;
        self.system_store.run_systems_for_stage(stage, ptr);
    }

    /// Add [`ContextCommand`] to queue
    #[inline]
    pub fn add_command(&mut self, command: Box<dyn ContextCommand>) {
        self.commands.push_back(command);
    }
    /// Append [`ContextCommand`]s to queue
    #[inline]
    pub fn apppend_commands(&mut self, commands: &mut VecDeque<Box<dyn ContextCommand>>) {
        info!("ECS - Commands to be appended: {:#?}", commands);
        self.commands.append(commands);
    }
    /// Execute [`ContextCommand`]s buffered
    pub fn execute_commands(&mut self) {
        let commands = take(&mut self.commands);
        for command in commands {
            command.execute(self);
        }
        warn!("ECS - Executing commands");
    }
}
