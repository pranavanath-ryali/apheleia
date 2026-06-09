use crate::{systems::system_param::SystemParam, world::world_cell::UnsafeWorldCellMut};

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
            if let Some(p1) = p1 {
                f(p1);
            }
        }
        run_system::<P1>(self, world);
        // run_system(self, world);
    }
}

// 2 Params
impl<Func, P1, P2> SystemParamFunction<(P1, P2)> for Func
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
            let p1 = P1::fetch(world.clone());
            let p2 = P2::fetch(world);
            if let Some(p1) = p1
                && let Some(p2) = p2
            {
                f(p1, p2);
            }
        }
        run_system::<P1, P2>(self, world);
        // run_system(self, world);
    }
}
