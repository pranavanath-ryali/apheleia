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

// I dont know what to name this...
#[derive(Default)]
pub struct TBLRValues {
    pub top: u16,
    pub bottom: u16,
    pub left: u16,
    pub right: u16,
}

#[derive(Default)]
pub struct Anchors {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

pub struct Layout {
    pub anchors: Anchors,

    pub preffered_width: Option<u16>,
    pub preffered_height: Option<u16>,

    pub fill_width: bool,
    pub fill_height: bool,

    pub margin: TBLRValues,
    pub padding: TBLRValues,
}
