use apheleia_ecs::types::NodeId;
use log::info;

use crate::{app::App, builder::node::NodeBuilder, node_definer::NodeDefiner};

impl App {
    pub(crate) fn add_definer(&mut self, id: NodeId, definer: Box<dyn NodeDefiner>) {
        self.definers.push_back((id, definer));
    }

    pub fn create_node(mut self, f: impl FnOnce(NodeBuilder) -> NodeBuilder) -> Self {
        info!("[APP] building new node");

        let builder = f(NodeBuilder::new(0, &mut self));

        let (commmands, (id, definer)) = builder.build();
        for command in commmands {
            self.push_command(command);
        }
        self.add_definer(id, definer);

        self
    }
}
