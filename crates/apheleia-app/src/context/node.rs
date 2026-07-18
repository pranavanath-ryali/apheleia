use std::collections::VecDeque;

use apheleia_ecs::{
    commands::{
        ContextCommand, extension::AddExtensionToNode, node::SetDataForNode, resource::AddResource,
        system::AddSystem,
    },
    extensions::Extension,
    nodedata::data::NodeData,
    resources::Resource,
    systems::system::IntoSystem,
    types::{NodeId, SystemRunStage},
    world::World,
};
use log::info;

use crate::{app::App, builder::node::NodeBuilder};

pub struct NodeContext<'w> {
    id: NodeId,
    app: &'w mut App,
    commands: VecDeque<Box<dyn ContextCommand>>,
}
impl<'w> NodeContext<'w> {
    pub(crate) fn new(id: NodeId, app: &'w mut App) -> Self {
        Self {
            id,
            app,
            commands: Default::default(),
        }
    }

    pub fn create_node(&mut self, f: impl FnOnce(NodeBuilder) -> NodeBuilder) {
        info!("[APP] building new node");

        let builder = f(NodeBuilder::new(self.id, self.app));
        builder.execute();
    }

    pub fn add_resource<R: Resource + 'static>(&mut self, res: R) {
        self.commands.push_back(AddResource::new(res));
    }

    pub fn add_extension<E: Extension>(&mut self, extension: E, id: Option<NodeId>) {
        self.commands
            .push_back(AddExtensionToNode::new(id.unwrap_or(self.id), extension));
    }

    pub fn set_nodedata(&mut self, data: NodeData, id: Option<NodeId>) {
        self.commands
            .push_back(SetDataForNode::new(id.unwrap_or(self.id), data));
    }

    pub fn add_system<Params: 'static>(
        &mut self,
        stage: SystemRunStage,
        priority: u16,
        system: impl IntoSystem<Params>,
    ) {
        self.commands
            .push_back(AddSystem::new(stage, priority, system));
    }

    pub(crate) fn get_commands(&mut self) -> &mut VecDeque<Box<dyn ContextCommand>> {
        &mut self.commands
    }
}
