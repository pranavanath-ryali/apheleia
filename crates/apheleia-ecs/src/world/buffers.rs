// use super::*;
//
// impl World {
//     /// Get the [`NodeBuffer`] for the given [`NodeId`]
//     /// Creates a new [`NodeBuffer`] if it doesn't exist for the node
//     ///
//     /// # Arguments
//     ///
//     /// * `node` - The [`NodeId`] to get the [`NodeBuffer`] for
//     #[inline]
//     pub fn get_buffer(&mut self, node: NodeId) -> Option<&mut Buffer> {
//         if let Some(&data) = self.get_nodedata(node) {
//             return self.buffer_store.get_buffer_mut(data, node);
//         }
//         None
//     }
// }
