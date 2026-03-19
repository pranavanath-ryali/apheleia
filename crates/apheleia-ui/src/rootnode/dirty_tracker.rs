use indexmap::IndexSet;

use crate::types::NodeId;

pub struct DirtyTracker {
    dirty_updates: IndexSet<NodeId>,
    dirty_renders: IndexSet<NodeId>,
}
impl Default for DirtyTracker {
    fn default() -> Self {
        DirtyTracker {
            dirty_updates: IndexSet::default(),
            dirty_renders: IndexSet::default(),
        }
    }
}

impl DirtyTracker {
    pub fn add_update(&mut self, id: NodeId) {
        self.dirty_updates.insert(id);
    }
    pub fn add_render(&mut self, id: NodeId) {
        self.dirty_renders.insert(id);
    }

    pub fn iter_update(&self) -> indexmap::set::Iter<'_, usize> {
        self.dirty_updates.iter()
    }
    pub fn iter_render(&self) -> indexmap::set::Iter<'_, usize> {
        self.dirty_renders.iter()
    }

    pub fn clear_update(&mut self) {
        self.dirty_updates.clear();
    }
    pub fn clear_render(&mut self) {
        self.dirty_renders.clear();
    }
}
