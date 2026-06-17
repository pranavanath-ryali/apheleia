use apheleia_core::buffer::Buffer;
use apheleia_ecs::{
    events::RenderDirty, systems::system::SystemParam, types::{NodeId, SystemRunStage}, world::World
};

pub struct SystemContext<'w> {
    world: &'w mut World,
}
impl<'w> SystemContext<'w> {
    pub(crate) fn new(world: &'w mut World) -> Self {
        Self { world }
    }

    pub fn mark_render_dirty(&mut self, id: NodeId) {
        self.world.add_event(id, RenderDirty);
    }

    pub fn get_buffer(&mut self, id: NodeId) -> Option<&mut Buffer> {
        if self.world.current_stage != SystemRunStage::Render {
            panic!("Trying to access Buffer but current stage is not Render");
        }

        self.world.get_buffer(id)
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
