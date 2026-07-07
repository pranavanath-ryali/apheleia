use std::ops::{Deref, DerefMut};

use crate::{resources::Resource, systems::system::SystemParam, world::World};

/// Used to get a reference to [`Resource`] asked that is registered in [`World`]
///
/// # Condition
///
/// * The `system param` is considered `valid` as long as the resource is registered to [`World`]
/// * The `system param` is considered `invalid` if the resource isn't registered to [`World`]
///
/// # Example
///
/// ```rust
/// #[derive(Resource)]
/// struct Velocity {
///     pub x: f32,
///     pub y: f32
/// }
/// impl Resource for Velocity {}
///
/// fn test_system(res: Res<Velocity>) {
///     assert_eq!(res.x, 10.0);
///     assert_eq!(res.y, 5.0);
/// }
/// 
/// world.add_resource(Velocity { x: 10.0, y: 5.0 });
/// world.add_system(SystemRunStage::Update, STAGE, test_system);
/// ```
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
    unsafe fn fetch(world: *mut World) -> Option<Self> {
        let world = unsafe { &*world };

        if let Some(res) = world.get_resource::<R>() {
            return Some(Res::new(res));
        }
        None
    }
}

/// Used to get a mutable reference to [`Resource`] asked that is registered in [`World`]
///
/// # Condition
///
/// * The `system param` is considered `valid` as long as the resource is registered to [`World`]
/// * The `system param` is considered `invalid` if the resource isn't registered to [`World`]
///
/// # Example
///
/// ```rust
/// #[derive(Resource)]
/// struct Velocity {
///     pub x: f32,
///     pub y: f32
/// }
/// impl Resource for Velocity {}
///
/// fn mut_resource_system(res: ResMut<Velocity>) {
///     res.x += 10;
/// }
///
/// fn test_system(res: Res<Velocity>) {
///     assert_eq!(res.x, 20.0);
///     assert_eq!(res.y, 5.0);
/// }
/// 
/// world.add_resource(Velocity { x: 10.0, y: 5.0 });
/// world.add_system(SystemRunStage::Update, PRE_STAGE, mut_resource_system);
/// world.add_system(SystemRunStage::Update, STAGE, test_system);
/// ```
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
    unsafe fn fetch(world: *mut World) -> Option<Self> {
        let world = unsafe { &mut *world };

        if let Some(res) = world.get_resource_mut::<R>() {
            return Some(ResMut::new(res));
        }
        None
    }
}
