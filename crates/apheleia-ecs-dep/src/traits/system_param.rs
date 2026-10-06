use crate::world::World;

pub trait SystemParam: Sized + 'static {
    unsafe fn fetch<'w>(world: *mut World) -> Option<Self>;
}
