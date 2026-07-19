use crate::{types::NodeId, world::World};
use std::collections::HashSet;

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
