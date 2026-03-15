use tree_ds::prelude::Tree;

use crate::NodeId;

pub struct RootNodeDup {
    width: u16,
    height: u16,
    running: bool,

    node_count: NodeId,

    relations: Tree<NodeId, NodeId>,
}
