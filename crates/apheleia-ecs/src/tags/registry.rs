use std::any::TypeId;

use log::info;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::{tags::TagTrait, types::NodeId};

#[derive(Default)]
pub struct TagRegistry {
    map: FxHashMap<TypeId, SmallVec<[NodeId; 8]>>,
}
impl TagRegistry {
    pub fn tag_node<T: TagTrait>(&mut self, node: NodeId, _tag: T) {
        let tag_type = TypeId::of::<T>();
        self.map.entry(tag_type).and_modify(|v| v.push(node)).or_insert({
            let mut v = SmallVec::<[NodeId; _]>::new();
            v.push(node);
            v
        });
        info!("[ECS] Tagged NodeId: {} with Tag: {:#?}", node, _tag);
    }

    pub fn get_nodes<T: TagTrait>(&self) -> Option<&SmallVec<[NodeId; 8]>> {
        let tag = TypeId::of::<T>();
        self.map.get(&tag)
    }
}
