use std::{cell::RefCell, rc::Rc};

use apheleia_core::buffer::Buffer;

use crate::{
    rootnode::RootNodeData,
    types::{EventData, NodeId},
};

pub struct SystemContext<'a> {
    id: Option<NodeId>,
    event_data: Option<&'a EventData>,
    buffer: Option<&'a mut Buffer>,

    rootnode_data: Rc<RefCell<RootNodeData>>,
}
impl<'a> SystemContext<'a> {}
