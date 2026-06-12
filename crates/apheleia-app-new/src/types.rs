use apheleia_core::types::Vec2;
use crossterm::event::{KeyEvent, MouseEvent};

#[derive(Default)]
pub enum EventData {
    Resize(Vec2),
    Keys(KeyEvent),
    Mouse(MouseEvent),

    FocusGained,
    FocusLost,

    #[default]
    None,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum EventType {
    None = 0,

    Resize = 1,
    Keys = 2,
    Mouse = 3,

    FocusGained = 4,
    FocusLost = 5,
}
impl EventType {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

pub const EVENT_RESIZE: usize = 1;
pub const EVENT_KEYS: usize = 2;
pub const EVENT_MOUSE: usize = 3;
pub const EVENT_FOCUS_GAINED: usize = 4;
pub const EVENT_FOCUS_LOST: usize = 5;
