use std::collections::HashMap;

use apheleia_core::types::vector::Vector2;
use crossterm::event::{KeyCode, KeyEvent};
use tree_ds::prelude::Tree;

use crate::{
    NodeId,
    node::data::{DirtyRenderLevel, NodeData},
    rootnode::RootNode,
    types::{EventType},
};

pub enum Commands {
    SetSize(Vector2),
    SetPosition(Vector2),

    RegisterForUpdate,
    RegisterForEvent(EventType),

    MarkRenderDirty(DirtyRenderLevel),
}
pub enum EventData {
    Resize(Vector2),
    Keys(KeyEvent),
}
pub struct Context<'a> {
    id: NodeId,

    event_data: Option<EventData>,

    class_ids: &'a HashMap<String, NodeId>,
    relations: &'a Tree<NodeId, NodeId>,

    pub commands: Box<Vec<Commands>>,
}
impl<'a> Context<'a> {
    pub fn new(id: NodeId, class_ids: &'a HashMap<String, NodeId>, relations: &'a Tree<NodeId, NodeId>) -> Self {
        Self {
            id,

            event_data: None,
            
            class_ids,
            relations,

            commands: Box::new(vec![]),
        }
    }

    pub fn new_event_context(id: NodeId, class_ids: &'a HashMap<String, NodeId>, relations: &'a Tree<NodeId, NodeId>, event_data: EventData) -> Self {
        Self {
            id,

            event_data: Some(event_data),
            
            class_ids,
            relations,

            commands: Box::new(vec![]),
        }
    }

    pub fn get_event(&self) -> &Option<EventData> {
        &self.event_data
    }

    pub fn get_class_by_id(&self, class: &str) -> Option<NodeId> {
        if let Some(id) = self.class_ids.get(class) {
            return Some(*id);
        }
        None
    }

    pub fn get_children(&self, id: NodeId) -> Option<Vec<NodeId>> {
        // TODO: Fix cases where no. of children is 0. Then return None
        let mut children: Vec<NodeId> = vec![];
        self.relations
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
        
        Some(children)
    }

    pub fn add_command(&mut self, command: Commands) {
        self.commands.push(command);
    }
}
