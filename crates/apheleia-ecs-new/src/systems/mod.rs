pub mod system_param;

use std::marker::PhantomData;

use crate::{systems::system_param::SystemParam, world::world_cell::UnsafeWorldCellMut};

/// The trait that is used to store the actual functions.
/// This is used in cases like a Vec of systems, specifically
/// ```rust
/// let mut systems: Vec<Box<dyn System>>
/// ```
/// In the above example, the vector can store multiple types of systems that have number of
/// parameters. A [`System`] is a generic trait used for storage.
pub trait System: 'static {
    fn run(&mut self, world: UnsafeWorldCellMut);
}

/// The actual type that stores the function and the type of system it is
pub struct FunctionSystem<F: SystemParamFunction<Marker>, Marker: 'static> {
    func: F,
    marker: PhantomData<Marker>,
}
impl<F: SystemParamFunction<Marker>, Marker> FunctionSystem<F, Marker> {
    pub fn new(func: F) -> Self {
        Self {
            func,
            marker: PhantomData,
        }
    }
}
impl<F: SystemParamFunction<Marker>, Marker> System for FunctionSystem<F, Marker> {
    /// Run the system
    fn run(&mut self, world: UnsafeWorldCellMut) {
        self.func.run(world);
    }
}

/// The actual function which also knows the _type_ of system. The type of a system is really just what
/// parameters it takes in. [`SystemParamFunction`] fetches the actual values from the provided [`SystemParam`] and runs the system.
pub trait SystemParamFunction<Marker>: 'static {
    fn run(&mut self, world: UnsafeWorldCellMut);
}

// TODO: Replace with a macro implementation
// 0 Params
impl<Func> SystemParamFunction<()> for Func
where
    Func: 'static,
    for<'a> &'a mut Func: FnMut(),
{
    fn run(&mut self, _: UnsafeWorldCellMut) {
        fn run_system(mut f: impl FnMut()) {
            f();
        }
        run_system(self);
    }
}

// 1 Params
impl<Func, P1: SystemParam + 'static> SystemParamFunction<(P1,)> for Func
where
    Func: 'static,
    for<'w> &'w mut Func: FnMut(P1::Item<'w>),
{
    fn run(&mut self, world: UnsafeWorldCellMut) {
        fn run_system<'w, P1: SystemParam>(
            mut f: impl FnMut(P1::Item<'w>),
            world: UnsafeWorldCellMut<'w>,
        ) {
            let p1 = P1::fetch(world);
            f(p1);
        }
        run_system::<P1>(self, world);
        // run_system(self, world);
    }
}

// 2 Params
impl<Func, P1,P2> SystemParamFunction<(P1,P2)> for Func
where
    P1: SystemParam + 'static,
    P2: SystemParam + 'static,

    Func: 'static,
    for<'w> &'w mut Func: FnMut(P1::Item<'w>, P2::Item<'w>),
{
    fn run(&mut self, world: UnsafeWorldCellMut) {
        fn run_system<'w, P1: SystemParam, P2: SystemParam>(
            mut f: impl FnMut(P1::Item<'w>, P2::Item<'w>),
            world: UnsafeWorldCellMut<'w>,
        ) {
            let p1 = P1::fetch(world);
            let p2 = P2::fetch(world);
            f(p1, p2);
        }
        run_system::<P1, P2>(self, world);
        // run_system(self, world);
    }
}

#[cfg(test)]
mod tests {
    use std::ops::{Deref, DerefMut};

    use crate::{resources::Resource, world::World};

    use super::*;

    /// A very basic Resource that stores a _i32_ value
    struct TestResource {
        value: i32,
    }
    impl Resource for TestResource {}

    /// A very crude implementation of [`ResMut`].
    /// Used to fetch mutable access to resource added to [`World`]
    struct ResMut<'w, R: Resource> {
        resource: &'w mut R,
    }
    impl<R: Resource> SystemParam for ResMut<'_, R> {
        type Item<'a> = ResMut<'a, R>;

        fn fetch<'a>(mut world: UnsafeWorldCellMut<'a>) -> Self::Item<'a> {
            ResMut {
                resource: unsafe { world.get_resource_mut::<R>().unwrap() }, // resource: unsafe { (*world.world_mut()).get_resource_mut::<R>().unwrap() },
            }
        }
    }

    impl<'w, R: Resource> Deref for ResMut<'w, R> {
        type Target = &'w mut R;

        fn deref(&self) -> &Self::Target {
            &self.resource
        }
    }

    impl<'w, R: Resource> DerefMut for ResMut<'w, R> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.resource
        }
    }

    #[test]
    fn test_basic_systems_calling() {
        fn test_system(res: ResMut<TestResource>) {
            assert_eq!(res.value, 10);
        }

        fn another_system(mut res: ResMut<TestResource>) {
            res.value = 256;
        }

        fn final_system(res: ResMut<TestResource>) {
            assert_eq!(res.value, 256);
        }

        let mut world = World::default();
        world.add_resource(TestResource { value: 10 });

        let mut system: Box<dyn System> =
            Box::new(FunctionSystem::<_, (ResMut<_>,)>::new(test_system));
        let mut another_system: Box<dyn System> =
            Box::new(FunctionSystem::<_, (ResMut<_>,)>::new(another_system));
        let mut final_system: Box<dyn System> =
            Box::new(FunctionSystem::<_, (ResMut<_>,)>::new(final_system));

        system.run(UnsafeWorldCellMut::from(&mut world));
        another_system.run(UnsafeWorldCellMut::from(&mut world));
        final_system.run(UnsafeWorldCellMut::from(&mut world));
    }
}
