use std::{cell::RefCell, mem, rc::Rc};

use apheleia_core::buffer::Buffer;

use crate::{
    contexts::traits::ContextCommand,
    rootnode::RootNodeData,
    types::{EventData, NodeId},
};

pub struct SystemContext<'a> {
    id: Option<NodeId>,
    event_data: Option<&'a EventData>,
    buffer: Option<&'a mut Buffer>,

    rootnode_data: Rc<RefCell<RootNodeData>>,
    commands: Vec<Box<dyn ContextCommand>>,
}
impl<'a> SystemContext<'a> {
    pub fn set_id(&mut self, id: NodeId) {
        self.id = Some(id);
    }
    pub fn get_id(&mut self) -> NodeId {
        self.id.unwrap()
    }

    pub fn new(rootnode_data: Rc<RefCell<RootNodeData>>) -> Self {
        Self {
            id: None,
            event_data: None,
            buffer: None,
            rootnode_data,
            commands: vec![],
        }
    }

    pub fn new_event(event_data: &'a EventData, rootnode_data: Rc<RefCell<RootNodeData>>) -> Self {
        Self {
            id: None,
            event_data: Some(event_data),
            buffer: None,
            rootnode_data,
            commands: vec![],
        }
    }

    pub fn new_render(buffer: &'a mut Buffer, rootnode_data: Rc<RefCell<RootNodeData>>) -> Self {
        Self {
            id: None,
            event_data: None,
            buffer: Some(buffer),
            rootnode_data,
            commands: vec![],
        }
    }

    pub(crate) fn run_commands(&mut self) {
        let commands = mem::take(&mut self.commands);
        for command in commands {
            command.execute(self.get_id(), self.rootnode_data.clone());
        }
    }
}
