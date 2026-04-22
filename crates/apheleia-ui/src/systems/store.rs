use std::collections::{BTreeMap, HashMap};

use log::info;
use rustc_hash::FxHashMap;

use crate::{
    contexts::system::SystemContext,
    id_generator::{IdGenerator, IdGeneratorTrait},
    types::{NodeId, System, SystemId, UpdateType},
};

#[derive(Default)]
pub struct SystemStore {
    id_generator: IdGenerator<SystemId>,

    id_systems: FxHashMap<SystemId, System>,
    updatetype_nodesystems: HashMap<UpdateType, FxHashMap<NodeId, BTreeMap<isize, SystemId>>>,
}
impl SystemStore {
    pub fn add_system(
        &mut self,
        node_id: NodeId,
        update_type: UpdateType,
        priority: isize,
        system: System,
    ) {
        info!(
            "added system for NodeID: {}, priority: {}, type: {:?}",
            node_id, priority, update_type
        );
        let id = self.id_generator.next();
        let map = self.updatetype_nodesystems.entry(update_type).or_default();
        let node_functions = map.entry(node_id).or_default();

        node_functions.insert(priority, id);
        self.id_systems.insert(id, system);
    }

    pub fn run_systems_for_type(&self, update_type: UpdateType, ctx: &mut SystemContext) {
        if let Some(map) = self.updatetype_nodesystems.get(&update_type) {
            for (node_id, treemap) in map.iter() {
                for (_, system_id) in treemap.iter() {
                    let system = self.id_systems.get(system_id).unwrap();

                    ctx.set_id(*node_id);
                    system(ctx);
                }
            }
        }
    }

    pub fn run_systems_for_node_with_type(
        &self,
        update_type: UpdateType,
        node_id: NodeId,
        ctx: &mut SystemContext,
    ) {
        if let Some(map) = self.updatetype_nodesystems.get(&update_type)
            && let Some(treemap) = map.get(&node_id)
        {
            for (_, system_id) in treemap.iter() {
                let system = self.id_systems.get(system_id).unwrap();

                ctx.set_id(node_id);
                system(ctx);
            }
        }
    }
}
