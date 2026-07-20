use std::{any::TypeId, collections::VecDeque, default, mem::replace};

use crate::{
    app::App,
    builder,
    node_definer::{EmptyNode, NodeDefiner},
};
use apheleia_ecs::{
    commands::{
        node::{
            ComputeBoundsForNode, ComputeGlobalBoundsForNode, RelateChildWithParent, SetDataForNode,
        },
        tag::TagNode,
    }, nodedata::NodeData, runtime_expressions::{ExprVec, Expression}, traits::{context_command::ContextCommand, tag::TagTrait}, types::NodeId, world::World
};
use indexmap::IndexSet;
use log::info;

/// [`NodeBuilder`] automates the creation process of a node during the setup process with any extensions and systems
pub struct NodeBuilder<'w> {
    id: NodeId,
    parent_id: NodeId,

    tags: IndexSet<TypeId>,
    data: NodeData,
    definer: Box<dyn NodeDefiner>,

    app: &'w mut App,
    children: Vec<(
        VecDeque<Box<dyn ContextCommand>>,
        (NodeId, Box<dyn NodeDefiner>),
    )>,
}
impl<'w> NodeBuilder<'w> {
    pub fn new(parent_id: NodeId, app: &'w mut App) -> NodeBuilder {
        let id = app.get_world().create_node();
        Self {
            id,
            parent_id,

            tags: Default::default(),
            data: NodeData::new(id),
            definer: Box::new(EmptyNode),

            app,
            children: Default::default(),
        }
    }

    pub fn create_child(mut self, f: impl FnOnce(NodeBuilder) -> NodeBuilder) -> Self {
        let builder = f(NodeBuilder::new(self.id, self.app));
        // println!("Hmm {:#?}", builder.build());
        self.children.push(builder.build());
        self
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
        self.definer = Box::new(node);
        self
    }

    pub(crate) fn build(
        self,
    ) -> (
        VecDeque<Box<dyn ContextCommand>>,
        (NodeId, Box<dyn NodeDefiner>),
    ) {
        let mut commands: VecDeque<Box<dyn ContextCommand>> = Default::default();

        commands.push_back(RelateChildWithParent::new(self.id, self.parent_id));
        commands.push_back(SetDataForNode::new(self.id, self.data));
        commands.push_back(ComputeBoundsForNode::new(self.id));
        commands.push_back(ComputeGlobalBoundsForNode::new(self.id));

        for tag in self.tags {
            commands.push_back(TagNode::new(self.id, tag));
        }

        for (child_commands, (id, definer)) in self.children {
            for c in child_commands {
                commands.push_back(c);
            }

            self.app.add_definer(id, definer);
        }

        (commands, (self.id, self.definer))
    }
}
