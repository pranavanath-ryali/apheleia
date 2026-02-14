use apheleia_core::{buffer::Line, types::vector::Vector2};

use crate::{
    NodeId,
    contexts::ContextCommand,
    rootnode::{self, RootNodeData},
    types::{DirtyRenderLevel, EventType},
    utils::calculate_global_position,
};

pub struct Command_SetSizeForId(pub NodeId, pub Vector2);
pub struct Command_SetPositionForId(pub NodeId, pub Vector2);
pub struct Command_RegisterForUpdate;
pub struct Command_RegisterForEvent(pub EventType);

pub struct Command_MarkRenderDirty(pub NodeId, pub DirtyRenderLevel);
pub struct Command_MarkUpdateDirty(pub NodeId);

pub struct Command_WriteLineToBuffer(pub Line);

impl ContextCommand for Command_SetSizeForId {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data
            .id_data
            .get_mut(&self.0)
            .unwrap()
            .set_size(self.1);
    }
}
impl ContextCommand for Command_SetPositionForId {
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
impl ContextCommand for Command_RegisterForUpdate {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data
            .id_update_type
            .get_mut(&crate::types::UpdateTypeNode::ConstantUpdate)
            .unwrap()
            .insert(id);
    }
}
impl ContextCommand for Command_RegisterForEvent {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data
            .id_update_type
            .get_mut(&crate::types::UpdateTypeNode::Event(self.0))
            .unwrap()
            .insert(id);
    }
}
impl ContextCommand for Command_MarkRenderDirty {
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
impl ContextCommand for Command_MarkUpdateDirty {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: &mut RootNodeData) {
        rootnode_data.id_dirty_update.insert(id);
    }
}

impl ContextCommand for Command_WriteLineToBuffer {
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
