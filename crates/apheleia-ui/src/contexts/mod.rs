use apheleia_core::types::vector::Vector2;
use crossterm::event::KeyCode;

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
    Keys(KeyCode),
}

pub struct EventContext {
    pub data: EventData,
    pub position: Vector2,
    pub size: Option<Vector2>,
}
