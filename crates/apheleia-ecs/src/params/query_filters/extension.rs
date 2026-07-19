use std::marker::PhantomData;

use crate::{traits::{extension::Extension, query_filter::QueryFilter}, types::NodeId, world::World};

/// A [`QueryFilter`] that validates if the node has the [`Extension`] binded to it
pub struct With<T: Extension> {
    marker: PhantomData<T>,
}
impl<T: Extension> QueryFilter for With<T> {
    fn matches(world: &World, id: NodeId) -> bool {
        world.get_extension::<T>(id).is_some()
    }
}

/// A [`QueryFilter`] that validates if the node has the [`Extension`] not binded to it
pub struct Without<T: Extension> {
    marker: PhantomData<T>,
}
impl<T: Extension> QueryFilter for Without<T> {
    fn matches(world: &World, id: NodeId) -> bool {
        world.get_extension::<T>(id).is_none()
    }
}
