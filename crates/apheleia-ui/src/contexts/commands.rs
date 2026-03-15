use apheleia_core::{buffer::Line, types::vector::Vector2};

use crate::{
    NodeId,
    contexts::ContextCommand,
    rootnode::data::RootNodeData,
    types::{DirtyRenderLevel, EventType, UpdateTypeNode},
};

pub struct SetSizeForId(pub NodeId, pub Vector2);
pub struct SetPositionForId(pub NodeId, pub Vector2);
pub struct RegisterForUpdate;
pub struct RegisterForEvent(pub EventType);

pub struct MarkRenderDirty(pub NodeId, pub DirtyRenderLevel);
pub struct MarkUpdateDirty(pub NodeId);

pub struct WriteLineToBuffer(pub Line);

impl ContextCommand for SetSizeForId {
    fn execute(self: Box<Self>, _id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data
            .node_storage
            .borrow_mut()
            .get_data_mut(self.0)
            .unwrap()
            .set_size(self.1);
    }
}
impl ContextCommand for SetPositionForId {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        // TODO
        // rootnode_data
        //     .node_storage
        //     .borrow_mut()
        //     .get_data_mut(self.0)
        //     .unwrap()
        //     .set_position(self.1);

        // let position =
        //     calculate_global_position(self.0, &rootnode_data.relations, &rootnode_data.id_data);
        // rootnode_data
        //     .id_data
        //     .get_mut(&self.0)
        //     .unwrap()
        //     .set_global_position(position);
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

impl ContextCommand for WriteLineToBuffer {
    fn execute(self: Box<Self>, _id: NodeId, rootnode_data: &mut RootNodeData) {
        let line = self.0;
        rootnode_data
            .buffer
            .write_line(line.position.0, line.position.1, &line.text, None);
    }
}
