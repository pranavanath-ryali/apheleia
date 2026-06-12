use std::{
    collections::HashSet, iter::{self, Map}, marker::PhantomData, process::id, slice
};

use apheleia_ecs_new::{
    NodeId,
    extensions::Extension,
    systems::system::SystemParam,
    types::NodeData,
    world::{self, World},
};
use log::info;

pub trait WorldQuery {
    type Item<'w>;

    fn match_ids(world: &World) -> Vec<NodeId>;
    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Self::Item<'w>;
}

impl<E: Extension> WorldQuery for &E {
    type Item<'w> = &'w E;

    fn match_ids(world: &World) -> Vec<NodeId> {
        world.get_nodes_with_extension::<E>()
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Self::Item<'w> {
        let world = unsafe { &*world };
        world
            .get_extension::<E>(id).expect("Unexpected error")
    }
}

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

impl<Q1: WorldQuery, Q2: WorldQuery> WorldQuery for (Q1, Q2) {
    type Item<'w> = (Q1::Item<'w>, Q2::Item<'w>);

    fn match_ids(world: &World) -> Vec<NodeId> {
        let q1_ids_set: HashSet<NodeId> = Q1::match_ids(world).iter().copied().collect();
        let q2_ids = Q2::match_ids(world);

        q1_ids_set
            .into_iter()
            .filter(|id| q2_ids.contains(id))
            .collect()
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Self::Item<'w> {
        unsafe { (Q1::fetch(world, id), Q2::fetch(world, id)) }
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

pub trait QueryFilter {
    fn matches(world: &World, id: NodeId) -> bool;
}

impl QueryFilter for () {
    fn matches(_world: &World, _id: NodeId) -> bool {
        true
    }
}

pub struct With<T> {
    marker: PhantomData<T>
}
impl<T: Extension> QueryFilter for With<T> {
    fn matches(world: &World, id: NodeId) -> bool {
        world.get_extension::<T>(id).is_some()
    }
}

pub struct Query<'w, Q: WorldQuery, F: QueryFilter = ()> {
    world: *mut World,
    ids: Vec<NodeId>,
    _marker: PhantomData<(&'w (), Q, F)>,
}

impl<'w, Q: WorldQuery, F: QueryFilter> Query<'w, Q, F> {
    pub(crate) fn new(world: &'w mut World) -> Self {
        let mut ids = Q::match_ids(world);
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

pub struct QueryIter<'a, Q: WorldQuery> {
    world: *mut World,
    ids: slice::Iter<'a, NodeId>,

    _marker: PhantomData<(&'a mut World, Q)>,
}
impl<'a, Q: WorldQuery> QueryIter<'a, Q> {
    pub(crate) fn new(world: *mut World, ids: slice::Iter<'a, NodeId>) -> Self {
        Self {
            world,
            ids,
            _marker: PhantomData,
        }
    }
}

impl<'a, Q: WorldQuery> Iterator for QueryIter<'a, Q> {
    type Item = Q::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.ids.next() {
            let world = unsafe { &mut *self.world };
            return Some(unsafe { Q::fetch(world, *id) });
        }
        None
    }
}

impl<Q: WorldQuery + 'static, F: QueryFilter + 'static> SystemParam for Query<'static, Q, F> {
    unsafe fn fetch(world: *mut World) -> Option<Self> {
        let world = unsafe { &mut *world };
        let query = Self::new(world);

        if query.is_empty() {
            return None;
        }
        Some(query)
    }
}
