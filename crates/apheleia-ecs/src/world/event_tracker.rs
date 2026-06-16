use indexmap::IndexSet;
use log::info;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::types::{EventId, NodeId, Tag};


pub const RENDER_DIRTY: u16 = 0;

#[derive(Default)]
pub struct EventTracker {
    localevents_to_nodeid: FxHashMap<EventId, IndexSet<NodeId>>,
    local_events: FxHashMap<NodeId, SmallVec<[EventId; 8]>>,
    global_events: FxHashMap<Tag, SmallVec<[EventId; 8]>>,
}
impl EventTracker {
    pub fn add_local_event(&mut self, node_id: NodeId, event_id: EventId) {
        info!("[ECS] Marked NodeId: {} with event: {}", node_id, event_id);
        self.local_events
            .entry(node_id)
            .and_modify(|v| {
                if !v.contains(&event_id) {
                    v.push(event_id);
                }
            })
            .or_insert_with(|| {
                let mut vec: SmallVec<[EventId; _]> = Default::default();
                vec.push(event_id);
                vec
            });

        self.localevents_to_nodeid.entry(event_id).and_modify(|v| {
            v.insert(node_id);
        }).or_insert_with(|| {
            let mut set: IndexSet<NodeId> = IndexSet::default();
            set.insert(node_id);
            set
        });
    }

    pub fn is_local_event(&self, node_id: NodeId, event_id: EventId) -> bool {
        if let Some(local_events) = self.local_events.get(&node_id)
            && local_events.contains(&event_id)
        {
            return true;
        }
        false
    }

    pub fn get_nodes_with_event(&mut self, event_id: EventId) -> Option<&mut IndexSet<NodeId>> {
        if let Some(set) = self.localevents_to_nodeid.get_mut(&event_id) {
            return Some(set);
        }
        None
    }

    pub fn clear_local_events(&mut self) {
        self.local_events.clear();
    }

    pub fn add_global_event(&mut self, tag: Tag, event_id: EventId) {
        info!("[ECS] Marked Tag: {} with event: {}", tag, event_id);
        self.global_events
            .entry(tag)
            .and_modify(|v| {
                if !v.contains(&event_id) {
                    v.push(event_id);
                }
            })
            .or_insert_with(|| {
                let mut vec: SmallVec<[EventId; _]> = Default::default();
                vec.push(event_id);
                vec
            });
    }

    pub fn is_global_event(&self, tag: Tag, event_id: EventId) -> bool {
        if let Some(global_events) = self.global_events.get(&tag)
            && global_events.contains(&event_id)
        {
            return true;
        }
        false
    }

    pub fn clear_global_events(&mut self) {
        self.global_events.clear();
    }
}
