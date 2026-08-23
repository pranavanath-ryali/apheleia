pub mod node;
pub mod stage;
pub mod world;

use std::{collections::VecDeque, fmt::Debug};

use apheleia_core::{buffer::Buffer, renderer::Renderer, terminal, types::Vec2};
use apheleia_ecs::{
    traits::{event_marker::EventMarker, tag::TagTrait},
    types::NodeId,
    world::World,
};
use log::info;
use tree_ds::prelude::{Node, Tree};

use crate::node_definer::NodeDefiner;

#[derive(Debug)]
pub struct Quit;
impl EventMarker for Quit {}

#[derive(Debug)]
pub struct RenderFlip;
impl EventMarker for RenderFlip {}

pub struct App {
    world: World,
    renderer: Renderer,
    buffer: Buffer,

    definers: VecDeque<(NodeId, Box<dyn NodeDefiner>)>,
}
impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        info!("[APP] Creating new App object");
        let size = {
            let (width, height) = terminal::size().expect("Failed to get terminal size");
            Vec2 {
                x: width as u32,
                y: height as u32,
            }
        };
        info!("[APP] Got terminal size {:?}", size);

        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        relations.add_node(Node::new(0, None), None).ok();

        App {
            world: World::new(size),
            renderer: Default::default(),
            buffer: Buffer::new(size),

            definers: Default::default(),
        }
    }
}

impl Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").finish()
    }
}
impl TagTrait for App {}
