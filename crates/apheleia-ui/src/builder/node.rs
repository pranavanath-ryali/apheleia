use std::{cell::RefCell, rc::Rc};

use crate::{NodeId, node::data::NodeData, rootnode::node_storage::NodeStorage};

pub struct NodeBuilder {
    node_storage: Rc<RefCell<NodeStorage>>,

    id: NodeId,
    data: NodeData,
}
impl NodeBuilder {
    pub fn new(id: NodeId, node_storage: Rc<RefCell<NodeStorage>>) -> Self {
        NodeBuilder {
            node_storage,

            id,
            data: NodeData::default(),
        }
    }
}
