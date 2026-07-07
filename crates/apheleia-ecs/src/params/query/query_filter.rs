use std::marker::PhantomData;

use crate::{extensions::Extension, tags::TagTrait, types::NodeId, world::World};

/// A trait for filtering [`NodeId`] from a [`Query`]
pub trait QueryFilter {
    fn matches(world: &World, id: NodeId) -> bool;
}

impl QueryFilter for () {
    fn matches(_world: &World, _id: NodeId) -> bool {
        true
    }
}

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

pub struct WithTag<T: TagTrait> {
    marker: PhantomData<T>
}
impl<T: TagTrait> QueryFilter for WithTag<T> {
    fn matches(world: &World, id: NodeId) -> bool {
        if let Some(nodes) = world.get_nodes_tagged::<T>() {
            return nodes.contains(&id);
        }
        false
    }
}

macro_rules! impl_query_filter {
    ( $($filter:ident),+ ) => {
        impl< $($filter: QueryFilter),+ > QueryFilter for ( $($filter,)* ) {
            fn matches(world: &World, id: NodeId) -> bool {
                $($filter::matches(world, id))&&+
            }
        }
    };
}

impl_query_filter!(F0, F1);
impl_query_filter!(F0, F1, F2);
impl_query_filter!(F0, F1, F2, F3);
