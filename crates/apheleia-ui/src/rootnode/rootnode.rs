use std::{error::Error, io::stdout, rc::Rc};

use apheleia_core::{buffer::Buffer, renderer::Renderer};
use crossterm::terminal::{self, enable_raw_mode};
use tree_ds::prelude::{Node, Tree};

use crate::{NodeId, rootnode::node_storage::NodeStorage};

pub struct RootNodeDup {
    width: u16,
    height: u16,
    running: bool,

    node_count: NodeId,

    relations: Tree<NodeId, NodeId>,
    node_storage: Rc<NodeStorage>,

    buffer: Buffer,
    renderer: Renderer,
}
impl Default for RootNodeDup {
    fn default() -> Self {
        let (width, height) = terminal::size().unwrap();

        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        _ = relations.add_node(Node::new(0, None), None);

        RootNodeDup {
            node_count: 0,
            running: false,
            width,
            height,

            relations,
            node_storage: Rc::new(NodeStorage::default()),

            buffer: Buffer::new(width, height),
            renderer: Renderer {
                width,
                height,
                stdout: stdout(),
            },
        }
    }
}
impl RootNodeDup {
    fn get_id(&mut self) -> NodeId {
        self.node_count += 1;
        self.node_count
    }

    fn initial_setup(&mut self) {}
    fn event(&mut self) {}
    fn update(&mut self) {}
    fn render_flip(&mut self) {}
    fn render(&mut self) {}
    pub fn run(&mut self) {
        _ = enable_raw_mode();

        self.initial_setup();
        self.render_flip();

        self.running = true;
        while self.running {
            self.event();
            self.update();
            self.render();
        }
    }
}
