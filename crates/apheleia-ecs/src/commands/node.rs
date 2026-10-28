use std::mem::take;

use crate::{commands::WorldCommand, nodedata::NodeData, types::{NodeId, traits::Extension}, world::World};

pub trait NodeSubCommand {
    fn execute(self: Box<Self>, id: NodeId, world: &mut World);
}

pub struct CreateNode {
    sub_commands: Vec<Box<dyn NodeSubCommand>>
}
impl WorldCommand for CreateNode {
    fn execute(mut self: Box<Self>, world: &mut crate::world::World) {
        let node_id = world.node_create();
        let commands = take(&mut self.sub_commands);
        for command in commands {
            command.execute(node_id, world);
        }
    }
}
impl CreateNode {
    pub fn new() -> Self {
        Self {
            sub_commands: Default::default()
        }
    }

    pub fn set_data(mut self, data: NodeData) -> Self {
        self.sub_commands.push(NodeSetData::new(data));
        self
    }

    pub fn add_extension<T: Extension>(mut self, ext: T) -> Self {
        self.sub_commands.push(NodeAddExtension::new(ext));
        self
    }

    pub fn build(self) -> Box<Self> {
        Box::new(self)
    }
}

pub struct NodeSetData(pub NodeData);
impl NodeSetData {
    pub fn new(data: NodeData) -> Box<Self> {
        Box::new(Self(data))
    }
}
impl NodeSubCommand for NodeSetData {
    fn execute(self: Box<Self>, id: NodeId, world: &mut World) {
        world.nodedata_write(id, self.0);
    }
}

pub struct NodeAddExtension<T: Extension>(pub T);
impl<T: Extension> NodeAddExtension<T> {
    pub fn new(ext: T) -> Box<Self> {
        Box::new(NodeAddExtension(ext))
    }
}
impl<T: Extension> NodeSubCommand for NodeAddExtension<T> {
    fn execute(self: Box<Self>, id: NodeId, world: &mut World) {
        world.add_extension(id, self.0);
    }
}


