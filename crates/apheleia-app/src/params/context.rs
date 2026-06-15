use apheleia_ecs::systems::system::SystemParam;

use crate::context::system::SystemContext;

impl SystemParam for SystemContext {
    unsafe fn fetch(world: *mut apheleia_ecs::world::World) -> Option<Self> {
        Some(SystemContext::new(world))
    }
}
