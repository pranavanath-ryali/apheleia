use std::any::TypeId;

use crate::{stores::systems::function_system::{IntoSystem, System}, world::World};

pub mod function_system {
    use std::{any::TypeId, marker::PhantomData};
    use crate::types::traits::SystemParam;

    use crate::world::World;

    pub trait System: 'static {
        fn id(&self) -> TypeId;
        fn run(&mut self, world: *mut World);
    }

    pub trait IntoSystem<Params> {
        fn into_system(self) -> Box<dyn System>;
    }

    struct FunctionSystem<F, Params> {
        func: F,
        _marker: PhantomData<fn() -> Params>,
    }

    macro_rules! impl_into_system {
        ($($param:ident),*) => {
            impl<Func, $($param: SystemParam),*> IntoSystem<($($param,)*)> for Func
                where
                    Func: FnMut($($param,)*) + 'static,
            {
                fn into_system(self) -> Box<dyn System> {
                    Box::new(FunctionSystem {
                        func: self,
                        _marker: std::marker::PhantomData::<fn() -> ($($param,)*)>,
                    })
                }
            }

            #[allow(non_snake_case, unused_variables)]
            impl<Func, $($param: SystemParam),*> System for FunctionSystem<Func, ($($param,)*)>
                where
                    Func: FnMut($($param,)*) + 'static,
                {
                    fn id(&self) -> TypeId {
                        TypeId::of::<Self>()
                    }

                    fn run(&mut self, world: *mut World) {
                        unsafe {
                            if let ($(Some($param),)*) = ($($param::fetch(world),)*) {
                                (self.func)($($param,)*);
                            }
                        }
                    }
                }
        };
    }

    impl_into_system!();
    impl_into_system!(P0);
    impl_into_system!(P0, P1);
    impl_into_system!(P0, P1, P2);
    impl_into_system!(P0, P1, P2, P3);
    impl_into_system!(P0, P1, P2, P3, P4);
    impl_into_system!(P0, P1, P2, P3, P4, P5);
}

#[derive(PartialEq, Eq)]
pub enum SystemStage {
    Event = 0,

    PreUpdate = 1,
    Update = 2,
    PostUpdate = 3,

    PreRender = 4,
    Render = 5,
    PostRender = 6
}

pub(crate) struct SystemStore {
    systems: Vec<Vec<Box<dyn System>>>,
    registered_systems: Vec<TypeId>,
}
impl Default for SystemStore {
    fn default() -> Self {
        let mut systems = vec![
            vec![],

            vec![],
            vec![],
            vec![],

            vec![],
            vec![],
            vec![],
        ];

        Self {
            systems,
            registered_systems: Default::default()
        }
    }
}
impl SystemStore {
    /// Register the given function the store and return the [`SystemId`]
    pub fn add_system<Params: 'static>(
        &mut self,
        stage: SystemStage,
        system: impl IntoSystem<Params>,
    ) {
        let system = system.into_system();
        let system_typeid = system.id();

        if self.registered_systems.contains(&system_typeid) {
            return;
        }

        self.registered_systems.push(system_typeid);

        let stage = &mut self.systems[stage as usize];
        stage.push(system);
    }

    pub fn add_system_boxed(&mut self, stage: SystemStage, system: Box<dyn System>) {
        let system_typeid = system.id();

        if self.registered_systems.contains(&system_typeid) {
            return;
        }
        self.registered_systems.push(system_typeid);

        let stage = &mut self.systems[stage as usize];
        stage.push(system);
    }

    pub fn run_stage(&mut self, stage: SystemStage, world: *mut World) {
        let systems = &mut self.systems[stage as usize];
        for system in systems {
            system.run(world);
        }
    }
}
