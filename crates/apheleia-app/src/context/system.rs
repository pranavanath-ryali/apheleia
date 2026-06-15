use apheleia_core::buffer::Buffer;
use apheleia_ecs::{
    NodeId, event_tracker::RENDER_DIRTY, systems::stages::SystemRunStage, world::World,
};

use crate::app::App;

pub struct SystemContext {
    world: *mut World,
}
impl SystemContext {
    pub(crate) fn new(world: *mut World) -> Self {
        Self { world }
    }

    pub fn mark_render_dirty(&mut self, id: NodeId) {
        let world = unsafe { &mut *self.world };
        world.add_local_event(id, RENDER_DIRTY);
    }

    pub fn get_buffer(&mut self, id: NodeId) -> Option<&mut Buffer> {
        let world = unsafe { &mut *self.world };

        if world.current_stage != SystemRunStage::Render {
            panic!("Trying to access Buffer but current stage is not Render");
        }

        world.get_buffer(id)
    }
}
