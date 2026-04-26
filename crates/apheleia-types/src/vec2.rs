use core::fmt;

#[derive(Clone, Copy, fmt::Debug)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}
impl Vec2 {
    pub fn zero() -> Self {
        Vec2 { x: 0, y: 0 }
    }
}
