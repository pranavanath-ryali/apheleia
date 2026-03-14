use apheleia_core::{buffer::Line, types::vector::Vector2};

use crate::{
    NodeId,
    contexts::ContextCommand,
    rootnode::RootNodeData,
    types::{DirtyRenderLevel, EventType},
    utils::calculate_global_position,
};

pub struct SetSizeForId(pub NodeId, pub Vector2);
pub struct SetPositionForId(pub NodeId, pub Vector2);
pub struct RegisterForUpdate;
pub struct RegisterForEvent(pub EventType);

pub struct MarkRenderDirty(pub NodeId, pub DirtyRenderLevel);
pub struct MarkUpdateDirty(pub NodeId);

pub struct WriteLineToBuffer(pub Line);

impl ContextCommand for SetSizeForId {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data
            .id_data
            .get_mut(&self.0)
            .unwrap()
            .set_size(self.1);
    }
}
impl ContextCommand for SetPositionForId {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data
            .id_data
            .get_mut(&self.0)
            .unwrap()
            .set_position(self.1);

        let position =
            calculate_global_position(self.0, &rootnode_data.relations, &rootnode_data.id_data);
        rootnode_data
            .id_data
            .get_mut(&self.0)
            .unwrap()
            .set_global_position(position);
    }
}
impl ContextCommand for RegisterForUpdate {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data
            .id_update_type
            .get_mut(&crate::types::UpdateTypeNode::ConstantUpdate)
            .unwrap()
            .insert(id);
    }
}
impl ContextCommand for RegisterForEvent {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data
            .id_update_type
            .get_mut(&crate::types::UpdateTypeNode::Event(self.0))
            .unwrap()
            .insert(id);
    }
}
impl ContextCommand for MarkRenderDirty {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        match self.1 {
            DirtyRenderLevel::SimpleDirty => {
                rootnode_data.id_dirty_render.insert(id);
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
                    rootnode_data.id_dirty_render.insert(*id);
                }
            }
        }
    }
}
impl ContextCommand for MarkUpdateDirty {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data.id_dirty_update.insert(id);
    }
}

impl ContextCommand for WriteLineToBuffer {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        let line = self.0;
        rootnode_data.buffer.write_line(
            line.position.0,
            line.position.1,
            &line.text,
            Some(line.style),
        );
    }
}
