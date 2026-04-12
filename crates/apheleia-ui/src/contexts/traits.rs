use std::{cell::RefCell, rc::Rc};

use crate::types::NodeId;

pub trait ContextCommand {
    fn execute(self: Box<Self>, rootnode_data: Rc<RefCell<World>>);
}
