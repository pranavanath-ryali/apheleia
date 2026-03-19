pub mod builder;
pub mod contexts;
pub mod node;
pub mod rootnode;
pub mod types;

pub type NodeId = usize;

pub const FAKE_NODEID: NodeId = 0;
pub const MAX_NODES: NodeId = 1000;

pub use crossterm::event::*;
