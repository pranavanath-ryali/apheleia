use std::{cell::RefCell, mem, rc::Rc, vec};

use apheleia_core::types::vector::Vector2;
use tree_ds::prelude::{Node, Tree};

use crate::{
    id_generator::IdGenerator,
    node::{EmptyNode, data::NodeData, node::NodeTrait},
    rootnode::node_storage::NodeStorage,
    types::NodeId,
};

pub struct NodeBuilder<'a> {
    id_generator: Rc<RefCell<IdGenerator<NodeId>>>,
    relations: &'a mut Tree<NodeId, NodeId>,
    node_storage: Rc<RefCell<NodeStorage>>,

    id: NodeId,
    class: String,
    parent_id: NodeId,
    node_box: Box<dyn NodeTrait>,
    data: NodeData,
}
impl<'a> NodeBuilder<'a> {
    pub fn new(
        id: NodeId,
        class: &str,
        id_generator: Rc<RefCell<IdGenerator<NodeId>>>,
        relations: &'a mut Tree<NodeId, NodeId>,
        node_storage: Rc<RefCell<NodeStorage>>,
    ) -> Self {
        NodeBuilder {
            id_generator,
            relations,
            node_storage,

            id,
            class: class.to_string(),
            parent_id: 0,
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
        if let Some(parent_id) = self.node_storage.borrow().get_id(parent) {
            self.parent_id = *parent_id;
        } else {
            panic!("Node of class '{}' doesn't exist.", { parent });
        }
        self
    }

    pub fn node(&mut self, node: Box<dyn NodeTrait>) -> &mut Self {
        self.node_box = node;
        self
    }

    pub fn build(&mut self) {
        let node = mem::replace(&mut self.node_box, Box::new(EmptyNode));
        _ = self
            .relations
            .add_node(Node::new(self.id, None), Some(&self.parent_id));
        self.node_storage
            .borrow_mut()
            .add_node(self.id, &self.class, node, self.data);
    }
}
