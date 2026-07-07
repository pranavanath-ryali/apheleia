use std::collections::HashSet;

use apheleia_core::node_buffer::NodeBuffer;

use crate::{extensions::Extension, nodedata::data::NodeData, types::NodeId, world::World};

/// A trait for querying [`Extension`]s, [`NodeId`]s, and [`NodeData`] from [`World`]
pub trait QueryType {
    type Item<'w>;

    /// This returns an Option of [`NodeIds`].
    /// * `None` - means its valid for any node. This is useful for querying over [`NodeId`], [`NodeData`], etc
    fn match_ids(world: &World) -> Option<Vec<NodeId>>;

    /// # Safety
    /// This function will dereference the given raw pointer of [`World`]
    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Self::Item<'w>;
}

/// Implement [`QueryType`] for reference to [`Extension`]
impl<E: Extension> QueryType for &E {
    type Item<'w> = &'w E;

    fn match_ids(world: &World) -> Option<Vec<NodeId>> {
        Some(world.get_nodes_with_extension::<E>())
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Self::Item<'w> {
        let world = unsafe { &*world };
        world.get_extension::<E>(id).expect("Unexpected error")
    }
}

/// Implement [`QueryType`] for mutable referece to [`Extension`]
impl<E: Extension> QueryType for &mut E {
    type Item<'w> = &'w mut E;

    fn match_ids(world: &World) -> Option<Vec<NodeId>> {
        Some(world.get_nodes_with_extension::<E>())
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Self::Item<'w> {
        let world = unsafe { &mut *world };
        world
            .get_extension_mut::<E>(id)
            .expect("Unexpected extension does not exist for node")
    }
}

impl QueryType for NodeData {
    type Item<'w> = NodeData;

    fn match_ids(_world: &World) -> Option<Vec<NodeId>> {
        None
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Self::Item<'w> {
        let world = unsafe { &*world };
        *world
            .get_nodedata(id)
            .expect("Unexpeted. No NodeData for node")
    }
}

impl QueryType for NodeId {
    type Item<'w> = NodeId;

    fn match_ids(_world: &World) -> Option<Vec<NodeId>> {
        None
    }

    unsafe fn fetch<'w>(_world: *mut World, id: NodeId) -> Self::Item<'w> {
        id
    }
}

macro_rules! impl_world_query {
    ( $($query:ident),+ ) => {
        impl<$($query: QueryType),+> QueryType for ($($query,)*) {
            type Item<'w> = ($($query::Item<'w>),*);

            fn match_ids(world: &World) -> Option<Vec<NodeId>> {
                let mut sets: Vec<HashSet<NodeId>> = Default::default();
                $(
                    if let Some(ids) = $query::match_ids(world) {
                        sets.push(ids.into_iter().collect());
                    }
                )+

                if sets.is_empty() {
                    return None;
                }
                Some(sets[0]
                    .iter()
                    .filter(|id| sets[1..].iter().all(|s| s.contains(id)))
                    .copied()
                    .collect())
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
