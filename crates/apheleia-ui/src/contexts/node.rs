use std::{cell::RefCell, rc::Rc};

use apheleia_core::types::Vec2;

use crate::{
    builder::node::NodeBuilder,
    contexts::{
        commands::{AddExtensionToId, HookSystemToId, SetPosition, SetSize},
        traits::ContextCommand,
    },
    extensions::traits::Extension,
    id_generator::{IdGenerator, IdGeneratorTrait},
    types::{NodeId, System, UpdateType},
};

pub struct NodeContext {
    id: NodeId,
    id_generator: Rc<RefCell<IdGenerator<NodeId>>>,

    position: Vec2,
    size: Option<Vec2>,

    commands: Vec<Box<dyn ContextCommand>>,
}
impl NodeContext {
    pub(crate) fn new(
        id: NodeId,
        id_generator: Rc<RefCell<IdGenerator<NodeId>>>,
        position: Vec2,
        size: Option<Vec2>,
    ) -> NodeContext {
        Self {
            id,
            id_generator,

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

    pub fn create_node(&mut self, f: impl FnOnce(NodeBuilder) -> NodeBuilder) {
        let id = self.id_generator.borrow_mut().next();
        let mut builder = f(NodeBuilder::new(id, self.id_generator.clone()));
        self.commands.append(&mut builder.build());
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

    pub fn get_position(&self) -> Vec2 {
        self.position
    }
    pub fn set_position(&mut self, position: Vec2) {
        self.add_command(Box::new(SetPosition(self.get_id(), position)));
    }

    pub fn get_size(&self) -> Option<Vec2> {
        self.size
    }
    pub fn set_size(&mut self, size: Vec2) {
        self.add_command(Box::new(SetSize(self.get_id(), size)));
    }
}
