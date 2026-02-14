use std::collections::HashMap;

use apheleia_core::types::vector::Vector2;
use tree_ds::prelude::{TraversalStrategy, Tree};

use crate::{NodeId, node::data::NodeData};

pub fn calculate_global_position(
    id: NodeId,
    relations: &Tree<NodeId, NodeId>,
    id_data: &HashMap<NodeId, NodeData>,
) -> Vector2 {
    let mut position = id_data.get(&id).unwrap().position;
    relations
        .get_ancestor_ids(&id)
        .unwrap()
        .iter()
        .filter(|id| **id != 0_usize)
        .for_each(|node_id| {
            let pos = id_data.get(node_id).unwrap().position;
            position.0 += pos.0;
            position.1 += pos.1;
        });

    position
}
