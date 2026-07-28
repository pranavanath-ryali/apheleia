use core::panic;
use std::{any::TypeId, collections::BTreeMap};

use log::{info, warn};
use rustc_hash::FxHashMap;

use crate::{
    constants::{MAX_SYSTEMS, SYSTEMS_MAX_PRIORITY_CHANGE},
    id_generator::IdGenerator,
    stores::system::function_system::{IntoSystem, System},
    types::{SystemId, SystemRunStage},
    world::World,
};

pub mod function_system {
    use crate::traits::system_param::SystemParam;
    use crate::world::World;
    use std::{any::TypeId, marker::PhantomData};

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

pub(crate) struct SystemStore {
    // TODO: Maybe make ID Generator obsolete and register by system's typeid
    id_generator: IdGenerator<SystemId>,

    stage_to_priority_ids: FxHashMap<SystemRunStage, BTreeMap<u16, SystemId>>,
    id_to_systems: FxHashMap<SystemId, Box<dyn System>>,
    typesids: Vec<TypeId>,
}
impl Default for SystemStore {
    fn default() -> Self {
        Self {
            id_generator: IdGenerator::new(MAX_SYSTEMS),

            stage_to_priority_ids: Default::default(),
            id_to_systems: Default::default(),
            typesids: Default::default(),
        }
    }
}

impl SystemStore {
    /// Register the given function the store and return the [`SystemId`]
    pub fn add_system<Params: 'static>(
        &mut self,
        stage: SystemRunStage,
        priority: u16,
        system: impl IntoSystem<Params>,
    ) {
        let id = self.id_generator.next_id();
        let system = system.into_system();
        let system_typeid = system.id();
        let mut priority = priority;

        if self.typesids.contains(&system_typeid) {
            warn!("[ECS] System already added. Skipped adding!");
            return;
        }

        self.stage_to_priority_ids
            .entry(stage)
            .and_modify(|map| {
                let mut modify_priority = false;
                map.entry(priority)
                    .and_modify(|_| modify_priority = true)
                    .or_insert(id);

                if modify_priority {
                    let next_priority = ((priority + 1)..(priority + SYSTEMS_MAX_PRIORITY_CHANGE))
                        .find(|i| !map.contains_key(i))
                        .unwrap_or(priority);
                    let prev_priority = (priority.saturating_sub(1)
                        ..priority.saturating_sub(SYSTEMS_MAX_PRIORITY_CHANGE))
                        .find(|i| !map.contains_key(i))
                        .unwrap_or(priority);

                    let new_priority: u16;
                    if next_priority - priority > priority - prev_priority && prev_priority != priority {
                        new_priority = prev_priority;
                    } else if next_priority != priority {
                        new_priority = next_priority;
                    } else {
                        panic!("Max priority change reached. Please manually change the priority for the given system");
                    }

                    warn!(
                        "[ECS] Priority changed for system {:?} to: {} from: {}",
                        system_typeid, new_priority, priority
                    );

                    priority = new_priority;
                    map.entry(new_priority).or_insert(id);
                }
            })
            .or_insert_with(|| {
                let mut map: BTreeMap<_, SystemId> = BTreeMap::default();
                map.insert(priority, id);
                map
            });
        info!(
            "[ECS] Added system TypeId: {:?}, SystemId: {}; priority: {}",
            system_typeid, id, priority
        );
        self.id_to_systems.insert(id, system);
        self.typesids.push(system_typeid);
    }

    pub fn add_system_boxed(
        &mut self,
        stage: SystemRunStage,
        priority: u16,
        system: Box<dyn System>,
    ) {
        let id = self.id_generator.next_id();
        let system_typeid = system.id();
        let mut priority = priority;

        if self.typesids.contains(&system_typeid) {
            warn!("[ECS] System already added. Skipped adding!");
            return;
        }

        self.stage_to_priority_ids
            .entry(stage)
            .and_modify(|map| {
                let mut modify_priority = false;
                map.entry(priority).and_modify(|_| modify_priority = true).or_insert(id);

                if modify_priority {
                    let new_priority = ((priority + 1)..(priority + SYSTEMS_MAX_PRIORITY_CHANGE)).find(|i| {
                        !map.contains_key(i)
                    }).or_else(|| panic!("Max priority change reached. Please manually change the priority for the given system")).unwrap();

                    priority = new_priority;

                    warn!("[ECS] Priority changed for system {:?} to: {} from: {}", system_typeid, new_priority, priority);

                    map.entry(new_priority).or_insert(id);
                }
            })
            .or_insert_with(|| {
                let mut map: BTreeMap<_, SystemId> = BTreeMap::default();
                map.insert(priority, id);
                map
            });
        info!(
            "[ECS] Added system TypeId: {:?}, SystemId: {}; priority: {}",
            system_typeid, id, priority
        );
        self.id_to_systems.insert(id, system);
        self.typesids.push(system_typeid);
    }

    pub fn run_systems_for_stage(&mut self, stage: SystemRunStage, world: *mut World) {
        info!("ECS - Running systems on stage {:#?}", stage);
        if let Some(map) = self.stage_to_priority_ids.get(&stage) {
            for id in map.values() {
                info!("ECS - Running system ID: {}", id);
                let system = self
                    .id_to_systems
                    .get_mut(id)
                    .expect("Unexpected Error! System not found");
                system.run(world);
            }
        }
    }
}
