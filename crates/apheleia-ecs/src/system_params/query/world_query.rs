use std::collections::HashSet;

use crate::{extensions::Extension, nodedata::data::NodeData, types::NodeId, world::World};

/// A trait for querying [`Extension`]s, [`NodeId`]s, and [`NodeData`] from [`World`]
pub trait WorldQuery {
    type Item<'w>;

    fn match_ids(world: &World) -> Vec<NodeId>;

    /// # Safety
    /// This function is unsafe because it will dereference the raw pointer of [`World`] which is
    /// provided
    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Self::Item<'w>;
}

/// Implement [`WorldQuery`] for reference to [`Extension`]
impl<E: Extension> WorldQuery for &E {
    type Item<'w> = &'w E;

    fn match_ids(world: &World) -> Vec<NodeId> {
        world.get_nodes_with_extension::<E>()
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Self::Item<'w> {
        let world = unsafe { &*world };
        world.get_extension::<E>(id).expect("Unexpected error")
    }
}

/// Implement [`WorldQuery`] for mutable referece to [`Extension`]
impl<E: Extension> WorldQuery for &mut E {
    type Item<'w> = &'w mut E;

    fn match_ids(world: &World) -> Vec<NodeId> {
        world.get_nodes_with_extension::<E>()
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Self::Item<'w> {
        let world = unsafe { &mut *world };
        world
            .get_extension_mut::<E>(id)
            .expect("Unexpected extension does not exist for node")
    }
}

impl WorldQuery for NodeData {
    type Item<'w> = NodeData;

    fn match_ids(world: &World) -> Vec<NodeId> {
        world.get_registered_nodes().iter().copied().collect()
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Self::Item<'w> {
        let world = unsafe { &*world };
        *world
            .get_nodedata(id)
            .expect("Unexpeted. No NodeData for node")
    }
}

impl WorldQuery for NodeId {
    type Item<'w> = NodeId;

    fn match_ids(world: &World) -> Vec<NodeId> {
        world.get_registered_nodes().iter().copied().collect()
    }

    unsafe fn fetch<'w>(_world: *mut World, id: NodeId) -> Self::Item<'w> {
        id
    }
}

macro_rules! impl_world_query {
    ( $($query:ident),+ ) => {
        impl<$($query: WorldQuery),*> WorldQuery for ($($query,)*) {
            type Item<'w> = ($($query::Item<'w>),*);

            fn match_ids(world: &World) -> Vec<NodeId> {
                let sets: Vec<HashSet<NodeId>> = vec![
                    $(
                        $query::match_ids(world).into_iter().collect()
                    ),+
                ];
                sets[0]
                    .iter()
                    .filter(|id| sets[1..].iter().all(|s| s.contains(id)))
                    .copied()
                    .collect()
            }

            unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Self::Item<'w> {
                unsafe { ($( $query::fetch(world, id) ),+) }
            }
        }
    };
}

impl_world_query!(Q0, Q1);
impl_world_query!(Q0, Q1, Q2);
impl_world_query!(Q0, Q1, Q2, Q3);
impl_world_query!(Q0, Q1, Q2, Q3, Q4);
impl_world_query!(Q0, Q1, Q2, Q3, Q4, Q5);
impl_world_query!(Q0, Q1, Q2, Q3, Q4, Q5, Q6);
