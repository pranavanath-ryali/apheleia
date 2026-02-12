use std::collections::HashMap;

use apheleia_core::types::vector::Vector2;
use crossterm::event::{KeyCode, KeyEvent};
use tree_ds::prelude::Tree;

use crate::{
    NodeId,
    node::data::{DirtyRenderLevel, NodeData},
    rootnode::{RootNode, RootNodeData},
    types::EventType,
};

pub enum EventData {
    Resize(Vector2),
    Keys(KeyEvent),
}
pub struct Context<'a> {
    id: NodeId,
    event_data: Option<EventData>,

    rootnode_data: RootNodeData<'a>,
    
    pub(crate) commands: Vec<Box<dyn ContextCommand>>,
}
impl<'a> Context<'a> {
    pub fn new(id: NodeId, rootnode_data: RootNodeData<'a>) -> Self {
        Self { id, event_data: None, rootnode_data, commands: vec![] }
    }

    pub fn new_event(id: NodeId, event_data: EventData, rootnode_data: RootNodeData<'a>) -> Self {
        Self { id, event_data: Some(event_data), rootnode_data, commands: vec![] }
    }

    pub fn get_id(&self) -> NodeId {
        self.id
    }

    pub fn get_event(&self) -> &Option<EventData> {
        &self.event_data
    }

    pub fn get_class_by_id(&self, class: &str) -> Option<NodeId> {
        self.rootnode_data.class_id.get(class).copied()
    }

    pub fn get_data_for_id(&self, id: NodeId) -> Option<&NodeData> {
        self.rootnode_data.id_data.get(&id)
    }

    pub fn get_children(&self, id: NodeId) -> Vec<NodeId> {
        // TODO: Fix cases where no. of children is 0. Then return None
        let mut children: Vec<NodeId> = vec![];
        self.rootnode_data.relations
            .get_subtree(&id, Some(1))
            .unwrap()
            .traverse(&id, tree_ds::prelude::TraversalStrategy::PreOrder).unwrap()
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

    pub(crate) fn run_commands(&self) {}
}

pub trait ContextCommand {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData);
}

pub mod commands;
