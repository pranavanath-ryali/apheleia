use apheleia_ecs::{
    systems::system::SystemParam, world::World
};

pub struct SystemContext<'w> {
    world: &'w mut World,
}
impl<'w> SystemContext<'w> {
    pub(crate) fn new(world: &'w mut World) -> Self {
        Self { world }
    }
}

impl SystemParam for SystemContext<'static> {
    unsafe fn fetch<'w>(world: *mut World) -> Option<Self> {
        let world = unsafe {
            &mut *world
        };

        Some(SystemContext::new(world))
    }
}
