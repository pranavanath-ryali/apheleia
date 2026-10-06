use std::ops::{Deref, DerefMut};
use crate::{traits::{resource::Resource, system_param::SystemParam}, world::World};

/// A [`SystemParam`] providing mutable access to a [`Resource`] registered in
/// the [`World`].
///
/// # Validity
///
/// This param is `valid` (and the system will run) only if a resource of
/// type `R` has been registered with the [`World`]; otherwise it is
/// `invalid` and the system is skipped for that call.
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
    /// Mutable reference to the resource fetched from the [`World`].
    res: &'w mut R,
}

impl<'w, R: 'static> ResMut<'w, R> {
    /// Wraps a mutable reference to a resource.
    ///
    /// Not intended for direct use; instances are created internally when a
    /// system parameter of type `ResMut<R>` is fetched from the [`World`]
    /// (see the [`SystemParam`] impl below).
    pub(crate) fn new(res: &'w mut R) -> Self {
        Self { res }
    }
}

impl<'w, R: 'static> Deref for ResMut<'w, R> {
    type Target = R;

    /// Dereferences to the underlying resource, allowing `ResMut<R>` to be
    /// used as if it were `&R`.
    fn deref(&self) -> &Self::Target {
        self.res
    }
}

impl<'w, R: 'static> DerefMut for ResMut<'w, R> {
    /// Mutably dereferences to the underlying resource, allowing `ResMut<R>`
    /// to be used as if it were `&mut R`.
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.res
    }
}

impl<R: Resource + 'static> SystemParam for ResMut<'static, R> {
    /// Fetches a mutable reference to the resource of type `R` from the
    /// [`World`], if it has been registered.
    ///
    /// Returns `None` if no resource of type `R` exists in the world, making
    /// this system param `invalid` for that call (per the type-level
    /// documentation above) and causing the system to be skipped.
    ///
    /// # Safety
    ///
    /// The caller must ensure `world` is a valid, non-null pointer for the
    /// duration of this call, and that no other aliasing access (mutable or
    /// shared) to the same resource occurs while the returned `ResMut`'s
    /// borrow is alive.
    unsafe fn fetch(world: *mut World) -> Option<Self> {
        let world = unsafe { &mut *world };
        if let Some(res) = world.get_resource_mut::<R>() {
            return Some(ResMut::new(res));
        }
        None
    }
}
