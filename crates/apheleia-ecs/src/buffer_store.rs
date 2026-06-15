use apheleia_core::buffer::Buffer;
use log::{info, warn};
use rustc_hash::FxHashMap;

use crate::{NodeId, types::NodeData, world::World};

#[derive(Default)]
pub struct BufferStore {
    id_to_buffer: FxHashMap<NodeId, Buffer>,
}
impl BufferStore {
    pub fn get_buffer_mut(&mut self, data: NodeData, id: NodeId) -> Option<&mut Buffer> {
        if self.id_to_buffer.contains_key(&id) {
            info!("[ECS] NodeBuffer exists for NodeID: {}", id);
            return self.id_to_buffer.get_mut(&id);
        }
        else if let Some(size) = data.global_size
            && (size.x != 0 && size.y != 0)
        {
            warn!("[ECS] Created new NodeBuffer of size: {:?} for node: {}", size, id);
            self.id_to_buffer.insert(id, Buffer::new(size));
            return Some(self.id_to_buffer.get_mut(&id).unwrap());
        }

        info!("[ECS] Skipped creating NodeBuffer since one of the dimension for global_size is 0 for NodeID: {}", id);

        None
    }
}
