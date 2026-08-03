use std::marker::PhantomData;

use apheleia_ecs::{
    stores::{events::EventRegistry, tag::TagRegistry},
    traits::{event_marker::EventMarker, system_param::SystemParam, tag::TagTrait},
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

    pub fn mark_tags<T: TagTrait>(&mut self, _tag: T) {
        let Some(nodes) = self
            .world
            .get_resource_mut::<TagRegistry>()
            .unwrap()
            .get_nodes_with_tag::<T>() else {
                return;
            };
        let nodes = nodes.clone();

        nodes.iter().for_each(|&id| self.mark(id));
    }

    pub fn mark(&mut self, id: NodeId) {
        self.world
            .get_resource_mut::<EventRegistry>()
            .unwrap()
            .add_local_event::<E>(id);
    }

    pub fn mark_subtree(&mut self, id: NodeId) {
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
        self.mark_subtree(parents[0]);
    }
}

impl<E: EventMarker> SystemParam for EventEmitter<'static, E> {
    unsafe fn fetch<'w>(world: *mut apheleia_ecs::world::World) -> Option<Self> {
        let world = unsafe { &mut *world };
        Some(EventEmitter::new(world))
    }
}
