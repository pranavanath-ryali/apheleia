use apheleia_core::types::Vector2;

use crate::{
    contexts::{
        commands::{AddExtensionToId, HookSystemToId, SetPosition, SetSize},
        traits::ContextCommand,
    },
    extensions::traits::Extension,
    types::{NodeId, System, UpdateType},
};

pub struct NodeContext {
    id: NodeId,

    position: Vector2,
    size: Option<Vector2>,

    commands: Vec<Box<dyn ContextCommand>>,
}
impl NodeContext {
    pub(crate) fn new(id: NodeId, position: Vector2, size: Option<Vector2>) -> NodeContext {
        Self {
            id,
            position,
            size,

            commands: vec![],
        }
    }

    pub fn get_id(&self) -> NodeId {
        self.id
    }

    pub fn add_command(&mut self, command: Box<dyn ContextCommand>) {
        self.commands.push(command);
    }

    pub(crate) fn get_commands(&mut self) -> &mut Vec<Box<dyn ContextCommand>> {
        &mut self.commands
    }

    pub fn add_extension<E: Extension>(&mut self, extension: E) {
        self.add_command(Box::new(AddExtensionToId(
            self.get_id(),
            Box::new(extension),
        )));
    }

    pub fn add_system(&mut self, update_type: UpdateType, priority: isize, system: System) {
        self.add_command(Box::new(HookSystemToId {
            id: self.get_id(),
            update_type,
            priority,
            system,
        }));
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }
    pub fn set_position(&mut self, position: Vector2) {
        self.add_command(Box::new(SetPosition(self.get_id(), position)));
    }

    pub fn get_size(&self) -> Option<Vector2> {
        self.size
    }
    pub fn set_size(&mut self, size: Vector2) {
        self.add_command(Box::new(SetSize(self.get_id(), size)));
    }
}
