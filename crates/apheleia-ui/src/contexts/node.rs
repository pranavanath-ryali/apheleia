use std::{cell::RefCell, mem, rc::Rc};

use crate::{
    contexts::traits::ContextCommand,
    systems::System,
    types::{NodeId, UpdateTypeNode},
    world::World,
};

pub struct NodeContext {
    id: NodeId,
    rootnode_data: Rc<RefCell<World>>,

    commands: Vec<Box<dyn ContextCommand>>,
}
impl NodeContext {
    pub fn new(id: NodeId, rootnode_data: Rc<RefCell<World>>) -> NodeContext {
        Self {
            id,
            rootnode_data,
            commands: vec![],
        }
    }

    pub fn get_id(&self) -> NodeId {
        self.id
    }

    pub fn add_system(&mut self, update_type: UpdateTypeNode, priority: isize, system: System) {
        let id = self.get_id();
        self.rootnode_data
            .borrow_mut()
            .system_store
            .add_system(id, update_type, priority, system);
    }

    pub(crate) fn run_commands(&mut self) {
        let commands = mem::take(&mut self.commands);
        for command in commands {
            command.execute(self.get_id(), self.rootnode_data.clone());
        }
    }
}
