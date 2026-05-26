use std::{marker::PhantomData, ptr};

use crate::world::World;

pub struct UnsafeWorldCell<'w> {
    ptr: *const World,
    marker: PhantomData<&'w World>,
}
impl<'w> From<&'w World> for UnsafeWorldCell<'w> {
    fn from(value: &'w World) -> Self {
        UnsafeWorldCell {
            ptr: ptr::from_ref(value),
            marker: PhantomData,
        }
    }
}
impl<'w> UnsafeWorldCell<'w> {
    #[inline]
    pub unsafe fn get_world(&self) -> &'w World {
        unsafe { &*self.ptr }
    }
}

#[derive(Clone)]
pub struct UnsafeWorldCellMut<'w> {
    ptr: *mut World,
    marker: PhantomData<&'w mut World>,
}
impl<'w> From<&'w mut World> for UnsafeWorldCellMut<'w> {
    fn from(value: &'w mut World) -> Self {
        Self {
            ptr: ptr::from_mut(value),
            marker: PhantomData,
        }
    }
}
impl<'w> UnsafeWorldCellMut<'w> {
    #[inline]
    pub unsafe fn get_world(&self) -> &'w World {
        unsafe { &*self.ptr }
    }

    #[inline]
    pub unsafe fn get_world_mut(&self) -> &'w mut World {
        unsafe { &mut *self.ptr }
    }
}
