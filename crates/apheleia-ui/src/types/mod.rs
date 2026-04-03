use std::fmt;

use apheleia_core::types::vector::Vector2;
use crossterm::event::KeyEvent;

pub type NodeId = usize;
pub type ExtensionId = usize;
pub type SystemId = usize;

#[derive(Hash, PartialEq, Eq, Clone, Copy, fmt::Debug)]
pub enum EventType {
    None,

    Resize,
    Keys,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum UpdateType {
    ConstantUpdate,
    Event(EventType),
    Render,
}

#[derive(fmt::Debug, Default)]
pub enum EventData {
    Resize(Vector2),
    Keys(KeyEvent),

    #[default]
    None,
}

#[derive(Clone, Copy)]
pub enum DirtyRenderLevel {
    SimpleDirty,  // Rerender node alone. Leave already defined attributes unless specified
    SubtreeDirty, // Rerender entire subtree which includes the node and including all its children
}
