use std::{any::TypeId, collections::HashSet, fmt::Debug};

use apheleia_ecs::{resources::Resource, tags::TagTrait, types::NodeId};
use rustc_hash::FxHashMap;

pub trait EventMarker: Debug + 'static {}

#[derive(Debug)]
pub struct RenderDirty;
impl EventMarker for RenderDirty {}

#[derive(Debug, Default)]
pub struct EventRegistry {
    localevents_to_ids: FxHashMap<TypeId, HashSet<NodeId>>,
    globalevents_to_tags: FxHashMap<TypeId, HashSet<TypeId>>,
}
impl EventRegistry {
    pub fn add_local_event<E: EventMarker>(&mut self, node: NodeId) {
        let type_id = TypeId::of::<E>();
        self.localevents_to_ids
            .entry(type_id)
            .and_modify(|set| {
                set.insert(node);
            })
            .or_insert_with(|| {
                let mut set: HashSet<NodeId> = Default::default();
                set.insert(node);
                set
            });
    }

    pub fn get_local_events<E: EventMarker>(&self, _event: E) -> Option<&HashSet<usize>> {
        self.localevents_to_ids.get(&TypeId::of::<E>())
    }

    pub fn is_local_event<E: EventMarker>(&self, id: NodeId) -> bool {
        if let Some(set) = self.localevents_to_ids.get(&TypeId::of::<E>()) {
            return set.contains(&id);
        }
        false
    }

    pub fn add_global_event<T: TagTrait, E: EventMarker>(&mut self) {
        let event = TypeId::of::<E>();
        let tag = TypeId::of::<T>();

        self.globalevents_to_tags
            .entry(event)
            .and_modify(|set| {
                set.insert(tag);
            })
            .or_insert_with(|| {
                let mut set: HashSet<TypeId> = Default::default();
                set.insert(tag);
                set
            });
    }

    pub fn is_global_event<T: TagTrait, E: EventMarker>(&self) -> bool {
        if let Some(set) = self.globalevents_to_tags.get(&TypeId::of::<E>()) {
            return set.contains(&TypeId::of::<T>());
        }
        false
    }

    pub fn clear(&mut self) {
        self.localevents_to_ids.clear();
        self.globalevents_to_tags.clear();
    }
}
impl Resource for EventRegistry {}
