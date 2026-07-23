use std::marker::PhantomData;

use apheleia_ecs::{
    stores::events::EventRegistry,
    traits::{event_marker::EventMarker, system_param::SystemParam},
    types::NodeId,
    world::World,
};
use tree_ds::prelude::Tree;

pub struct EventEmitter<'w, E: EventMarker> {
    world: &'w mut World,
    _marker: PhantomData<E>,
}
impl<'w, E: EventMarker> EventEmitter<'w, E> {
    pub fn new(world: &'w mut World) -> Self {
        Self {
            world,
            _marker: PhantomData,
        }
    }

    pub fn mark(&mut self, id: NodeId) {
        let relations = self.world.get_relations();
        let Some(subtree) = relations.get_subtree(&id, None).ok() else {
            return;
        };

        subtree
            .traverse(&id, tree_ds::prelude::TraversalStrategy::PreOrder)
            .unwrap()
            .iter()
            .filter(|&&id| id != 0)
            .for_each(|&id| {
                self.world
                    .get_resource_mut::<EventRegistry>()
                    .unwrap()
                    .add_local_event::<E>(id);
            });
    }

    pub fn mark_parent(&mut self, id: NodeId) {
        let relations = self.world.get_relations();
        let Some(parents) = relations.get_ancestor_ids(&id).ok() else {
            return;
        };
        self.mark(parents[0]);
    }
}

impl<E: EventMarker> SystemParam for EventEmitter<'static, E> {
    unsafe fn fetch<'w>(world: *mut apheleia_ecs::world::World) -> Option<Self> {
        let world = unsafe { &mut *world };
        Some(EventEmitter::new(world))
    }
}
