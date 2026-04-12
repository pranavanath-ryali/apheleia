use std::mem::{replace, take};

use apheleia_core::types::Vector2;

use crate::{
    contexts::{commands::CreateNode, traits::ContextCommand},
    node::{EmptyNode, data::NodeData, traits::NodeTrait},
    types::NodeId,
};

pub struct NodeBuilder {
    id: NodeId,
    class: Option<String>,
    parent_class: Option<String>,
    position: Vector2,
    size: Option<Vector2>,
    node: Box<dyn NodeTrait>,

    commands: Vec<Box<dyn ContextCommand>>,
}
impl NodeBuilder {
    pub fn new(id: NodeId) -> Self {
        NodeBuilder {
            id,
            class: None,
            parent_class: None,
            position: Vector2(0, 0),
            size: None,
            node: Box::new(EmptyNode),

            commands: vec![],
        }
    }

    pub fn with_class(&mut self, class: &str) -> &mut Self {
        self.class = Some(class.to_string());
        self
    }

    pub fn set_parent(&mut self, parent: &str) -> &mut Self {
        self.parent_class = Some(parent.to_string());
        self
    }

    pub fn set_position(&mut self, position: Vector2) -> &mut Self {
        self.position = position;
        self
    }

    pub fn set_size(&mut self, size: Vector2) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub(crate) fn build(&mut self) -> Vec<Box<dyn ContextCommand>> {
        let mut commands: Vec<Box<dyn ContextCommand>> = vec![];

        commands.push(Box::new(CreateNode {
            id: self.id,
            class: take(&mut self.class),
            parent_class: take(&mut self.parent_class),
            position: self.position,
            size: self.size,
            node: replace(&mut self.node, Box::new(EmptyNode)),
        }));
        commands.append(&mut self.commands);

        commands
    }

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
}
