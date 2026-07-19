use std::marker::PhantomData;

use apheleia_ecs::{systems::system::SystemParam, tags::TagTrait, types::NodeId};

use crate::resources::event_tracker::{EventMarker, EventRegistry};

pub struct GlobalEmitter<'w, T: TagTrait, E: EventMarker> {
    registry: &'w mut EventRegistry,
    _marker: PhantomData<(T, E)>,
}
impl<'w, T: TagTrait, E: EventMarker> GlobalEmitter<'w, T, E> {
    pub fn new(registry: &'w mut EventRegistry) -> Self {
        Self {
            registry,
            _marker: PhantomData,
        }
    }

    pub fn emit(&mut self) {
        self.registry.add_global_event::<T, E>();
    }
}

impl<T: TagTrait, E: EventMarker> SystemParam for GlobalEmitter<'static, T, E> {
    unsafe fn fetch<'w>(world: *mut apheleia_ecs::world::World) -> Option<Self> {
        let world = unsafe { &mut *world };
        Some(GlobalEmitter::new(world.get_resource_mut::<EventRegistry>().unwrap()))
    }
}
