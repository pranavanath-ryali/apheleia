pub mod rootnode;
pub mod node;
pub mod commands;
pub mod contexts;

pub type NodeId = usize;

pub const FAKE_NODEID: NodeId = 0;
pub const MAX_NODES: NodeId = 1000;
