use apheleia_core::{buffer::Buffer, types::vector::Vector2};

use crate::{
    rootnode::RootNodeData,
    systems::System,
    types::{EventData, NodeId, UpdateTypeNode},
};
use std::{cell::RefCell, mem, rc::Rc};

pub struct Context<'a> {
    id: Option<NodeId>,
    event_data: Option<&'a EventData>,
    buffer: Option<&'a mut Buffer>,
    // buffer: Option<Buffer>,
    rootnode_data: Rc<RefCell<RootNodeData>>,

    pub(crate) commands: Vec<Box<dyn ContextCommand>>,
}
impl<'a> Context<'a> {
    pub fn set_id(&mut self, id: NodeId) {
        self.id = Some(id);
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

    pub fn get_id(&self) -> NodeId {
        // Unwrap because this should ideally never be None
        self.id.unwrap()
    }

    pub fn get_event(&self) -> &Option<&'a EventData> {
        &self.event_data
    }

    pub fn get_position(&self) -> Vector2 {
        self.rootnode_data
            .borrow()
            .node_storage
            .get_data(self.get_id())
            .unwrap()
            .get_position()
    }

    // pub fn get_data_for_id(&self, id: NodeId) -> Option<&NodeData> {
    //     if let Some(data) = self.rootnode_data.node_storage.borrow().get_data(id) {
    //         // Some(data) // :(
    //     } else {
    //         None
    //     }
    // }

    pub fn get_children(&self, id: NodeId) -> Vec<NodeId> {
        // TODO: Fix cases where no. of children is 0. Then return None
        let mut children: Vec<NodeId> = vec![];
        self.rootnode_data
            .borrow()
            .relations
            .get_subtree(&id, Some(1))
            .unwrap()
            .traverse(&id, tree_ds::prelude::TraversalStrategy::PreOrder)
            .unwrap()
            .iter()
            .for_each(|i| {
                if *i == id {
                    return;
                }

                children.push(*i);
            });

        children
    }

    pub fn get_buffer(&mut self) -> &mut Buffer {
        self.buffer.as_mut().unwrap()
    }

    pub fn add_system(&mut self, update_type: UpdateTypeNode, priority: isize, system: System) {
        let id = self.get_id();
        self.rootnode_data
            .borrow_mut()
            .system_store
            .add_system(id, update_type, priority, system);
    }

    pub fn add_command(&mut self, command: Box<dyn ContextCommand>) {
        self.commands.push(command);
    }

    pub(crate) fn run_commands(&mut self) {
        let commands = mem::take(&mut self.commands);
        for command in commands {
            command.execute(self.get_id(), self.rootnode_data.clone());
        }
    }
}

pub trait ContextCommand {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: Rc<RefCell<RootNodeData>>);
}

pub mod commands;
pub mod node;
pub mod systems;
