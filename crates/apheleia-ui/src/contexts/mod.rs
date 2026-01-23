use apheleia_core::types::vector::Vector2;
use crossterm::event::{KeyCode, KeyEvent};

use crate::{NodeId, node::data::DirtyRenderLevel, types::{EventData, EventType}};

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
    SetSize(Vector2),
    SetPosition(Vector2),

    MarkRenderDirty(DirtyRenderLevel),
}
pub struct EventUpdateContext {
    pub id: NodeId,

    position: Vector2,
    size: Option<Vector2>,

    pub event_data: EventData,

    pub commands: Vec<EventUpdateCommands>
}
impl EventUpdateContext {
    pub fn new(id: NodeId, position: &Vector2, size: &Option<Vector2>, event_data: EventData) -> Self{
        EventUpdateContext { id, position: *position, size: *size, event_data, commands: vec![] }
    }

    pub fn add_command(&mut self, command: EventUpdateCommands) {
        self.commands.insert(0, command);
    }
    pub fn get_commands(&self) -> &Vec<EventUpdateCommands> {
        &self.commands
    }
}

pub struct UpdateContext {
    pub id: NodeId,

    position: Vector2,
    size: Option<Vector2>,
}
impl UpdateContext {
    pub fn new(id: NodeId, position: Vector2, size: &Option<Vector2>) -> Self {
        UpdateContext { id, position, size: *size }
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    pub fn get_size(&self) -> Option<Vector2> {
        self.size
    }
}

pub struct RenderContext {
    pub id: NodeId,

    position: Vector2,
    size: Option<Vector2>,
}
impl RenderContext {
    pub fn new(id: NodeId, position: Vector2, size: &Option<Vector2>) -> Self {
        RenderContext { id, position, size: *size }
    }

    pub fn get_position(&self) -> Vector2 {
        self.position
    }

    pub fn get_size(&self) -> Option<Vector2> {
        self.size
    }
}
