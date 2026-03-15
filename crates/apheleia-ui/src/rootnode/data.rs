use std::rc::Rc;

use tree_ds::prelude::Tree;

use crate::{
    NodeId,
    rootnode::{dirty_tracker::DirtyTracker, node_storage::NodeStorage},
};

pub struct RootNodeData<'a> {
    pub relations: &'a mut Tree<NodeId, NodeId>,
    pub node_storage: Rc<NodeStorage>,
    pub dirty_tracker: Rc<DirtyTracker>,
}
