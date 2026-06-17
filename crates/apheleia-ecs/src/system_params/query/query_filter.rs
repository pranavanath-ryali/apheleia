use std::marker::PhantomData;

use crate::{extensions::Extension, types::NodeId, world::World};

pub trait QueryFilter {
    fn matches(world: &World, id: NodeId) -> bool;
}

impl QueryFilter for () {
    fn matches(_world: &World, _id: NodeId) -> bool {
        true
    }
}

// pub struct WithEvent<const E: EventId>;
// impl<const E: EventId> QueryFilter for WithEvent<E> {
//     fn matches(world: &World, id: NodeId) -> bool {
//         world.is_local_event(id, E)
//     }
// }

pub struct With<T: Extension> {
    marker: PhantomData<T>,
}
impl<T: Extension> QueryFilter for With<T> {
    fn matches(world: &World, id: NodeId) -> bool {
        world.get_extension::<T>(id).is_some()
    }
}

pub struct Without<T: Extension> {
    marker: PhantomData<T>,
}
impl<T: Extension> QueryFilter for Without<T> {
    fn matches(world: &World, id: NodeId) -> bool {
        world.get_extension::<T>(id).is_none()
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
