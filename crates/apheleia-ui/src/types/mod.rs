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

#[derive(Clone, Copy)]
pub enum DirtyRenderLevel {
    SimpleDirty, // Rerender node alone. Leave already defined attributes unless specified
    SubtreeDirty, // Rerender entire subtree which includes the node and including all its children
}
