use std::rc::Rc;

use tree_ds::prelude::Tree;

use crate::{NodeId, rootnode::node_storage::NodeStorage};

pub struct RootNodeData<'a> {
    pub relations: &'a mut Tree<NodeId, NodeId>,
    pub node_storage: Rc<NodeStorage>,
}
