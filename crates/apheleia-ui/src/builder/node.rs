use std::{any::Any, cell::RefCell, mem, rc::Rc, vec};

use apheleia_core::types::vector::Vector2;
use tree_ds::prelude::{Node, Tree};

use crate::{
    extensions::{Extension, ExtensionStore},
    id_generator::IdGenerator,
    node::{EmptyNode, NodeTrait, data::NodeData, storage::NodeStorage},
    types::NodeId,
};

pub struct NodeBuilder<'a> {
    relations: &'a mut Tree<NodeId, NodeId>,
    node_storage: Rc<RefCell<NodeStorage>>,
    extension_store: Rc<RefCell<ExtensionStore>>,

    id: NodeId,
    class: String,
    parent_id: NodeId,
    data: NodeData,
}
impl<'a> NodeBuilder<'a> {
    pub fn new(
        id: NodeId,
        class: &str,
        relations: &'a mut Tree<NodeId, NodeId>,
        node_storage: Rc<RefCell<NodeStorage>>,
        extension_store: Rc<RefCell<ExtensionStore>>,
    ) -> Self {
        NodeBuilder {
            relations,
            node_storage,
            extension_store,

            id,
            class: class.to_string(),
            parent_id: 0,
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

    pub fn extension<T: Extension>(&mut self, extension: Box<T>) -> &mut Self {
        {
            let mut store = self.extension_store.borrow_mut();
            let ext_id = store.get_id();
            store.add_extension(ext_id, extension);
            _ = store.bind_extension::<T>(self.id, ext_id);
        }
        self
    }

    pub fn build<T: NodeTrait>(&mut self, node: T) {
        _ = self
            .relations
            .add_node(Node::new(self.id, None), Some(&self.parent_id));

        self.node_storage
            .borrow_mut()
            .add_node(self.id, &self.class, Box::new(node), self.data);
    }
}
