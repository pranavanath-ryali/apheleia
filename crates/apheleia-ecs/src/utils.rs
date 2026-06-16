use apheleia_core::types::Vec2;
use log::info;

use crate::{types::NodeId, world::World};

/// Calculates the global position of a node by accumulating
/// the relative positions of all its ancestors defined as position in [`NodeData`]
///
/// The root node (id `0`) is excluded as it acts as
/// the origin of the world and contributes no positional offset.
///
/// # Arguments
///
/// * `world` - A reference to the [`World`] containing node data and relationships.
/// * `id` - The [`NodeId`] of the node whose global position should be calculated.
///
/// # Returns
///
/// A [`Vec2`] representing the node's absolute position in world space.
///
/// # Panics
///
/// Panics if `id` does not exist in the [`World`], or if the ancestor chain
/// for `id` cannot be retrieved from the relation graph.
///
/// # Example
///
/// ```rust
/// let global_pos = calculate_global_position(&world, node_id);
/// println!("Node is at world position: ({}, {})", global_pos.x, global_pos.y);
/// ```
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

#[cfg(test)]
mod test_utils {
    use apheleia_core::types::Vec2;

    use crate::{nodedata::data::NodeData, world::World};

    #[test]
    fn test_calculate_global_position() {
        let mut world = World::default();
        let node_0 = world.create_node();
        let node_1 = world.create_node();
        let node_2 = world.create_node();

        world.

        world.set_data(node_0, NodeData {
            position: Vec2 { x: 3, y: 3 },
            ..Default::default()
        });
        world.set_data(node_1, NodeData {
            position: Vec2 { x: 3, y: 3 },
            ..Default::default()
        });
        world.set_data(node_2, NodeData {
            position: Vec2 { x: 3, y: 3 },
            ..Default::default()
        });
    }

    #[test]
    fn test_calculate_global_size() {

    }
}
