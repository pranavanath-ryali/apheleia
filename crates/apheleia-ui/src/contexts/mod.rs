use apheleia_core::types::vector::Vector2;

use crate::{
    NodeId,
    node::data::NodeData,
    rootnode::data::{self, RootNodeData},
    types::EventData,
};
use std::mem;

pub struct Context<'a> {
    id: NodeId,
    data: NodeData,
    event_data: Option<EventData>,

    rootnode_data: RootNodeData<'a>,

    pub(crate) commands: Vec<Box<dyn ContextCommand>>,
}
impl<'a> Context<'a> {
    pub fn new(id: NodeId, data: NodeData, rootnode_data: RootNodeData<'a>) -> Self {
        Self {
            id,
            data,
            event_data: None,
            rootnode_data,
            commands: vec![],
        }
    }

    pub fn new_event(
        id: NodeId,
        data: NodeData,
        event_data: EventData,
        rootnode_data: RootNodeData<'a>,
    ) -> Self {
        Self {
            id,
            data,
            event_data: Some(event_data),
            rootnode_data,
            commands: vec![],
        }
    }

    pub fn get_id(&self) -> NodeId {
        self.id
    }

    pub fn get_event(&self) -> &Option<EventData> {
        &self.event_data
    }

    pub fn get_position(&self) -> &Vector2 {
        self.data.get_position()
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
            command.execute(self.id, &mut self.rootnode_data);
        }
    }
}

pub trait ContextCommand {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData);
}

pub mod commands;
