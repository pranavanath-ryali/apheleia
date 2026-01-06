use apheleia_core::types::vector::Vector2;
use crossterm::event::{KeyCode, KeyEvent};

use crate::{NodeId, types::EventType};

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
