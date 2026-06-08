use apheleia_ecs_new::NodeId;

use crate::commands::Command;

#[derive(Debug)]
pub struct RelateChildNodeToParent {
    pub child: NodeId,
    pub parent: Option<NodeId>,
}
impl Command for RelateChildNodeToParent {
    fn execute(&self, app: &mut crate::app::App) {
        let relation = app.get_relation_mut();
    }
}
