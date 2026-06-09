use std::mem::{replace, take};

use apheleia_ecs_new::{NodeId, types::NodeData};

use crate::{commands::ContextCommand, node_definer::NodeDefiner};

#[derive(Debug)]
pub struct CreateNode(pub NodeId);

impl CreateNode {
    pub fn new(id: NodeId) -> Box<Self> {
        Box::new(Self(id))
    }
}
impl ContextCommand for CreateNode {
    fn execute(&mut self, app: &mut crate::app::App) {
    }
}

#[derive(Debug)]
pub struct SetDataForNode(pub NodeId, pub NodeData);

impl SetDataForNode {
    pub fn new(id: NodeId, data: NodeData) -> Box<Self> {
        Box::new(Self(id, data))
    }
}
impl ContextCommand for SetDataForNode {
    fn execute(&mut self, app: &mut crate::app::App) {
        let world = app.get_world_mut();
        world.set_data(self.0, self.1);
    }
}

#[derive(Debug)]
pub struct SetDefinerForNode(pub NodeId, pub Box<dyn NodeDefiner>);

impl SetDefinerForNode {
    pub fn new(id: NodeId, definer: Box<dyn NodeDefiner>) -> Box<Self> {
        Box::new(Self(id, definer))
    }
}
impl ContextCommand for SetDefinerForNode {
    fn execute(&mut self, app: &mut crate::app::App) {
        app.add_definer(self.0, take(&mut self.1));
    }
}
