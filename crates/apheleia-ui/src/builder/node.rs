use std::mem::{replace, take};

use apheleia_core::types::Vector2;

use crate::{
    contexts::{
        commands::{AddExtensionToId, CreateNode, HookSystemToId},
        traits::ContextCommand,
    },
    extensions::traits::Extension,
    node::{EmptyNode, traits::NodeTrait},
    types::{NodeId, System, UpdateType},
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
    pub(crate) fn new(id: NodeId) -> Self {
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

    pub fn set_class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    pub fn set_parent(mut self, parent: &str) -> Self {
        self.parent_class = Some(parent.to_string());
        self
    }

    pub fn set_position(mut self, position: Vector2) -> Self {
        self.position = position;
        self
    }

    pub fn set_size(mut self, size: Vector2) -> Self {
        self.size = Some(size);
        self
    }

    pub fn add_extension(mut self, extension: Box<dyn Extension>) -> Self {
        self.commands
            .push(Box::new(AddExtensionToId(self.id, extension)));
        self
    }

    pub fn add_system(mut self, update_type: UpdateType, priority: isize, system: System) -> Self {
        self.commands.push(Box::new(HookSystemToId {
            id: self.id,
            update_type,
            priority,
            system,
        }));
        self
    }

    pub fn node<N: NodeTrait>(mut self, node: N) -> Self {
        self.node = Box::new(node);
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
}
