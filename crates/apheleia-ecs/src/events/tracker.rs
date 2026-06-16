use std::any::TypeId;

use indexmap::IndexSet;
use log::info;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::{
    events::EventTrait,
    types::{EventId, NodeId, Tag},
};

#[derive(Default)]
pub struct EventTracker {
    localevents_to_nodeid: FxHashMap<TypeId, IndexSet<NodeId>>,
    local_events: FxHashMap<NodeId, SmallVec<[TypeId; 8]>>,
    global_events: FxHashMap<Tag, SmallVec<[TypeId; 8]>>,
}
impl EventTracker {
    pub fn add_local_event<E: EventTrait>(&mut self, node_id: NodeId, _event: E) {
        let event = TypeId::of::<E>();
        info!("[ECS] Marked NodeId: {} with event: {:#?}", node_id, _event);
        self.local_events
            .entry(node_id)
            .and_modify(|v| {
                if !v.contains(&event) {
                    v.push(event);
                }
            })
            .or_insert_with(|| {
                let mut vec: SmallVec<[TypeId; _]> = Default::default();
                vec.push(event);
                vec
            });

        self.localevents_to_nodeid
            .entry(event)
            .and_modify(|v| {
                v.insert(node_id);
            })
            .or_insert_with(|| {
                let mut set: IndexSet<NodeId> = IndexSet::default();
                set.insert(node_id);
                set
            });
    }

    pub fn is_local_event<E: EventTrait>(&self, node_id: NodeId, _event: E) -> bool {
        let event = TypeId::of::<E>();
        if let Some(local_events) = self.local_events.get(&node_id)
            && local_events.contains(&event)
        {
            return true;
        }
        false
    }

    pub fn get_nodes_with_event<E: EventTrait>(&mut self) -> Option<&mut IndexSet<NodeId>> {
        let event = TypeId::of::<E>();
        if let Some(set) = self.localevents_to_nodeid.get_mut(&event) {
            return Some(set);
        }
        None
    }

    pub fn clear_local_events(&mut self) {
        self.local_events.clear();
    }

    pub fn add_global_event<E: EventTrait>(&mut self, tag: Tag, _event: E) {
        let event = TypeId::of::<E>();
        info!("[ECS] Marked Tag: {} with event: {:#?}", tag, _event);
        self.global_events
            .entry(tag)
            .and_modify(|v| {
                if !v.contains(&event) {
                    v.push(event);
                }
            })
            .or_insert_with(|| {
                let mut vec: SmallVec<[TypeId; _]> = Default::default();
                vec.push(event);
                vec
            });
    }

    pub fn is_global_event<E: EventTrait>(&self, tag: Tag, _event: E) -> bool {
        let event = TypeId::of::<E>();
        if let Some(global_events) = self.global_events.get(&tag)
            && global_events.contains(&event)
        {
            return true;
        }
        false
    }

    pub fn clear_global_events(&mut self) {
        self.global_events.clear();
    }
}
