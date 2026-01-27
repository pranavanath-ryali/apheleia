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

// pub enum IntialCallCommands {
//     SetSize(Vector2),
//
//     RegisterForUpdate,
//     RegisterForEvent(EventType),
// }
// pub struct InitialCallContext {
//     position: Vector2,
//     size: Option<Vector2>,
//
//     commands: Vec<IntialCallCommands>,
// }
// impl InitialCallContext {
//     pub fn new(position: &Vector2, size: &Option<Vector2>) -> Self {
//         InitialCallContext {
//             position: *position,
//             size: *size,
//             commands: vec![],
//         }
//     }
//
//     pub fn add_command(&mut self, command: IntialCallCommands) {
//         self.commands.insert(0, command);
//     }
//     pub fn get_commands(&self) -> &Vec<IntialCallCommands> {
//         &self.commands
//     }
// }
//
// pub enum EventUpdateCommands {
//     SetSize(Vector2),
//     SetPosition(Vector2),
//
//     MarkRenderDirty(DirtyRenderLevel),
// }
// pub struct EventUpdateContext {
//     pub id: NodeId,
//
//     position: Vector2,
//     size: Option<Vector2>,
//
//     pub event_data: EventData,
//
//     pub commands: Vec<EventUpdateCommands>,
// }
// impl EventUpdateContext {
//     pub fn new(
//         id: NodeId,
//         position: &Vector2,
//         size: &Option<Vector2>,
//         event_data: EventData,
//     ) -> Self {
//         EventUpdateContext {
//             id,
//             position: *position,
//             size: *size,
//             event_data,
//             commands: vec![],
//         }
//     }
//
//     pub fn add_command(&mut self, command: EventUpdateCommands) {
//         self.commands.insert(0, command);
//     }
//     pub fn get_commands(&self) -> &Vec<EventUpdateCommands> {
//         &self.commands
//     }
// }
//
// pub enum UpdateCommands {
//     SetSize(Vector2),
//     SetPosition(Vector2),
//
//     MarkRenderDirty(DirtyRenderLevel),
// }
// pub struct UpdateContext {
//     pub id: NodeId,
//
//     position: Vector2,
//     size: Option<Vector2>,
//
//     pub commands: Vec<UpdateCommands>,
// }
// impl UpdateContext {
//     pub fn new(id: NodeId, position: Vector2, size: &Option<Vector2>) -> Self {
//         UpdateContext {
//             id,
//             position,
//             size: *size,
//             commands: vec![],
//         }
//     }
//
//     pub fn get_position(&self) -> Vector2 {
//         self.position
//     }
//
//     pub fn get_size(&self) -> Option<Vector2> {
//         self.size
//     }
//
//     pub fn add_command(&mut self, command: UpdateCommands) {
//         self.commands.insert(0, command);
//     }
//     pub fn get_commands(&self) -> &Vec<UpdateCommands> {
//         &self.commands
//     }
// }
//
// pub struct RenderContext {
//     pub id: NodeId,
//
//     position: Vector2,
//     size: Vector2,
// }
// impl RenderContext {
//     pub fn new(id: NodeId, position: Vector2, size: Vector2) -> Self {
//         RenderContext { id, position, size }
//     }
//
//     pub fn get_position(&self) -> Vector2 {
//         self.position
//     }
//
//     pub fn get_size(&self) -> Vector2 {
//         self.size
//     }
// }

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
