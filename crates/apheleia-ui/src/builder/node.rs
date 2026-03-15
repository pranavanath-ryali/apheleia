use std::{cell::RefCell, mem, rc::Rc};

use apheleia_core::types::vector::Vector2;
use tree_ds::prelude::Tree;

use crate::{
    EmptyNode, NodeId,
    node::{data::NodeData, node::NodeTrait},
    rootnode::node_storage::NodeStorage,
};

pub struct NodeBuilder<'a> {
    relations: &'a mut Tree<NodeId, NodeId>,
    node_storage: Rc<RefCell<NodeStorage>>,

    id: NodeId,
    class: String,
    parent_class: String,
    node_box: Box<dyn NodeTrait>,
    data: NodeData,
}
impl<'a> NodeBuilder<'a> {
    pub fn new(
        id: NodeId,
        class: &str,
        relations: &'a mut Tree<NodeId, NodeId>,
        node_storage: Rc<RefCell<NodeStorage>>,
    ) -> Self {
        NodeBuilder {
            relations,
            node_storage,

            id,
            class: class.to_string(),
            parent_class: "".to_string(),
            node_box: Box::new(EmptyNode),
            data: NodeData::default(),
        }
    }

    pub fn set_position(&mut self, position: Vector2) -> &mut Self {
        self.data.set_position(position);
        self
    }

    pub fn set_size(&mut self, size: Vector2) -> &mut Self {
        self.data.set_size(size);
        self
    }

    pub fn set_parent(&mut self, parent: &str) -> &mut Self {
        self.parent_class = parent.to_string();
        self
    }

    pub fn node(&mut self, node: Box<dyn NodeTrait>) -> &mut Self {
        self.node_box = node;
        self
    }

    pub fn build(&mut self) {
        let node = mem::replace(&mut self.node_box, Box::new(EmptyNode));
        self.node_storage
            .borrow_mut()
            .add_node(self.id, &self.class, node, self.data);
    }
}
