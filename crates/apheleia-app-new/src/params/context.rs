use apheleia_ecs_new::systems::system::SystemParam;

use crate::context::system::SystemContext;

impl SystemParam for SystemContext {
    unsafe fn fetch(world: *mut apheleia_ecs_new::world::World) -> Option<Self> {
        Some(SystemContext::new(world))
    }
}
