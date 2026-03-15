use indexmap::IndexSet;

use crate::NodeId;

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
    fn add_update(&mut self, id: NodeId) {
        self.dirty_updates.insert(id);
    }
    fn add_render(&mut self, id: NodeId) {
        self.dirty_renders.insert(id);
    }

    fn iter_update(&self) -> indexmap::set::Iter<'_, usize> {
        self.dirty_updates.iter()
    }
    fn iter_render(&self) -> indexmap::set::Iter<'_, usize> {
        self.dirty_renders.iter()
    }

    fn clear_update(&mut self) {
        self.dirty_updates.clear();
    }
    fn clear_render(&mut self) {
        self.dirty_renders.clear();
    }
}
