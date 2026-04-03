use std::{cell::RefCell, mem, rc::Rc};

use crate::{contexts::ContextCommand, rootnode::RootNodeData, types::NodeId};

pub struct NodeContext {
    id: NodeId,
    rootnode_data: Rc<RefCell<RootNodeData>>,

    commands: Vec<Box<dyn ContextCommand>>,
}
impl NodeContext {
    pub fn new(id: NodeId, rootnode_data: Rc<RefCell<RootNodeData>>) -> NodeContext {
        Self {
            id,
            rootnode_data,
            commands: vec![],
        }
    }

    pub fn get_id(&self) -> NodeId {
        self.id
    }

    pub(crate) fn run_commands(&mut self) {
        let commands = mem::take(&mut self.commands);
        for command in commands {
            command.execute(self.get_id(), self.rootnode_data.clone());
        }
    }
}
