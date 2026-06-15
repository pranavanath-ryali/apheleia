use apheleia_core::types::Vec2;
use log::info;

use crate::{NodeId, world::World};

pub fn calculate_global_position(world: &World, id: NodeId) -> Vec2 {
    let mut position = world.get_nodedata(id).unwrap().position;

    world
        .get_relations()
        .get_ancestor_ids(&id)
        .unwrap()
        .iter()
        .filter(|id| **id != 0_usize)
        .for_each(|node_id| {
            let pos = world.get_nodedata(*node_id).unwrap().position;
            position.x += pos.x;
            position.y += pos.y;
        });

    info!(
        "[ECS] Calculated global position of NodeId: {} {:?}",
        id, position
    );
    position
}

pub fn calculate_global_size(world: &World, id: NodeId) -> Vec2 {
    let mut global_size = world.get_nodedata(id).unwrap().size;

    let relations = world.get_relations();
    let parent_id = relations.get_ancestor_ids(&id).unwrap()[0];
    if parent_id != 0 {
        let parent_global_size = world
            .get_nodedata(parent_id)
            .unwrap()
            .global_size
            .unwrap_or(Vec2::zero());

        if parent_global_size.x == 0 {
            global_size.x = 0;
        }
        if parent_global_size.y == 0 {
            global_size.y = 0;
        }

        let position = world.get_nodedata(id).unwrap().position;
        if position.x + global_size.x > parent_global_size.x - 1 {
            if position.x >= parent_global_size.x {
                global_size.x = 0;
            } else {
                global_size.x = parent_global_size.x - position.x;
            }
        }
        if position.y + global_size.y > parent_global_size.y - 1 {
            if position.y >= parent_global_size.y {
                global_size.y = 0;
            } else {
                global_size.y = parent_global_size.y - position.y;
            }
        }
    }

    if global_size.x == 0 {
        global_size.x = 0;
    }
    if global_size.y == 0 {
        global_size.y = 0;
    }

    info!(
        "[ECS] Calculated global size of NodeId: {} {:?}",
        id, global_size
    );
    global_size
}
