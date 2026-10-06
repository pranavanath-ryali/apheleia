pub mod id_generator;

pub mod stores;
pub mod commands;
pub mod nodedata;
pub mod resources;
pub mod params;
pub mod utils;
pub mod world;
pub mod runtime_expressions;
pub mod traits;

pub mod types {
    pub type NodeId = usize;
    pub(crate) type ExtensionId = usize;
    pub(crate) type SystemId = usize;

    pub type EventType = u8;

    /// Defines the stage at which a system runs and what capabilities it can access
    ///
    /// Each tick follows a pipeline:
    ///
    /// ```text
    ///     Event -> Update -> Render
    /// ```
    ///
    /// # Stages
    /// 1. [`Event`]: Can access events provided by _crossterm_ which include [`Keys`], [`Mouse`],
    ///    [`Resize`], etc.
    /// 2. [`Update`]: No extra capabilities. This is the stage where most systems are expected to run in.
    /// 3. [`Render`]: Can access the [`NodeBuffer`] for rendering capability.
    #[derive(PartialEq, Eq, Hash, Debug)]
    pub enum SystemRunStage {
        Event,
        Update,
        Render,
        RenderFlip,
    }
}

pub mod constants {
    pub(crate) const MAX_NODES: usize = 512;
    pub(crate) const MAX_EXTENSIONS: usize = 1024;
    pub(crate) const MAX_SYSTEMS: usize = u16::MAX as usize;

    pub const SYSTEMS_MAX_PRIORITY_CHANGE: u16 = 32;

    /// The very first priority slot. Reserved for foundational systems that must run before everything
    /// else — such as layout engines, animation drivers, or third-party crates that produce data other
    /// systems depend on. Prefer [`PRE_STAGE`] or [`STAGE`] for application logic.
    pub const FIRST: u16 = 0;

    /// Runs before the main [`STAGE`]
    pub const PRE_STAGE: u16 = 127;

    /// The default priority for systems. Most app logic should run here unless there is a specific ordering requirement.
    pub const STAGE: u16 = 255;

    /// Runs after the main [`STAGE`]
    pub const POST_STAGE: u16 = 383;

    /// The very last priority slot. Reserved for systems that must run after everything else — such as
    /// rendering, debug overlays, or third-party crates that consume fully-resolved state.
    pub const LAST: u16 = 479;

    pub const EVENT_NONE: u8 = 0;
    pub const EVENT_RESIZE: u8 = 1;
    pub const EVENT_KEYS: u8 = 2;
    pub const EVENT_MOUSE: u8 = 3;
    pub const EVENT_FOCUS_GAINED: u8 = 4;
    pub const EVENT_FOCUS_LOST: u8 = 5;
}
