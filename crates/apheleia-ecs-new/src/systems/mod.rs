pub mod system_param;
pub mod into_system;
pub mod store;
mod system_param_function;
mod system;

use crate::systems::system_param_function::SystemParamFunction;

#[cfg(test)]
mod systems_tests {
    use std::ops::{Deref, DerefMut};

    use crate::{resources::Resource, systems::{system::{FunctionSystem, System}, system_param::SystemParam}, world::{World, world_cell::UnsafeWorldCellMut}};

    use super::*;

    /// A very basic Resource that stores a _i32_ value
    struct TestResource {
        value: i32,
    }
    impl Resource for TestResource {}

    struct AnotherTestResource {
        value: i32,
    }
    impl Resource for AnotherTestResource {}

    /// A very crude implementation of [`ResMut`].
    /// Used to fetch mutable access to resource added to [`World`]
    struct ResMut<'w, R: Resource> {
        resource: &'w mut R,
    }
    impl<R: Resource> SystemParam for ResMut<'_, R> {
        type Item<'a> = ResMut<'a, R>;

        fn fetch<'a>(mut world: UnsafeWorldCellMut<'a>) -> Option<Self::Item<'a>> {
            let resource = unsafe { world.get_resource_mut::<R>() };
            if let Some(resource) = resource {
                return Some(ResMut { resource });
            }
            None
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

        fn this_system_should_not_run(_: ResMut<AnotherTestResource>, _: ResMut<TestResource>) {
            panic!("This system shouldn't run")
        }

        let mut world = World::default();
        world.add_resource(TestResource { value: 10 });
        //
        // fn add_system<Marker>(system: impl IntoSystem<Marker>) -> <impl IntoSystem<Marker> as IntoSystem<Marker>>::System {
        //     let system = IntoSystem::into_system(system);
        //     system
        // }
        //
        // let system: Box<dyn System> = add_system(test_system);

        let mut system: Box<dyn System> =
            Box::new(FunctionSystem::<_, (ResMut<_>,)>::new(test_system));
        let mut another_system: Box<dyn System> =
            Box::new(FunctionSystem::<_, (ResMut<_>,)>::new(another_system));
        let mut final_system: Box<dyn System> =
            Box::new(FunctionSystem::<_, (ResMut<_>,)>::new(final_system));
        let mut useless_system: Box<dyn System> =
            Box::new(FunctionSystem::<_, (ResMut<_>, ResMut<_>)>::new(
                this_system_should_not_run,
            ));

        system.run(UnsafeWorldCellMut::from(&mut world));
        another_system.run(UnsafeWorldCellMut::from(&mut world));
        final_system.run(UnsafeWorldCellMut::from(&mut world));
        useless_system.run(UnsafeWorldCellMut::from(&mut world));
    }
}
