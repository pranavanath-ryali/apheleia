use crate::world::world_cell::UnsafeWorldCellMut;

pub trait SystemParam {
    type Item<'w>;
    fn fetch<'w>(world: UnsafeWorldCellMut<'w>) -> Option<Self::Item<'w>>;
}

impl SystemParam for () {
    type Item<'w> = ();

    fn fetch<'w>(world: UnsafeWorldCellMut<'w>) -> Option<Self::Item<'w>> {
        Some(())
    }
}
impl<P1: SystemParam> SystemParam for (P1,) {
    type Item<'w> = ();

    fn fetch<'w>(world: UnsafeWorldCellMut<'w>) -> Option<Self::Item<'w>> {
        Some(())
    }
}
impl<P1: SystemParam, P2: SystemParam> SystemParam for (P1, P2) {
    type Item<'w> = ();

    fn fetch<'w>(world: UnsafeWorldCellMut<'w>) -> Option<Self::Item<'w>> {
        Some(())
    }
}
