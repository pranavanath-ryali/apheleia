use apheleia_core::types::vector::Vector2;
use crossterm::event::KeyEvent;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum EventType {
    Resize,
    Keys,
}

#[derive(Hash, PartialEq, Eq)]
pub enum UpdateTypeNode {
    ConstantUpdate,
    Event(EventType),
}

pub enum EventData {
    Resize(Vector2),
    Keys(KeyEvent),
}
