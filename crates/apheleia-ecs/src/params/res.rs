use std::ops::{Deref};
use crate::{traits::{resource::Resource, system_param::SystemParam}, world::World};

/// A [`SystemParam`] providing shared access to a [`Resource`] registered in
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
/// fn test_system(res: Res<Velocity>) {
///     assert_eq!(res.x, 10.0);
///     assert_eq!(res.y, 5.0);
/// }
///
/// world.add_resource(Velocity { x: 10.0, y: 5.0 });
/// world.add_system(SystemRunStage::Update, STAGE, test_system);
/// ```
pub struct Res<'w, R: 'static> {
    /// Shared reference to the resource fetched from the [`World`].
    res: &'w R,
}

impl<'w, R: 'static> Res<'w, R> {
    /// Wraps a shared reference to a resource.
    ///
    /// Not intended for direct use; instances are created internally when a
    /// system parameter of type `Res<R>` is fetched from the [`World`] (see
    /// the [`SystemParam`] impl below).
    pub(crate) fn new(res: &'w R) -> Self {
        Self { res }
    }
}

impl<'w, R: 'static> Deref for Res<'w, R> {
    type Target = R;

    /// Dereferences to the underlying resource, allowing `Res<R>` to be used
    /// as if it were `&R`.
    fn deref(&self) -> &Self::Target {
        self.res
    }
}

impl<R: Resource + 'static> SystemParam for Res<'static, R> {
    /// Fetches a shared reference to the resource of type `R` from the
    /// [`World`], if it has been registered.
    ///
    /// Returns `None` if no resource of type `R` exists in the world, making
    /// this system param `invalid` for that call (per the type-level
    /// documentation above) and causing the system to be skipped.
    ///
    /// # Safety
    ///
    /// The caller must ensure `world` is a valid, non-null pointer for the
    /// duration of this call, and that no conflicting mutable access to the
    /// world occurs while the returned `Res`'s borrow is alive.
    unsafe fn fetch(world: *mut World) -> Option<Self> {
        let world = unsafe { &*world };
        if let Some(res) = world.get_resource::<R>() {
            return Some(Res::new(res));
        }
        None
    }
}
