use apheleia_core::node_buffer::NodeBuffer;
use log::{info, warn};
use rustc_hash::FxHashMap;

use crate::{nodedata::data::NodeData, resources::Resource, types::NodeId};

#[derive(Debug, Default)]
pub struct BufferStore {
    id_to_buffer: FxHashMap<NodeId, NodeBuffer>
}
impl BufferStore {
    pub fn get_buffer(&mut self, id: NodeId) -> Option<&mut NodeBuffer> {
        self.id_to_buffer.get_mut(&id)
    }

    pub fn create_or_get_buffer(&mut self, data: NodeData, id: NodeId) -> Option<&mut NodeBuffer> {
        if let Some(size) = data.global_size && let Some(global_position) = data.global_position && (size.x != 0 || size.y != 0) {
            return Some(self.id_to_buffer.entry(id).or_insert_with(|| {
                info!("[ECS] Creating new NodeBuffer of size: {:?} for node: {}", size, id);
                NodeBuffer::new(global_position, size)
            }));
        }

        info!("[ECS] Skipped creating NodeBuffer since one of the dimension for global_size is 0 for NodeID: {}", id);
        None
    }
}


impl Resource for BufferStore {}
