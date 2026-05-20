use apheleia_ecs::World;
use apheleia_types::{NodeId, vec2::Vec2};
use tree_ds::prelude::Tree;

// fn calculate_global_position(&self, id: NodeId) -> Vec2 {
//     let mut position = self.node_store.get_data(id).unwrap().position;
//     self.relations
//         .get_ancestor_ids(&id)
//         .unwrap()
//         .iter()
//         .filter(|id| **id != 0_usize)
//         .for_each(|node_id| {
//             let pos = self.node_store.get_data(*node_id).unwrap().position;
//
//             position.x += pos.x;
//             position.y += pos.y;
//         });
//
//     position
// }
//
// pub fn calculate_global_position(
//     relations: &Tree<NodeId, NodeId>,
//     world: &World,
//     id: NodeId,
// ) -> Vec2 {
//     let mut position = world.get_data(id).unwrap().get_position();
//     relations
//         .get_ancestor_ids(&id)
//         .unwrap()
//         .iter()
//         .filter(|id| **id != 0_usize)
//         .for_each(|node_id| {
//             let pos = world.get_data(*node_id).unwrap().get_position();
//
//             position.x = pos.x;
//             position.y = pos.y;
//         });
//
//     position
// }
//
// pub fn calculate_global_size(
//     relations: &Tree<NodeId, NodeId>,
//     world: &World,
//     id: NodeId,
// ) -> Option<Vec2> {
//     let mut global_size = world
//         .get_data(id)
//         .unwrap()
//         .get_size()
//         .unwrap_or(Vec2::zero());
//
//     let parent_id = relations.get_ancestor_ids(&id).unwrap()[0];
//     if parent_id != 0 {
//         let parent_global_size = world
//             .get_data(parent_id)
//             .unwrap()
//             .get_global_size()
//             .unwrap_or(Vec2::zero());
//
//         if parent_global_size.x == 0 || parent_global_size.y == 0 {
//             return None;
//         }
//
//         let position = world.get_data(id).unwrap().get_position();
//         if position.x + global_size.x > parent_global_size.x - 1 {
//             if position.x >= parent_global_size.x {
//                 global_size.x = 0;
//             } else {
//                 global_size.x = parent_global_size.x - position.x;
//             }
//         }
//         if position.y + global_size.y > parent_global_size.y - 1 {
//             if position.y >= parent_global_size.y {
//                 global_size.y = 0;
//             } else {
//                 global_size.y = parent_global_size.y - position.y;
//             }
//         }
//     }
//
//     if global_size.x == 0 || global_size.y == 0 {
//         return None;
//     }
//     Some(global_size)
// }
