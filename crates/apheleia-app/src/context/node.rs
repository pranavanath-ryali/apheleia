use std::collections::VecDeque;

use apheleia_ecs_new::{NodeId, command::ContextCommand, extensions::Extension, resources::Resource, systems::{stages::SystemRunStage, system::IntoSystem}};

use crate::commands::{extension::AddExtensionToNode, resource::AddResource, system::AddSystem};

pub struct NodeContext {
    id: NodeId,
    commands: VecDeque<Box<dyn ContextCommand>>
}
impl NodeContext {
    pub(crate) fn new(id: NodeId) -> Self {
        Self {
            id,
            commands: Default::default(),
        }
    }

    pub fn add_resource<R: Resource + 'static>(&mut self, res: R) {
        self.commands.push_back(AddResource::new(res));
    }

    pub fn add_extension<E: Extension>(&mut self, extension: E) {
        self.commands.push_back(AddExtensionToNode::new(self.id, extension));
    }

    pub fn add_system<Params: 'static>(&mut self, stage: SystemRunStage, priority: u8, system: impl IntoSystem<Params>) {
        self.commands.push_back(AddSystem::new(stage, priority, system));
    }

    pub(crate) fn get_commands(&mut self) -> &mut VecDeque<Box<dyn ContextCommand>> {
        &mut self.commands
    }
}
