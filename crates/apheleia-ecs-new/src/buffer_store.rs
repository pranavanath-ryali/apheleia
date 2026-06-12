use apheleia_core::buffer::Buffer;
use log::info;
use rustc_hash::FxHashMap;

use crate::{NodeId, types::NodeData, world::World};

#[derive(Default)]
pub struct BufferStore {
    id_to_buffer: FxHashMap<NodeId, Buffer>,
}
impl BufferStore {
    pub fn get_buffer_mut(&mut self, data: NodeData, id: NodeId) -> Option<&mut Buffer> {
        if self.id_to_buffer.contains_key(&id) {
            return self.id_to_buffer.get_mut(&id);
        }
        else if let Some(size) = data.global_size
            && (size.x != 0 && size.y != 0)
        {
            info!("ECS - Created new NodeBuffer of size: {:?} for node: {}", size, id);
            self.id_to_buffer.insert(id, Buffer::new(size));
            return Some(self.id_to_buffer.get_mut(&id).unwrap());
        }

        None
    }

    pub(crate) fn get_buffers(&mut self) -> &mut FxHashMap<NodeId, Buffer> {
        &mut self.id_to_buffer
    }
}
