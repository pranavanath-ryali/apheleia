use crate::{
    stores::system::function_system::{IntoSystem, System}, traits::context_command::ContextCommand, types::SystemRunStage, world::World
};
use std::fmt::Debug;

/// A [`ContextCommand`] that registers a system with the [`World`], to be run
/// during a specific [`SystemRunStage`] and ordered by priority.
///
/// Wraps a boxed [`System`] along with the stage it should run in and its
/// priority relative to other systems in that stage.
pub struct AddSystem {
    /// The stage during which this system should run.
    pub stage: SystemRunStage,
    /// The priority of this system within its stage. Lower values typically
    /// run before higher ones (see [`World::add_system_boxed`] for exact ordering).
    pub priority: u16,
    /// The boxed system to be added.
    pub system: Box<dyn System>,
}

impl Debug for AddSystem {
    /// Formats the command for debugging purposes.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AddSystem")
            .field("stage", &self.stage)
            .field("priority", &self.priority)
            .field("type_id", &self.system.id())
            .finish()
    }
}

impl AddSystem {
    /// Creates a new boxed [`AddSystem`] command from any value implementing
    /// [`IntoSystem`].
    ///
    /// # Arguments
    ///
    /// * `stage` - The stage during which the system should run.
    /// * `priority` - The priority of the system within its stage.
    /// * `system` - The system (or system-like value) to add, converted into
    ///   a boxed [`System`] via [`IntoSystem::into_system`].
    pub fn new<Params>(
        stage: SystemRunStage,
        priority: u16,
        system: impl IntoSystem<Params>,
    ) -> Box<Self> {
        Box::new(Self {
            stage,
            priority,
            system: system.into_system(),
        })
    }
}

impl ContextCommand for AddSystem {
    /// Executes the command, registering the system with the [`World`] under
    /// the specified stage and priority.
    fn execute(self: Box<Self>, world: &mut World) {
        world.add_system_boxed(self.stage, self.priority, self.system);
    }
}
