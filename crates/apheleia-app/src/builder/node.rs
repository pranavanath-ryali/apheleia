use std::mem::take;

use apheleia_types::{ContextCommand, vec2::Vec2};

use crate::node_definer::NodeDefiner;

pub struct NodeBuilder {
    class: Option<String>,
    parent_class: Option<String>,

    position: Vec2,
    size: Option<Vec2>,
    node: Box<dyn NodeDefiner>,

    commands: Vec<Box<dyn ContextCommand>>,
}
impl Default for NodeBuilder {
    fn default() -> Self {
        Self {
            class: None,
            parent_class: None,
            position: Vec2::zero(),
            size: None,
            node: Box::new(EmptyNode::default()),
            commands: vec![],
        }
    }
}
impl NodeBuilder {
    pub fn set_class(mut self, class: &str) -> Self {
        self.class = Some(class);
        self
    }

    pub fn set_parent_class(mut self, parent: &str) -> Self {
        self.parent_class = Some(parent);
        self
    }

    pub fn set_position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    pub fn set_size(mut self, size: Vec2) -> Self {
        self.size = Some(size);
        self
    }

    pub fn node(mut self, node: Box<dyn NodeDefiner>) -> Self {
        self.node = node;
        self
    }

    pub fn build(&mut self) -> Vec<Box<dyn ContextCommand>> {
        take(&mut self.commands)
    }
}
