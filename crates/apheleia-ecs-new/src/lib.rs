pub mod id_generator;
pub mod constants;
pub mod extensions;
mod nodedata_store;
pub mod resources;
pub mod systems;
pub mod types;
pub mod world;
pub(crate) mod tag;
pub mod command;

pub type NodeId = usize;
pub type ExtensionId = usize;
pub type SystemId = usize;

pub const MAX_NODES: usize = 128;
pub const MAX_EXTENSIONS: usize = 64;
pub const MAX_EXTENSIONS_PER_NODE: usize = 16;
