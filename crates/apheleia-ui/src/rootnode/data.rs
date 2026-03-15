use std::{cell::RefCell, rc::Rc};

use tree_ds::prelude::Tree;

use crate::{
    NodeId,
    rootnode::{
        dirty_tracker::DirtyTracker, node_storage::NodeStorage, update_tracker::UpdateTracker,
    },
};

pub struct RootNodeData<'a> {
    pub relations: &'a mut Tree<NodeId, NodeId>,
    pub node_storage: Rc<RefCell<NodeStorage>>,
    pub dirty_tracker: Rc<RefCell<DirtyTracker>>,
    pub update_tracker: Rc<RefCell<UpdateTracker>>,
}
