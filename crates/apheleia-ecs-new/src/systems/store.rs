use rustc_hash::FxHashMap;

use crate::{SystemId, id_generator::IdGenerator, systems::system::System};

/// Stores & manages all the systems.
pub struct SystemStore {
    id_generator: IdGenerator<SystemId>,

    id_to_systems: FxHashMap<SystemId, Box<dyn System>>
}
impl Default for SystemStore {
    fn default() -> Self {
        Self {
            id_generator: IdGenerator::new(0),

            id_to_systems: Default::default()
        }
    }
}
