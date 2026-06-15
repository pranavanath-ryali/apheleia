use std::{any::TypeId, collections::BTreeMap};

use log::{info, warn};
use rustc_hash::FxHashMap;

use crate::{
    SystemId,
    constants::{MAX_SYSTEMS, SYSTEMS_MAX_PRIORITY_CHANGE},
    id_generator::IdGenerator,
    systems::{
        stages::SystemRunStage,
        system::{IntoSystem, System},
    }, world::World,
};

pub struct SystemStore {
    // TODO: Maybe make ID Generator obsolete and register by system's typeid
    id_generator: IdGenerator<SystemId>,

    stage_to_priority_ids: FxHashMap<SystemRunStage, BTreeMap<u8, SystemId>>,
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
        priority: u8,
        system: impl IntoSystem<Params>,
    ) {
        let id = self.id_generator.next();
        let system = system.into_system();
        let system_typeid = system.id();

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

                    warn!("[ECS] Priority changed for system {:?} to: {} from: {}", system_typeid, new_priority, priority);
                    map.entry(new_priority).or_insert(id);
                }
            })
            .or_insert_with(|| {
                let mut map: BTreeMap<u8, SystemId> = BTreeMap::default();
                map.insert(priority, id);
                map
            });
        info!("[ECS] Added system TypeId: {:?}, SystemId: {}", system_typeid, id);
        self.id_to_systems.insert(id, system);
        self.typesids.push(system_typeid);
    }

    // TODO: Refactor
    pub fn add_system_boxed(&mut self, stage: SystemRunStage, priority: u8, system: Box<dyn System>) {
        let id = self.id_generator.next();
        let system_typeid = system.id();

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

                    warn!("[ECS] Priority changed for system {:?} to: {} from: {}", system_typeid, new_priority, priority);

                    map.entry(new_priority).or_insert(id);
                }
            })
            .or_insert_with(|| {
                let mut map: BTreeMap<u8, SystemId> = BTreeMap::default();
                map.insert(priority, id);
                map
            });
        info!("[ECS] Added system TypeId: {:?}, SystemId: {}", system_typeid, id);
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
