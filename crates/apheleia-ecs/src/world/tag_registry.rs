use log::info;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::types::{NodeId, Tag};

#[derive(Default)]
pub struct TagRegistry {
    map: FxHashMap<Tag, SmallVec<[NodeId; 8]>>,
}
impl TagRegistry {
    pub fn tag_node(&mut self, node: NodeId, tag: usize) {
        self.map.entry(tag).and_modify(|v| v.push(node)).or_insert({
            let mut v = SmallVec::<[NodeId; _]>::new();
            v.push(node);
            v
        });
        info!("[ECS] Tagged NodeId: {} with Tag: {}", node, tag);
    }

    pub fn get_nodes(&self, tag: usize) -> Option<&SmallVec<[NodeId; 8]>> {
        self.map.get(&tag)
    }
}
