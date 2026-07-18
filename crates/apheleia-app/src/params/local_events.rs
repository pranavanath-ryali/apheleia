use std::marker::PhantomData;

use apheleia_ecs::{systems::system::SystemParam, types::NodeId};

use crate::resources::event_tracker::{EventMarker, EventRegistry};

pub struct EventEmitter<'w, E: EventMarker> {
    registry: &'w mut EventRegistry,
    _marker: PhantomData<E>,
}
impl<'w, E: EventMarker> EventEmitter<'w, E> {
    pub fn new(registry: &'w mut EventRegistry) -> Self {
        Self {
            registry,
            _marker: PhantomData
        }
    }

    pub fn mark(&mut self, id: NodeId) {
        self.registry.add_local_event::<E>(id);
    }
}

impl<E: EventMarker> SystemParam for EventEmitter<'static, E> {
    unsafe fn fetch<'w>(world: *mut apheleia_ecs::world::World) -> Option<Self> {
        let world = unsafe { &mut *world };
        Some(EventEmitter::new(world.get_resource_mut::<EventRegistry>().unwrap()))
    }
}
