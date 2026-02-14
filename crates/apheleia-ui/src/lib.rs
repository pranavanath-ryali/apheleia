pub mod rootnode;
pub mod node;
pub mod contexts;
pub mod types;
pub mod utils;

pub type NodeId = usize;

pub const FAKE_NODEID: NodeId = 0;
pub const MAX_NODES: NodeId = 1000;

pub use crossterm::event::*;
