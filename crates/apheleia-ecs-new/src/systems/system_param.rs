use crate::world::world_cell::UnsafeWorldCellMut;

pub trait SystemParam {
    type Item<'w>;
    fn fetch<'w>(world: UnsafeWorldCellMut<'w>) -> Option<Self::Item<'w>>;
}
