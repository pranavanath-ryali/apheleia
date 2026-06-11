use std::{collections::VecDeque, mem::replace};

use crate::{
    commands::node::{CreateNode, SetDataForNode},
    node_definer::{EmptyNode, NodeDefiner},
};
use apheleia_core::types::Vec2;
use apheleia_ecs_new::{NodeId, command::ContextCommand, types::NodeData, world::World};
use indexmap::IndexSet;

/// [`NodeBuilder`] automates the creation process of a node during the setup process with any extensions and systems
pub struct NodeBuilder {
    id: NodeId,
    world: *mut World,

    tags: IndexSet<usize>,
    data: NodeData,
    node: Box<dyn NodeDefiner>,

    commands: VecDeque<Box<dyn ContextCommand>>,
}
impl NodeBuilder {
    pub fn new(world: &mut World) -> NodeBuilder {
        let id = world.create_node();

        let mut commands: VecDeque<Box<dyn ContextCommand>> = Default::default();
        commands.push_back(CreateNode::new(id));

        Self {
            id,
            world,

            tags: Default::default(),
            data: NodeData::default(),
            node: Box::new(EmptyNode),

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
    pub(crate) fn execute(mut self) -> (VecDeque<Box<dyn ContextCommand>>, (NodeId, Box<dyn NodeDefiner>)) {
        self.commands
            .push_back(SetDataForNode::new(self.id, self.data));
        (self.commands, (self.id, self.node))
    }
}
