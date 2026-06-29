use apheleia_core::node_buffer::NodeBuffer;
use apheleia_ecs::{nodedata::data::NodeData, resources::Resource, types::NodeId};
use log::{info, warn};
use rustc_hash::FxHashMap;

#[derive(Debug, Default)]
pub struct NodeBuffers {
    id_to_buffer: FxHashMap<NodeId, NodeBuffer>
}
impl NodeBuffers {
    pub(crate) fn get_buffer(&mut self, id: NodeId) -> Option<&mut NodeBuffer> {
        self.id_to_buffer.get_mut(&id)
    }

    pub fn create_or_get_buffer(&mut self, data: NodeData, id: NodeId) -> Option<&mut NodeBuffer> {
        if self.id_to_buffer.contains_key(&id) {
            info!("[ECS] NodeBuffer exists for NodeID: {}", id);
            return self.id_to_buffer.get_mut(&id);
        }
        else if let Some(size) = data.global_size
            && (size.x != 0 && size.y != 0)
        {
            warn!("[ECS] Created new NodeBuffer of size: {:?} for node: {}", size, id);
            self.id_to_buffer.insert(id, NodeBuffer::new(size));
            return Some(self.id_to_buffer.get_mut(&id).unwrap());
        }

        info!("[ECS] Skipped creating NodeBuffer since one of the dimension for global_size is 0 for NodeID: {}", id);

        None
    }
}


impl Resource for NodeBuffers {}
