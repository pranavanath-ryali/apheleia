use std::{cell::RefCell, rc::Rc};

use apheleia_core::types::Vector2;
use tree_ds::prelude::Node;

use crate::{
    extensions::traits::Extension,
    node::{data::NodeData, traits::NodeTrait},
    types::{NodeId, System, UpdateType},
    world::BuilderView,
};

pub struct NodeBuilder<'a> {
    rootnode_data: &'a mut BuilderView<'a>,

    id: NodeId,
    class: String,
    parent_id: NodeId,
    data: NodeData,
}
impl<'a> NodeBuilder<'a> {
    pub fn new(id: NodeId, class: &str, rootnode_data: &'a mut BuilderView<'a>) -> Self {
        NodeBuilder {
            rootnode_data,

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
        if let Some(parent_id) = self.rootnode_data.node_storage.get_id(parent) {
            self.parent_id = *parent_id;
        } else {
            panic!("Node of class '{}' doesn't exist.", { parent });
        }
        self
    }

    pub fn extension<T: Extension>(&mut self, extension: Box<T>) -> &mut Self {
        {
            let ext_id = self.rootnode_data.extension_store.get_id();
            self.rootnode_data
                .extension_store
                .add_extension(ext_id, extension);
            _ = self
                .rootnode_data
                .extension_store
                .bind_extension::<T>(self.id, ext_id);
        }
        self
    }

    pub fn add_system(
        &mut self,
        update_type: UpdateType,
        priority: isize,
        system: System,
    ) -> &mut Self {
        self.rootnode_data
            .system_store
            .add_system(self.id, update_type, priority, system);
        self
    }

    pub fn build<T: NodeTrait>(&mut self, node: T) {
        _ = self
            .rootnode_data
            .relations
            .add_node(Node::new(self.id, None), Some(&self.parent_id));

        self.rootnode_data
            .node_storage
            .add_node(self.id, &self.class, Box::new(node), self.data);
    }
}
