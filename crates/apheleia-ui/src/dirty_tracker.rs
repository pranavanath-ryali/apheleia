use indexmap::IndexSet;
use log::info;

use crate::types::NodeId;

#[derive(Default)]
pub struct DirtyTracker {
    dirty_setups: IndexSet<NodeId>,
    dirty_updates: IndexSet<NodeId>,
    dirty_renders: IndexSet<NodeId>,
}

impl DirtyTracker {
    pub fn add_setup(&mut self, id: NodeId) {
        self.dirty_setups.insert(id);
    }

    pub fn add_update(&mut self, id: NodeId) {
        self.dirty_updates.insert(id);
    }
    pub fn add_render(&mut self, id: NodeId) {
        info!("Node {} is marked Dirty!", id);
        self.dirty_renders.insert(id);
    }

    pub fn is_setup_empty(&self) -> bool {
        self.dirty_setups.is_empty()
    }
    pub fn is_update_empty(&self) -> bool {
        self.dirty_updates.is_empty()
    }
    pub fn is_render_empty(&self) -> bool {
        self.dirty_renders.is_empty()
    }

    pub fn iter_setup(&self) -> indexmap::set::Iter<'_, usize> {
        self.dirty_setups.iter()
    }
    pub fn iter_update(&self) -> indexmap::set::Iter<'_, usize> {
        self.dirty_updates.iter()
    }
    pub fn iter_render(&self) -> indexmap::set::Iter<'_, usize> {
        self.dirty_renders.iter()
    }

    pub fn clear_setups(&mut self) {
        self.dirty_setups.clear();
    }
    pub fn clear_update(&mut self) {
        self.dirty_updates.clear();
    }
    pub fn clear_render(&mut self) {
        self.dirty_renders.clear();
    }
}
