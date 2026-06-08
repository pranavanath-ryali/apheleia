use std::collections::VecDeque;

use apheleia_ecs_new::NodeId;

use crate::commands::ContextCommand;

pub struct NodeContext {
    id: NodeId,
    commands: VecDeque<Box<dyn ContextCommand>>
}
impl NodeContext {
    pub(crate) fn new(id: NodeId) -> Self {
        Self {
            id,
            commands: Default::default(),
        }
    }

    pub(crate) fn get_commands(&mut self) -> &mut VecDeque<Box<dyn ContextCommand>> {
        &mut self.commands
    }
}
