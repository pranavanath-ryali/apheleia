use std::{cell::RefCell, rc::Rc};

use crate::{rootnode::World, types::NodeId};

pub trait ContextCommand {
    fn execute(self: Box<Self>, id: NodeId, rootnode_data: Rc<RefCell<World>>);
}
