use std::collections::VecDeque;

use apheleia_core::{buffer::Buffer, renderer::Renderer};
use apheleia_types::{NodeId, Vec2};
use crossterm::terminal;
use tree_ds::prelude::{Node, Tree};

pub struct App {
    pub fps: u16,
    pub size: Vec2,
    pub running: bool,

    relations: Tree<NodeId, NodeId>,

    buffer: Buffer,
    renderer: Renderer,
}
impl Default for App {
    fn default() -> Self {
        let (width, height) = terminal::size().expect("Failed to get terminal size");
        let size = Vec2 {
            x: width,
            y: height,
        };

        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        _ = relations.add_node(Node::new(0, None), None);

        Self {
            fps: 15,
            size: size,
            running: true,

            relations,

            buffer: Default::default(),
            renderer: Default::default(),
        }
    }
}
