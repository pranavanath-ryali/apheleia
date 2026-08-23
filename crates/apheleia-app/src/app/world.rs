use apheleia_ecs::{
    stores::system::function_system::IntoSystem,
    traits::{context_command::ContextCommand, resource::Resource},
    types::SystemRunStage,
    world::World,
};

use crate::app::App;

impl App {
    pub(crate) fn get_world(&mut self) -> &mut World {
        &mut self.world
    }
    pub(crate) fn push_command(&mut self, command: Box<dyn ContextCommand>) {
        self.world.add_command(command);
    }

    pub fn add_resource<R: Resource>(mut self, resource: R) -> Self {
        self.world.add_resource(resource);
        self
    }

    pub fn add_system<Params: 'static>(
        mut self,
        stage: SystemRunStage,
        priority: u16,
        system: impl IntoSystem<Params>,
    ) -> Self {
        self.world.add_system(stage, priority, system);
        self
    }
}
