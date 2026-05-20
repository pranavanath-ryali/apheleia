// use std::mem::take;
//
// use apheleia_types::{ContextCommand, vec2::Vec2};
//
// use crate::node_definer::NodeDefiner;
//
// pub struct NodeBuilder {
//     class: Option<String>,
//     parent_class: Option<String>,
//
//     position: Vec2,
//     size: Option<Vec2>,
//     node: Box<dyn NodeDefiner>,
//
//     commands: Vec<Box<dyn ContextCommand>>,
// }
// impl Default for NodeBuilder {
//     fn default() -> Self {
//         Self {
//             class: None,
//             parent_class: None,
//             position: Vec2::zero(),
//             size: None,
//             node: Box::new(EmptyNode::default()),
//             commands: vec![],
//         }
//     }
// }
// impl NodeBuilder {
//     pub fn set_class(mut self, class: &str) -> Self {
//         self.class = Some(class);
//         self
//     }
//
//     pub fn set_parent_class(mut self, parent: &str) -> Self {
//         self.parent_class = Some(parent);
//         self
//     }
//
//     pub fn set_position(mut self, position: Vec2) -> Self {
//         self.position = position;
//         self
//     }
//
//     pub fn set_size(mut self, size: Vec2) -> Self {
//         self.size = Some(size);
//         self
//     }
//
//     pub fn node(mut self, node: Box<dyn NodeDefiner>) -> Self {
//         self.node = node;
//         self
//     }
//
//     pub fn build(&mut self) -> Vec<Box<dyn ContextCommand>> {
//         take(&mut self.commands)
//     }
// }

use apheleia_types::{ContextCommand, node_data::NodeData, vec2::Vec2};
use indexmap::{IndexSet, indexset};

use crate::node_definer::NodeDefiner;

/// [`NodeBuilder`] automates the creation process of a node during the setup process with any extensions and systems
pub struct NodeBuilder {
    pub tags: IndexSet<usize>,

    data: NodeData,
    node: Box<dyn NodeDefiner>,

    commands: Vec<Box<dyn ContextCommand>>,
}
impl Default for NodeBuilder {
    fn default() -> Self {
        Self {
            tags: indexset! {},

            data: NodeData::new(Vec2::zero(), Vec2::zero()),
            node: todo!("Create Empty Node"),

            commands: vec![],
        }
    }
}

impl NodeBuilder {
    pub fn tag<const TAG: usize>(mut self) -> Self {
        self.tags.insert(TAG);
        self
    }

    pub fn position(mut self, position: Vec2) -> Self {
        self.data.position = position;
        self
    }
    pub fn size(mut self, size: Vec2) -> Self {
        self.data.size = size;
        self
    }

    pub fn node<N: NodeDefiner + 'static>(mut self, node: N) -> Self {
        self.node = Box::new(node);
        self
    }
}

#[cfg(test)]
mod tests {
    use apheleia_types::{node_data::NodeData, vec2::Vec2};
    use indexmap::indexset;

    use crate::{builder::node::NodeBuilder, node_definer::NodeDefiner};

    #[test]
    fn test_node_builder() {
        let builder = NodeBuilder::default()
            .tag::<0>()
            .tag::<123>()
            .position(Vec2 { x: 10, y: 10 })
            .size(Vec2 { x: 5, y: 3 });

        assert_eq!(builder.tags, indexset! {0, 123});
        assert_eq!(builder.data, NodeData {
            position: Vec2 { x: 10, y: 10 },
            size: Vec2 { x: 5, y: 3 }
        });
    }
}
