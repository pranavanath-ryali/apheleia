use std::collections::{HashMap, VecDeque};

use crate::{MAX_NODES, NodeId, node::data::NodeWrapperTrait};
use crate::node::data::NodeWrapper;
use apheleia_core::{
    buffer::Buffer,
    renderer::Renderer,
    terminal,
};

pub struct RootNode {
    width: u16,
    height: u16,

    available_node_ids: VecDeque<NodeId>,
    nodes: HashMap<NodeId, NodeWrapper>,

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

    pub fn add_node(&mut self, node: NodeWrapper) {
        if let Some(id) = self.get_id() {
            self.nodes.insert(id, node);
        }
    }

    pub fn initial_setup(&mut self) {
        for (id, data) in self.nodes.iter_mut() {
            data.node.initial_setup(&mut data.data);
        }
    }

    pub fn start(&mut self) {
        for (_, node) in self.nodes.iter_mut() {
            if let Some(size) = node.get_size() {
                if let Some(position) = node.get_position() {
                    let mut node_buffer = Buffer::new(size.0, size.1);
                    node.get_node().render(&mut node_buffer);
                    self.buffer
                        .render_buffer(position.0, position.1, &mut node_buffer);
                }
            }
        }

        self.renderer.flip(&mut self.buffer);
    }
}
