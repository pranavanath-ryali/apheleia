use apheleia_ecs_new::world::World;

use crate::app::App;

pub struct SystemContext {
    world: *mut World,
}
impl SystemContext {
    pub fn new(world: *mut World) -> Self {
        Self {
            world
        }
    }
}
