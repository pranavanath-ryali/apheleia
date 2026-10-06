use apheleia_types::id_gen::IdGenerator;
use tree_ds::prelude::{Node, Tree};

use crate::{stores::nodedata::NodeDataStore, types::NodeId};

pub struct World {
    nodeid_gen: IdGenerator<NodeId>,
    relations: Tree<NodeId, NodeId>,

    nodedata: NodeDataStore,
}
impl Default for World {
    fn default() -> Self {
        let mut relations = Tree::<NodeId, NodeId>::new(None);
        relations.add_node(Node::new(0, None), None).ok();

        Self {
            nodeid_gen: IdGenerator::new(1), // 0 is reserved for terminal 'root'
            relations,

            nodedata: Default::default(),
        }
    }
}
