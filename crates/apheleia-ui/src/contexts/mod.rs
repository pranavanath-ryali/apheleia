use apheleia_core::types::vector::Vector2;
use crossterm::event::{KeyCode, KeyEvent};

use crate::{NodeId, types::EventType};

pub struct RenderContext {
    pub position: Vector2,
    pub size: Vector2,
}

pub struct UpdateContext {
    pub position: Vector2,
    pub size: Option<Vector2>,
}

pub enum EventData {
    Resize(Vector2),
    Keys(KeyEvent),
}

// pub struct EventContext {
//     pub data: EventData,
//     pub position: Vector2,
//     pub size: Option<Vector2>,
// }

// New Code
// pub struct InitalCallContext {
//     position: Option<Vector2>,
//     size: Option<Vector2>,
// }
// impl InitalCallContext {
//     pub fn new() -> Self {
//         InitalCallContext { position: None, size: None }
//     }
//
//     pub fn set_position(&mut self, position: &Vector2) {
//         self.position = Some(*position);
//     }
//     pub fn get_position(&self) -> &Option<Vector2> {
//         &self.position
//     }
//
//     pub fn set_size(&mut self, size: &Vector2) {
//         self.size = Some(*size);
//     }
//     pub fn get_size(&self) -> &Option<Vector2> {
//         &self.size
//     }
// }

pub enum IntialCallCommands {
    SetSize(Vector2),

    RegisterForUpdate,
    RegisterForEvent(EventType),
}
pub struct InitialCallContext {
    position: Vector2,
    size: Option<Vector2>,

    commands: Vec<IntialCallCommands>
}
impl InitialCallContext {
    pub fn new(position: &Vector2, size: &Option<Vector2>) -> Self {
        InitialCallContext { position: *position, size: *size, commands: vec![] }
    }

    pub fn add_command(&mut self, command: IntialCallCommands) {
        self.commands.insert(0, command);
    }
    pub fn get_commands(&self) -> &Vec<IntialCallCommands> {
        &self.commands
    }
}

pub enum EventUpdateCommands {

}
pub struct EventUpdateContext {
    position: Vector2,
    size: Option<Vector2>,

    event_data: EventData,

    commands: Vec<EventUpdateCommands>
}
impl EventUpdateContext {
    pub fn new(position: &Vector2, size: &Option<Vector2>, event_data: EventData) -> Self{
        EventUpdateContext { position: *position, size: *size, event_data, commands: vec![] }
    }

    pub fn add_command(&mut self, command: EventUpdateCommands) {
        self.commands.insert(0, command);
    }
    pub fn get_commands(&self) -> &Vec<EventUpdateCommands> {
        &self.commands
    }
}
