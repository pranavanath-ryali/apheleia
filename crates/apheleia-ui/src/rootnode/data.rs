use std::{cell::RefCell, rc::Rc};
use tree_ds::prelude::Tree;

use crate::{
    node::storage::NodeStorage,
    rootnode::{dirty_tracker::DirtyTracker, update_tracker::UpdateTracker},
    types::NodeId,
};

// pub struct RootNodeData<'a> {
//     pub relations: &'a mut Tree<NodeId, NodeId>,
//     pub node_storage: Rc<RefCell<NodeStorage>>,
//     pub dirty_tracker: Rc<RefCell<DirtyTracker>>,
//     pub update_tracker: Rc<RefCell<UpdateTracker>>,
// }
