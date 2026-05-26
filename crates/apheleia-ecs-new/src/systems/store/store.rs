use std::collections::BTreeMap;

use rustc_hash::FxHashMap;

use crate::{
    SystemId,
    constants::SYSTEMS_MAX_PRIORITY_CHANGE,
    id_generator::IdGenerator,
    systems::{into_system::IntoSystem, store::stages::SystemRunStage, system::System},
    world::world_cell::UnsafeWorldCellMut,
};

pub struct SystemStore {
    id_generator: IdGenerator<SystemId>,

    stage_to_priority_ids: FxHashMap<SystemRunStage, BTreeMap<u8, SystemId>>,
    id_to_systems: FxHashMap<SystemId, Box<dyn System>>,
}
impl Default for SystemStore {
    fn default() -> Self {
        Self {
            id_generator: IdGenerator::new(0),

            stage_to_priority_ids: Default::default(),
            id_to_systems: Default::default(),
        }
    }
}

impl SystemStore {
    /// Register the given function the store and return the [`SystemId`]
    pub fn add_system<Marker>(
        &mut self,
        stage: SystemRunStage,
        priority: u8,
        system: impl IntoSystem<Marker>,
    ) -> SystemId {
        let id = self.id_generator.next();

        self.stage_to_priority_ids
            .entry(stage)
            .and_modify(|map| {
                let mut modify_priority = false;
                map.entry(priority).and_modify(|_| modify_priority = true).or_insert(id);

                if modify_priority {
                    let new_priority = (1..SYSTEMS_MAX_PRIORITY_CHANGE).find(|i| {
                        !map.contains_key(i)
                    }).or_else(|| panic!("Max priority change reached. Please manually change the priority for the given system")).unwrap();

                    map.entry(new_priority).or_insert(id);
                }
            })
            .or_insert_with(|| {
                let mut map: BTreeMap<u8, SystemId> = BTreeMap::default();
                map.insert(priority, id);
                map
            });
        self.id_to_systems
            .insert(id, IntoSystem::into_system(system));

        id
    }

    pub fn run_systems_for_stage(&mut self, stage: SystemRunStage, world: UnsafeWorldCellMut) {
        if let Some(map) = self.stage_to_priority_ids.get(&stage) {
            for (_, id) in map {
                let system = self
                    .id_to_systems
                    .get_mut(id)
                    .expect("Unexpected Error! System not found");
                system.run(world);
            }
        }
    }
}
