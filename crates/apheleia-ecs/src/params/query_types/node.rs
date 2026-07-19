use apheleia_core::node_buffer::NodeBuffer;

use crate::{nodedata::NodeData, stores::nodebuffer::NodeBufferStore, traits::query_type::QueryType, types::NodeId, world::World};


impl QueryType for NodeBuffer {
    type Item<'w> = &'w mut NodeBuffer;

    fn match_ids(_world: &World) -> Option<Vec<NodeId>> {
        None
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Option<Self::Item<'w>> {
        let world = unsafe { &mut *world };

        let data = world.get_nodedata(id).unwrap();
        let global_position = data.get_global_position();
        let global_size = data.get_global_size();

        world
            .get_resource_mut::<NodeBufferStore>()
            .unwrap()
            .create_or_get_buffer(global_position, global_size, id)
    }
}

impl QueryType for NodeData {
    type Item<'w> = &'w NodeData;

    fn match_ids(_world: &World) -> Option<Vec<NodeId>> {
        None
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Option<Self::Item<'w>> {
        let world = unsafe { &*world };
        if let Some(data) = world.get_nodedata(id) {
            return Some(data);
        }
        None
    }
}

impl QueryType for NodeId {
    type Item<'w> = NodeId;

    fn match_ids(_world: &World) -> Option<Vec<NodeId>> {
        None
    }

    unsafe fn fetch<'w>(_world: *mut World, id: NodeId) -> Option<Self::Item<'w>> {
        Some(id)
    }
}
