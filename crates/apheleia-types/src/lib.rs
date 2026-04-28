use std::any::Any;

use crossterm::event::{KeyEvent, MouseEvent};

use crate::{vec2::Vec2, views::SystemContext};

pub mod id_generator;
pub mod vec2;
pub mod views;
pub mod world_access;

pub type NodeId = usize;
pub type ExtensionId = usize;

pub trait Extension: Any {}
pub trait Resource: Any {}

pub trait ContextCommand {
    fn execute(&mut self, ctx: &mut SystemContext);
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum EventType {
    None,

    Resize,
    Keys,
    FocusGained,
    FocusLost,
    Mouse,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum UpdateType {
    ConstantUpdate,
    Event(EventType),
    Render,
}

#[derive(Default)]
pub enum EventData {
    Resize(Vec2),
    Keys(KeyEvent),
    Mouse(MouseEvent),

    #[default]
    None,
}

#[derive(Clone, Copy)]
pub enum DirtyRenderLevel {
    SimpleDirty,  // Rerender node alone. Leave already defined attributes unless specified
    SubtreeDirty, // Rerender entire subtree which includes the node and including all its children
}
