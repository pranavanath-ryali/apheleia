use std::collections::{HashMap, VecDeque};

use crate::{MAX_NODES, NodeId, node::data::{NodeData, NodeDataTrait}};
use apheleia_core::{
    buffer::{Buffer, NodeBuffer},
    renderer::Renderer,
    terminal,
};

pub struct RootNode {
    width: u16,
    height: u16,

    available_node_ids: VecDeque<NodeId>,
    nodes: HashMap<NodeId, NodeData>,

    buffer: Buffer,
    renderer: Renderer,
}
impl RootNode {
    pub fn new() -> Self {
        let size = terminal::size().unwrap();

        let mut available_node_ids: VecDeque<NodeId> = VecDeque::new();
        for i in 1..MAX_NODES {
            available_node_ids.push_back(i);
        }

        Self {
            width: size.0,
            height: size.1,

            available_node_ids,
            nodes: HashMap::new(),

            buffer: Buffer::new(size.0, size.1),
            renderer: Renderer::new(),
        }
    }

    fn get_id(&mut self) -> Option<NodeId> {
        self.available_node_ids.pop_front()
    }

    pub fn add_node(&mut self, node: NodeData) {
        let id = self.get_id().unwrap();
        self.nodes.insert(id, node);
    }

    pub fn start(&mut self) {
        for (_, node) in self.nodes.iter_mut() {
            let mut node_buffer = NodeBuffer::new(node.get_width(), node.get_height());
            node.get_node().render(&mut node_buffer);
            self.buffer
                .render_node_buffer(node.get_x(), node.get_y(), &node_buffer);
        }

        self.renderer.flip(&mut self.buffer);
    }
}
