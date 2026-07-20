use std::marker::PhantomData;

use apheleia_ecs::{stores::events::EventRegistry, traits::{event_marker::EventMarker, system_param::SystemParam, tag::TagTrait}};

pub struct GlobalEmitter<'w, T, E> where T: TagTrait, E: EventMarker {
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

impl<T, E> SystemParam for GlobalEmitter<'static, T, E> where T: TagTrait, E: EventMarker {
    unsafe fn fetch<'w>(world: *mut apheleia_ecs::world::World) -> Option<Self> {
        let world = unsafe { &mut *world };
        Some(GlobalEmitter::new(
            world.get_resource_mut::<EventRegistry>().unwrap(),
        ))
    }
}
