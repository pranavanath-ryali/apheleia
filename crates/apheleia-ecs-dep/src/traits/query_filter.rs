use crate::{types::NodeId, world::World};


/// A trait for filtering [`NodeId`] from a [`Query`]
pub trait QueryFilter {
    fn matches(world: &World, id: NodeId) -> bool;
}

impl QueryFilter for () {
    fn matches(_world: &World, _id: NodeId) -> bool {
        true
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
