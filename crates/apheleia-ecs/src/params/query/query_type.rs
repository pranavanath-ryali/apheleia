use std::collections::HashSet;

use apheleia_core::node_buffer::NodeBuffer;

use crate::{
    extensions::Extension, nodedata::data::NodeData, resources::buffer_store::BufferStore,
    types::NodeId, world::World,
};

/// A trait for querying [`Extension`]s, [`NodeId`]s, and [`NodeData`] from [`World`]
pub trait QueryType {
    type Item<'w>;

    /// This returns an Option of [`NodeIds`].
    /// * `None` - means its valid for any node. This is useful for querying over [`NodeId`], [`NodeData`], etc
    fn match_ids(world: &World) -> Option<Vec<NodeId>>;

    /// # Safety
    /// This function will dereference the given raw pointer of [`World`]
    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Option<Self::Item<'w>>;
}

/// Implement [`QueryType`] for reference to [`Extension`]
impl<E: Extension> QueryType for &E {
    type Item<'w> = &'w E;

    fn match_ids(world: &World) -> Option<Vec<NodeId>> {
        Some(world.get_nodes_with_extension::<E>())
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Option<Self::Item<'w>> {
        let world = unsafe { &*world };
        world.get_extension::<E>(id)
    }
}

/// Implement [`QueryType`] for mutable referece to [`Extension`]
impl<E: Extension> QueryType for &mut E {
    type Item<'w> = &'w mut E;

    fn match_ids(world: &World) -> Option<Vec<NodeId>> {
        Some(world.get_nodes_with_extension::<E>())
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Option<Self::Item<'w>> {
        let world = unsafe { &mut *world };
        world.get_extension_mut::<E>(id)
    }
}

impl QueryType for NodeBuffer {
    type Item<'w> = &'w mut NodeBuffer;

    fn match_ids(_world: &World) -> Option<Vec<NodeId>> {
        None
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Option<Self::Item<'w>> {
        let world = unsafe { &mut *world };
        let data = *world.get_nodedata(id).unwrap();
        world
            .get_resource_mut::<BufferStore>()
            .unwrap()
            .create_or_get_buffer(data, id)
    }
}

impl QueryType for NodeData {
    type Item<'w> = NodeData;

    fn match_ids(_world: &World) -> Option<Vec<NodeId>> {
        None
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Option<Self::Item<'w>> {
        let world = unsafe { &*world };
        if let Some(data) = world.get_nodedata(id) {
            return Some(*data);
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

            unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Option<Self::Item<'w>> {
                unsafe {
                $(
                    let $query = $query::fetch(world, id)?;
                )+

                Some( ($($query),+) )
                }
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
