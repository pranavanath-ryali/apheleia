use std::{any::TypeId, collections::VecDeque, mem::replace};

use crate::node_definer::{EmptyNode, NodeDefiner};
use apheleia_core::types::Vec2;
use apheleia_ecs::{
    commands::{
        ContextCommand,
        node::{
            CalculateGlobalPositionForNode, CalculateGlobalSizeForNode, RelateChildWithParent,
            SetDataForNode,
        }, tag::TagNode,
    },
    nodedata::data::NodeData,
    tags::TagTrait,
    types::NodeId,
    world::World,
};
use indexmap::IndexSet;

/// [`NodeBuilder`] automates the creation process of a node during the setup process with any extensions and systems
pub struct NodeBuilder<'w> {
    id: NodeId,
    parent_id: NodeId,

    tags: IndexSet<TypeId>,
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

    pub fn tag<T: TagTrait>(mut self, _tag: T) -> Self {
        self.tags.insert(TypeId::of::<T>());
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

        for tag in self.tags {
            self.commands.push_back(TagNode::new(self.id, tag));
        }

        (self.commands, (self.id, self.node))
    }
}
