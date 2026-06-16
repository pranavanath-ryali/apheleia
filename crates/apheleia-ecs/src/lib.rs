mod id_generator;

pub(crate) mod buffer_store;

pub mod extensions;
pub mod nodedata;
pub mod resources;
pub mod systems;
pub mod utils;
pub mod world;
pub mod commands;
pub mod tags;
pub mod events;

pub mod types {
    pub type NodeId = usize;
    pub type ExtensionId = usize;
    pub type SystemId = usize;
}

pub mod constants {
    pub const MAX_NODES: usize = 512;
    pub const MAX_EXTENSIONS: usize = 1024;
    pub const MAX_SYSTEMS: usize = u8::MAX as usize;

    pub const SYSTEMS_MAX_PRIORITY_CHANGE: u8 = 15;

    // System priority
    pub const FIRST: u8 = 0;
    pub const PRE_STAGE: u8 = 50;
    pub const STAGE: u8 = 120;
    pub const POST_STAGE: u8 = 200;
    pub const LAST: u8 = 255;
}
