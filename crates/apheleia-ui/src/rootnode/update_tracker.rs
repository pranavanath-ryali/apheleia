use std::collections::HashMap;

use indexmap::{IndexSet, indexset};

use crate::types::{NodeId, UpdateTypeNode};

#[derive(Default)]
pub struct UpdateTracker {
    id_update: HashMap<UpdateTypeNode, IndexSet<NodeId>>,
}
impl UpdateTracker {
    pub fn add_node(&mut self, id: NodeId, update_type: UpdateTypeNode) {
        self.id_update
            .entry(update_type)
            .or_insert(indexset![])
            .insert(id);
    }

    pub fn is_empty(&self, update_type: UpdateTypeNode) -> bool {
        if let Some(ids) = self.id_update.get(&update_type) {
            if ids.is_empty() {
                return false;
            }
            return true;
        }
        false
    }

    pub fn iter(&self, update_type: UpdateTypeNode) -> Option<indexmap::set::Iter<'_, usize>> {
        if let Some(ids) = self.id_update.get(&update_type) {
            return Some(ids.into_iter());
        }
        None
    }
}
