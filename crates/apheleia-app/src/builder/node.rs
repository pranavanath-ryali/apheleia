use std::{collections::VecDeque, mem::replace};

use crate::node_definer::{EmptyNode, NodeDefiner};
use apheleia_core::types::Vec2;
use apheleia_ecs::{
    commands::ContextCommand,
    commands::node::{
        CalculateGlobalPositionForNode, CalculateGlobalSizeForNode, RelateChildWithParent,
        SetDataForNode,
    },
    nodedata::data::NodeData,
    types::NodeId,
    world::World,
};
use indexmap::IndexSet;

/// [`NodeBuilder`] automates the creation process of a node during the setup process with any extensions and systems
pub struct NodeBuilder<'w> {
    id: NodeId,
    parent_id: NodeId,

    tags: IndexSet<usize>,
    data: NodeData,
    node: Box<dyn NodeDefiner>,

    world: &'w mut World,
    commands: VecDeque<Box<dyn ContextCommand>>,
}
impl<'w> NodeBuilder<'w> {
    pub fn new(parent_id: NodeId, world: &'w mut World) -> NodeBuilder {
        let id = world.create_node();

        let mut commands: VecDeque<Box<dyn ContextCommand>> = Default::default();

        Self {
            id,
            parent_id,

            tags: Default::default(),
            data: NodeData::default(),
            node: Box::new(EmptyNode),

            world,
            commands,
        }
    }

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
    pub(crate) fn execute(
        mut self,
    ) -> (
        VecDeque<Box<dyn ContextCommand>>,
        (NodeId, Box<dyn NodeDefiner>),
    ) {
        self.commands
            .push_back(RelateChildWithParent::new(self.id, self.parent_id));
        self.commands
            .push_back(SetDataForNode::new(self.id, self.data));
        self.commands
            .push_back(CalculateGlobalPositionForNode::new(self.id));
        self.commands
            .push_back(CalculateGlobalSizeForNode::new(self.id));

        (self.commands, (self.id, self.node))
    }
}
