use apheleia_types::NodeId;
use indexmap::{IndexSet, indexset};
use rustc_hash::FxHashMap;

type Tag = usize;

/// The [`TagStore`] will associate number of [`NodeId`]s to a unique unsigned int where the
/// developer can store as a const static
/// # Example:
/// const static BUTTON_TAG: usize = 0;
///
/// TODO: Maybe turn it into a macro enum for easier implementation of these const static.
#[derive(Default)]
pub struct TagStore {
    tagged_nodes: IndexSet<NodeId>,
    nodes_tag: FxHashMap<Tag, IndexSet<NodeId>>,
}
impl TagStore {
    pub fn tag_node<const TAG: Tag>(&mut self, node_id: NodeId) {
        assert!(!self.tagged_nodes.contains(&node_id));

        self.nodes_tag
            .entry(TAG)
            .and_modify(|v| {
                if !v.insert(node_id) {
                    panic!("Node ID: {} already tagged {}!", node_id, TAG);
                }
            })
            .or_insert_with(|| indexset! {node_id});
        self.tagged_nodes.insert(node_id);
    }

    pub fn get_nodes<const TAG: Tag>(&self) -> Option<&IndexSet<usize>> {
        self.nodes_tag.get(&TAG)
    }

    pub fn get_single_node<const TAG: Tag>(&self) -> Option<NodeId> {
        if let Some(nodes) = self.nodes_tag.get(&TAG)
            && !nodes.is_empty()
        {
            return Some(nodes[0]);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_node_tagging() {
        let mut store = TagStore::default();

        let node_0: NodeId = 1;
        let node_1: NodeId = 2;

        const FIRST_TAG: Tag = 1;
        const SECOND_TAG: Tag = 2;

        store.tag_node::<FIRST_TAG>(node_0);
        store.tag_node::<FIRST_TAG>(node_1);
        store.tag_node::<SECOND_TAG>(node_0);
    }

    #[test]
    fn test_get_node_from_tag() {
        let mut store = TagStore::default();

        let node_0: NodeId = 1;
        let node_1: NodeId = 2;
        let node_2: NodeId = 3;

        const FIRST_TAG: Tag = 1;
        const SECOND_TAG: Tag = 2;

        store.tag_node::<FIRST_TAG>(node_0);
        store.tag_node::<SECOND_TAG>(node_1);
        store.tag_node::<SECOND_TAG>(node_2);

        assert_eq!(store.get_single_node::<FIRST_TAG>(), Some(node_0));
        assert_eq!(store.get_single_node::<SECOND_TAG>(), Some(node_1));
    }
}
