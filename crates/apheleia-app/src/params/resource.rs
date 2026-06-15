use std::ops::{Deref, DerefMut};

use apheleia_ecs::{resources::Resource, systems::system::SystemParam};

pub struct Res<'w, R: 'static> {
    res: &'w R,
}
impl<'w, R: 'static> Res<'w, R> {
    pub(crate) fn new(res: &'w R) -> Self {
        Self { res }
    }
}
impl<'w, R: 'static> Deref for Res<'w, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.res
    }
}

impl<R: Resource + 'static> SystemParam for Res<'static, R> {
    unsafe fn fetch(world: *mut apheleia_ecs::world::World) -> Option<Self> {
        let world = unsafe { &*world };

        if let Some(res) = world.get_resource::<R>() {
            return Some(Res::new(res));
        }
        None
    }
}

pub struct ResMut<'w, R: 'static> {
    res: &'w mut R,
}
impl<'w, R: 'static> ResMut<'w, R> {
    pub(crate) fn new(res: &'w mut R) -> Self {
        Self { res }
    }
}

impl<'w, R: 'static> Deref for ResMut<'w, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.res
    }
}
impl<'w, R: 'static> DerefMut for ResMut<'w, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.res
    }
}

impl<R: Resource + 'static> SystemParam for ResMut<'static, R> {
    unsafe fn fetch(world: *mut apheleia_ecs::world::World) -> Option<Self> {
        let world = unsafe { &mut *world };

        if let Some(res) = world.get_resource_mut::<R>() {
            return Some(ResMut::new(res));
        }
        None
    }
}
