use std::collections::HashMap;

use indexmap::{IndexSet, indexset};

use crate::types::{NodeId, UpdateTypeNode};

pub struct UpdateTracker {
    id_update: HashMap<UpdateTypeNode, IndexSet<NodeId>>,
}
impl Default for UpdateTracker {
    fn default() -> Self {
        UpdateTracker {
            id_update: HashMap::new(),
        }
    }
}
impl UpdateTracker {
    pub fn add_node(&mut self, id: NodeId, update_type: UpdateTypeNode) {
        if !self.id_update.contains_key(&update_type) {
            self.id_update.insert(update_type, indexset![]);
        }
        self.id_update.get_mut(&update_type).unwrap().insert(id);
    }

    pub fn iter(&self, update_type: UpdateTypeNode) -> Option<indexmap::set::Iter<'_, usize>> {
        if let Some(set) = self.id_update.get(&update_type) {
            Some(set.iter())
        } else {
            None
        }
    }
}
