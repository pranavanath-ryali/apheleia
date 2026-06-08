use std::{collections::VecDeque, mem::{replace, take}};

use apheleia_core::types::Vec2;
use apheleia_ecs_new::{NodeId, types::NodeData};
use crate::{commands::node::{CreateNode, SetDataForNode, SetDefinerForNode}, id_generator::{IdGenerator, IdGeneratorTrait}, node_definer::{EmptyNode, NodeDefiner}};
use indexmap::IndexSet;

use crate::commands::ContextCommand;

/// [`NodeBuilder`] automates the creation process of a node during the setup process with any extensions and systems
pub struct NodeBuilder<'a> {
    id: NodeId,
    id_gen: &'a mut IdGenerator<NodeId>,

    tags: IndexSet<usize>,
    data: NodeData,
    node: Box<dyn NodeDefiner>,

    commands: VecDeque<Box<dyn ContextCommand>>
}
impl<'a> NodeBuilder<'a> {
    pub fn new(id_gen: &'a mut IdGenerator<NodeId>) -> NodeBuilder<'a> {
        let id = id_gen.next();

        let mut commands: VecDeque<Box<dyn ContextCommand>> = Default::default();
        commands.push_back(CreateNode::new(id));

        Self {
            id,
            id_gen,

            tags: Default::default(),
            data: NodeData::default(),
            node: Box::new(EmptyNode),

            commands
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
    pub(crate) fn get_commands(&mut self) -> &mut VecDeque<Box<dyn ContextCommand>> {
        self.commands.push_back(SetDataForNode::new(self.id, self.data));
        self.commands.push_back(SetDefinerForNode::new(self.id, replace(&mut self.node, Box::new(EmptyNode))));
        &mut self.commands
    }
}
