use log::info;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::NodeId;

#[derive(Default)]
pub struct TagRegistry {
    map: FxHashMap<usize, SmallVec<[NodeId; 4]>>,
}
impl TagRegistry {
    pub fn tag_node(&mut self, tag: usize, node: NodeId) {
        self.map.entry(tag).and_modify(|v| v.push(node)).or_insert({
            let mut v = SmallVec::<[NodeId; 4]>::new();
            v.push(node);
            v
        });
        info!("[ECS] Tagged NodeId: {} with Tag: {}", node, tag);
    }

    pub fn get_nodes(&self, tag: usize) -> Option<&SmallVec<[usize; 4]>> {
        self.map.get(&tag)
    }
}
