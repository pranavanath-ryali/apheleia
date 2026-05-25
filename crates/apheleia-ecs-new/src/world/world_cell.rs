use std::{marker::PhantomData, ptr};

use crate::{
    resources::Resource,
    world::{self, World},
};

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
    pub unsafe fn world(&self) -> &'w World {
        unsafe { &*self.ptr }
    }
}

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
    pub unsafe fn world(&self) -> &'w World {
        unsafe { &*self.ptr }
    }

    #[inline]
    pub unsafe fn world_mut(&mut self) -> &'w mut World {
        unsafe { &mut *self.ptr }
    }

    pub unsafe fn get_resource_mut<R: Resource>(&mut self) -> Option<&'w mut R> {
        unsafe { self.world_mut().get_resource_mut::<R>() }
    }
}
