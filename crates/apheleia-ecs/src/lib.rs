pub mod command;
pub mod constants;
pub mod extensions;
mod id_generator;
mod nodedata_store;
pub mod resources;
pub mod systems;
pub(crate) mod tag;
pub mod types;
pub mod world;
mod buffer_store;
pub mod utils;
pub mod event_tracker;

pub type NodeId = usize;
pub type ExtensionId = usize;
pub type SystemId = usize;

pub type Tag = usize;

pub const MAX_NODES: usize = 128;
pub const MAX_EXTENSIONS: usize = 64;
pub const MAX_EXTENSIONS_PER_NODE: usize = 16;
