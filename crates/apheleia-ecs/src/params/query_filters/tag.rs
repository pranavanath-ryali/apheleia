use std::marker::PhantomData;

use crate::{
    stores::tag::TagRegistry,
    traits::{query_filter::QueryFilter, tag::TagTrait},
    types::NodeId,
    world::World,
};

pub struct WithTag<T: TagTrait> {
    marker: PhantomData<T>,
}
impl<T: TagTrait> QueryFilter for WithTag<T> {
    fn matches(world: &World, id: NodeId) -> bool {
        if let Some(nodes) = world
            .get_resource::<TagRegistry>()
            .unwrap()
            .get_nodes_with_tag::<T>()
        {
            return nodes.contains(&id);
        }
        false
    }
}
