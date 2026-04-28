use std::mem::take;

use apheleia_types::NodeId;
use indexmap::IndexSet;

#[derive(Default)]
pub struct DirtyTracker {
    pub setup_dirty: IndexSet<NodeId>,
    pub render_dirty: IndexSet<NodeId>,
    pub update_dirty: IndexSet<NodeId>,
}
impl DirtyTracker {
    pub fn is_setup_empty(&self) -> bool {
        self.setup_dirty.is_empty()
    }
    pub fn add_setup(&mut self, id: NodeId) {
        self.setup_dirty.insert(id);
    }
    pub fn take_setup(&mut self) -> IndexSet<NodeId> {
        take(&mut self.setup_dirty)
    }

    pub fn is_update_empty(&self) -> bool {
        self.update_dirty.is_empty()
    }
    pub fn add_update(&mut self, id: NodeId) {
        self.update_dirty.insert(id);
    }
    pub fn take_update(&mut self) -> IndexSet<NodeId> {
        take(&mut self.update_dirty)
    }

    pub fn is_render_empty(&self) -> bool {
        self.render_dirty.is_empty()
    }
    pub fn add_render(&mut self, id: NodeId) {
        self.render_dirty.insert(id);
    }
    pub fn take_render(&mut self) -> IndexSet<NodeId> {
        take(&mut self.render_dirty)
    }
}
