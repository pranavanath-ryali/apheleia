use std::fmt::Debug;

use apheleia_ecs::{
    command::ContextCommand,
    systems::{
        stages::SystemRunStage,
        system::{IntoSystem, System},
    },
    world::World,
};

pub struct AddSystem {
    pub stage: SystemRunStage,
    pub priority: u8,
    pub system: Box<dyn System>,
}
impl Debug for AddSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AddSystem")
            .field("stage", &self.stage)
            .field("priority", &self.priority)
            .finish()
    }
}
impl AddSystem {
    pub fn new<Params>(
        stage: SystemRunStage,
        priority: u8,
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
    fn execute(self: Box<Self>, world: &mut World) {
        world.add_system_boxed(self.stage, self.priority, self.system);
    }
}
