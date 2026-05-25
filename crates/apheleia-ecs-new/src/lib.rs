pub(crate) mod id_generator;

pub mod world;
pub mod types;
mod nodedata_store;
pub mod extensions;
pub mod resources;
pub mod systems;

pub type NodeId = usize;
pub type ExtensionId = usize;
pub type SystemId = usize;

pub const MAX_NODES: usize = 128;
pub const MAX_EXTENSIONS: usize = 64;
pub const MAX_EXTENSIONS_PER_NODE: usize = 16;
