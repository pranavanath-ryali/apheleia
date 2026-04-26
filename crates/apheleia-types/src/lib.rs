use std::fmt;

pub type NodeId = usize;
pub type ExtensionId = usize;

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

pub trait IdGeneratorTrait<T> {
    fn new(start: T) -> Self;
    fn next(&mut self) -> T;
}

pub struct IdGenerator<T> {
    pub count: T,
}
impl Default for IdGenerator<usize> {
    fn default() -> Self {
        IdGenerator { count: 0 }
    }
}
impl IdGeneratorTrait<usize> for IdGenerator<usize> {
    fn new(start: usize) -> Self {
        IdGenerator { count: start }
    }

    fn next(&mut self) -> usize {
        self.count += 1;
        self.count
    }
}
