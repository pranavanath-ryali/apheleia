use apheleia_core::{buffer::Buffer, types::vector::Vector2};

use crate::{
    rootnode::RootNodeData,
    types::{EventData, NodeId},
};
use std::{cell::RefCell, mem, rc::Rc};

pub struct Context<'a> {
    id: NodeId,
    event_data: Option<&'a EventData>,
    buffer: Option<&'a mut Buffer>,

    rootnode_data: Rc<RefCell<RootNodeData>>,

    pub(crate) commands: Vec<Box<dyn ContextCommand>>,
}
impl<'a> Context<'a> {
    pub fn new(id: NodeId, rootnode_data: Rc<RefCell<RootNodeData>>) -> Self {
        Self {
            id,
            event_data: None,
            buffer: None,
            rootnode_data,
            commands: vec![],
        }
    }

    pub fn new_event(
        id: NodeId,
        event_data: &'a EventData,
        rootnode_data: Rc<RefCell<RootNodeData>>,
    ) -> Self {
        Self {
            id,
            event_data: Some(event_data),
            buffer: None,
            rootnode_data,
            commands: vec![],
        }
    }

    pub fn new_render(
        id: NodeId,
        buffer: &'a mut Buffer,
        rootnode_data: Rc<RefCell<RootNodeData>>,
    ) -> Self {
        Self {
            id,
            event_data: None,
            buffer: Some(buffer),
            rootnode_data,
            commands: vec![],
        }
    }

    pub fn get_id(&self) -> NodeId {
        self.id
    }

    pub fn get_event(&self) -> &Option<&'a EventData> {
        &self.event_data
    }

    pub fn get_position(&self) -> Vector2 {
        self.rootnode_data
            .borrow()
            .node_storage
            .get_data(self.id)
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

    pub fn add_command(&mut self, command: Box<dyn ContextCommand>) {
        self.commands.push(command);
    }

    pub(crate) fn run_commands(&mut self) {
        let commands = mem::take(&mut self.commands);
        for command in commands {
            command.execute(self.id, self.rootnode_data.clone());
        }
    }
}

pub trait ContextCommand {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: Rc<RefCell<RootNodeData>>);
}

pub mod commands;
