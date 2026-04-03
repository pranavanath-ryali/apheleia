use std::{cell::RefCell, rc::Rc};

use crate::{
    rootnode::RootNodeData,
    systems::System,
    types::{NodeId, UpdateTypeNode},
};

pub struct NodeContext {
    id: NodeId,
    rootnode_data: Rc<RefCell<RootNodeData>>,
}
impl NodeContext {
    pub fn new(id: NodeId, rootnode_data: Rc<RefCell<RootNodeData>>) -> NodeContext {
        Self { id, rootnode_data }
    }

    pub fn get_id(&self) -> NodeId {
        self.id
    }

    pub fn add_system(&mut self, update_type: UpdateTypeNode, priority: isize, system: System) {
        self.rootnode_data.borrow_mut().system_store.add_system(
            self.id,
            update_type,
            priority,
            system,
        );
    }
}
