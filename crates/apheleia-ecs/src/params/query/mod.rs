use core::slice;
use std::marker::PhantomData;

use crate::{
    params::query::{query_filter::QueryFilter, query_type::QueryType},
    systems::system::SystemParam,
    types::NodeId,
    world::World,
};

pub mod query_filter;
pub mod query_type;

/// A query for searching and filtering [`Extension`]s, [`NodeData`]s, [`NodeId`]s, etc within the ECS world.
///
/// `Query` provides an interface to fetch data from the `World` based on a data query `Q`
/// and an optional filter `F`.
///
/// # Type Parameters
///
/// * `Q`: The [`QueryType`] that defines what component data will be fetched.
/// * `F`: The [`QueryFilter`] used to narrow down the results (e.g., tracking additions or mutations).
///   Defaults to `()`, which applies no filtering.
///
/// # Safety
///
/// This struct contains a raw pointer (`*mut World`) and uses a lifetime parameter `'w`
/// to bound its access to the world. Ensure that the `Query` does not outlive the
/// `World` it points to, and that aliasing rules for components are strictly enforced.
pub struct Query<'w, Q: QueryType, F: QueryFilter = ()> {
    world: *mut World,
    ids: Vec<NodeId>,
    _marker: PhantomData<(&'w (), Q, F)>,
}

impl<'w, Q: QueryType, F: QueryFilter> Query<'w, Q, F> {
    pub(crate) fn new(world: &'w mut World) -> Self {
        let mut ids = Q::match_ids(world)
            .unwrap_or_else(|| world.get_registered_nodes().iter().copied().collect());
        ids.retain(|id| F::matches(world, *id));

        Self {
            world,
            ids,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn get_single(&self) -> Option<Q::Item<'_>> {
        if self.is_empty() {
            return None;
        }
        Some(unsafe { Q::fetch(self.world, self.ids[0]) })
    }

    pub fn iter(&self) -> QueryIter<'_, Q> {
        QueryIter::new(self.world, self.ids.iter())
    }
}

pub struct QueryIter<'a, Q: QueryType> {
    world: *mut World,
    ids: slice::Iter<'a, NodeId>,

    _marker: PhantomData<(&'a mut World, Q)>,
}
impl<'a, Q: QueryType> QueryIter<'a, Q> {
    pub(crate) fn new(world: *mut World, ids: slice::Iter<'a, NodeId>) -> Self {
        Self {
            world,
            ids,
            _marker: PhantomData,
        }
    }
}

impl<'a, Q: QueryType> Iterator for QueryIter<'a, Q> {
    type Item = Q::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.ids.next() {
            let world = unsafe { &mut *self.world };
            return Some(unsafe { Q::fetch(world, *id) });
        }
        None
    }
}

impl<Q: QueryType + 'static, F: QueryFilter + 'static> SystemParam for Query<'static, Q, F> {
    unsafe fn fetch(world: *mut World) -> Option<Self> {
        let world = unsafe { &mut *world };
        let query = Self::new(world);

        if query.is_empty() {
            return None;
        }
        Some(query)
    }
}
