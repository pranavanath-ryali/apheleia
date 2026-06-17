use std::marker::PhantomData;

use crate::{events::EventTrait, extensions::Extension, types::NodeId, world::World};

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

/// A [`QueryFilter`] that validates if the node has the given `local event`
pub struct WithEvent<E: EventTrait> {
    _marker: PhantomData<E>,
}
impl<E: EventTrait> QueryFilter for WithEvent<E> {
    fn matches(world: &World, id: NodeId) -> bool {
        world.is_event::<E>(id)
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
