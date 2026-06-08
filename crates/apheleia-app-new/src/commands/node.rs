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
    fn execute(&self, app: &mut crate::app::App) {
        todo!()
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
    fn execute(&self, app: &mut crate::app::App) {
        todo!()
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
    fn execute(&self, app: &mut crate::app::App) {
        todo!()
    }
}
