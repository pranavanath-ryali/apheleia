use std::{any::TypeId, collections::VecDeque, default, mem::replace};

use crate::node_definer::{EmptyNode, NodeDefiner};
use apheleia_ecs::{
    commands::{
        ContextCommand,
        node::{
            ComputeBoundsForNode, ComputeGlobalBoundsForNode, RelateChildWithParent, SetDataForNode
        }, tag::TagNode,
    }, nodedata::data::NodeData, runtime_expressions::{ExprVec, Expression}, tags::TagTrait, types::NodeId, world::World
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
        Self {
            id,
            parent_id,

            tags: Default::default(),
            data: NodeData::new(id),
            node: Box::new(EmptyNode),

            world,
            commands: Default::default()
        }
    }

    pub fn tag<T: TagTrait>(mut self, _tag: T) -> Self {
        self.tags.insert(TypeId::of::<T>());
        self
    }

    // Functions for NodeData
    pub fn position(mut self, expr: ExprVec) -> Self {
        self.data.position_expr(expr);
        self
    }
    pub fn size(mut self, expr: ExprVec) -> Self {
        self.data.size_expr(expr);
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
        self.commands.push_back(ComputeBoundsForNode::new(self.id));
        self.commands.push_back(ComputeGlobalBoundsForNode::new(self.id));
        for tag in self.tags {
            self.commands.push_back(TagNode::new(self.id, tag));
        }

        (self.commands, (self.id, self.node))
    }
}
