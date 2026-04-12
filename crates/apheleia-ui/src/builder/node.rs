use std::{cell::RefCell, rc::Rc};

use apheleia_core::types::Vector2;
use tree_ds::prelude::Node;

use crate::{
    contexts::traits::ContextCommand,
    extensions::traits::Extension,
    node::{data::NodeData, traits::NodeTrait},
    types::{NodeId, System, UpdateType},
};

pub struct NodeBuilder {
    id: NodeId,
    class: Option<String>,
    parent_id: NodeId,
    data: NodeData,

    commands: Vec<Box<dyn ContextCommand>>,
}
impl NodeBuilder {
    pub fn new(id: NodeId) -> Self {
        NodeBuilder {
            id,
            class: None,
            parent_id: 0,
            data: NodeData::default(),

            commands: vec![],
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

    // pub fn set_parent(&mut self, parent: &str) -> &mut Self {
    //     if let Some(parent_id) = self.world.node_storage.get_id(parent) {
    //         self.parent_id = *parent_id;
    //     } else {
    //         panic!("Node of class '{}' doesn't exist.", { parent });
    //     }
    //     self
    // }

    // pub fn extension<T: Extension>(&mut self, extension: Box<T>) -> &mut Self {
    //     {
    //         let ext_id = self.world.extension_store.get_id();
    //         self.world.extension_store.add_extension(ext_id, extension);
    //         _ = self
    //             .world
    //             .extension_store
    //             .bind_extension::<T>(self.id, ext_id);
    //     }
    //     self
    // }

    // pub fn add_system(
    //     &mut self,
    //     update_type: UpdateType,
    //     priority: isize,
    //     system: System,
    // ) -> &mut Self {
    //     self.world
    //         .system_store
    //         .add_system(self.id, update_type, priority, system);
    //     self
    // }

    // pub fn build<T: NodeTrait>(&mut self, node: T) {
    //     _ = self
    //         .world
    //         .relations
    //         .add_node(Node::new(self.id, None), Some(&self.parent_id));

    //     self.world
    //         .node_storage
    //         .add_node(self.id, &self.class, Box::new(node), self.data);
    // }
    pub fn get_commands(&mut self) -> &mut Vec<Box<dyn ContextCommand>> {
        &mut self.commands
    }
}
