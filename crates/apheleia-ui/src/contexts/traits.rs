use std::{cell::RefCell, rc::Rc};

use crate::{rootnode::RootNodeData, types::NodeId};

pub trait ContextCommand {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: Rc<RefCell<RootNodeData>>);
}
