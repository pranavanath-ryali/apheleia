use apheleia_core::types::vector::Vector2;

use crate::{
    contexts::ContextCommand,
    rootnode::data::RootNodeData,
    types::{DirtyRenderLevel, EventType, NodeId, UpdateTypeNode},
};

pub struct SetSize(pub Vector2);
pub struct SetSizeForNode(pub String, pub Vector2);

pub struct RegisterForUpdate;
pub struct RegisterForEvent(pub EventType);

pub struct MarkRenderDirty(pub NodeId, pub DirtyRenderLevel);
pub struct MarkUpdateDirty(pub NodeId);

impl ContextCommand for SetSize {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data
            .node_storage
            .borrow_mut()
            .get_data_mut(id)
            .unwrap()
            .set_size(self.0);
    }
}
impl ContextCommand for SetSizeForNode {
    fn execute(self: Box<Self>, _id: NodeId, rootnode_data: &mut RootNodeData) {
        if let Some(id) = rootnode_data.node_storage.borrow().get_id(self.0.as_str()) {
            rootnode_data
                .node_storage
                .borrow_mut()
                .get_data_mut(*id)
                .unwrap()
                .set_size(self.1);
        }
    }
}
impl ContextCommand for RegisterForUpdate {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data
            .update_tracker
            .borrow_mut()
            .add_node(id, UpdateTypeNode::ConstantUpdate);
    }
}
impl ContextCommand for RegisterForEvent {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data
            .update_tracker
            .borrow_mut()
            .add_node(id, UpdateTypeNode::Event(self.0));
    }
}
impl ContextCommand for MarkRenderDirty {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        match self.1 {
            DirtyRenderLevel::SimpleDirty => {
                rootnode_data.dirty_tracker.borrow_mut().add_render(id);
            }
            DirtyRenderLevel::SubtreeDirty => {
                for id in rootnode_data
                    .relations
                    .get_subtree(&self.0, None)
                    .unwrap()
                    .traverse(&self.0, tree_ds::prelude::TraversalStrategy::PreOrder)
                    .unwrap()
                    .iter()
                {
                    rootnode_data.dirty_tracker.borrow_mut().add_render(*id);
                }
            }
        }
    }
}
impl ContextCommand for MarkUpdateDirty {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data.dirty_tracker.borrow_mut().add_update(id);
    }
}
