use core::fmt;
use std::cmp;

/// A simple struct that stores x, y: u16
#[derive(Clone, Copy, fmt::Debug, cmp::PartialEq)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}
impl Vec2 {
    /// Returns a [`Vec2`] where x = y = 0
    pub fn zero() -> Self {
        Vec2 { x: 0, y: 0 }
    }
}
