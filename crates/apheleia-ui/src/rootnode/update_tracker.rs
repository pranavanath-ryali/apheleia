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

    pub fn iter(&self, update_type: UpdateTypeNode) -> Option<indexmap::set::Iter<'_, usize>> {
        self.id_update.get(&update_type).map(|set| set.iter())
    }
}
