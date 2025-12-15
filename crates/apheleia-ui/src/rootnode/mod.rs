use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::time::Duration;

use crate::commands::{InitialCallContext, IntialCallCommands};
use crate::contexts::{RenderContext, UpdateContext};
use crate::node::data::NodeWrapper;
use crate::{MAX_NODES, NodeId, node::data::NodeWrapperTrait};
use apheleia_core::types::vector::Vector2;
use apheleia_core::{buffer::Buffer, renderer::Renderer, terminal};
use crossterm::{
    event::{KeyCode, poll, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tree_ds::prelude::{self, Node, TraversalStrategy, Tree};

pub enum UpdateType {
    Event,
    Update,
}

struct Relation {
    pub id: NodeId,
    pub children: Vec<Relation>,
}

pub struct RootNode {
    width: u16,
    height: u16,

    relations: Tree<NodeId, NodeId>,

    available_node_ids: VecDeque<NodeId>,
    nodes: HashMap<NodeId, NodeWrapper>,

    event_type_nodes: Vec<NodeId>,
    update_type_nodes: Vec<NodeId>,

    buffer: Buffer,
    renderer: Renderer,
}

impl Default for RootNode {
    fn default() -> Self {
        let size = terminal::size().unwrap();

        let mut available_node_ids: VecDeque<NodeId> = VecDeque::new();
        for i in 1..MAX_NODES {
            available_node_ids.push_back(i);
        }

        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        relations.add_node(Node::new(0, None), None);

        Self {
            width: size.0,
            height: size.1,

            relations,

            available_node_ids,
            nodes: HashMap::new(),

            event_type_nodes: vec![],
            update_type_nodes: vec![],

            buffer: Buffer::new(size.0, size.1),
            renderer: Renderer::default(),
        }
    }
}

impl RootNode {
    fn get_id(&mut self) -> Option<NodeId> {
        self.available_node_ids.pop_front()
    }

    pub fn add_node(&mut self, node: NodeWrapper, parent_id: Option<NodeId>) -> Option<NodeId> {
        if let Some(id) = self.get_id() {
            self.nodes.insert(id, node);

            if let Some(parent) = &parent_id {
                self.relations.add_node(Node::new(id, None), Some(parent));
            } else {
                self.relations
                    .add_node(Node::new(id, None), Some(&(0 as usize)));
            }

            return Some(id);
        }

        None
    }

    pub fn initial_setup(&mut self) {
        for (id, data) in self.nodes.iter_mut() {
            let mut ctx = InitialCallContext::default();
            data.node.initial_setup(&mut ctx);

            for command in ctx.get_commands() {
                match command {
                    IntialCallCommands::SetSize(s) => {
                        data.set_size(*s);
                    }

                    IntialCallCommands::RegisterUpdateType(UpdateType::Event) => {
                        self.event_type_nodes.insert(0, *id);
                    }
                    IntialCallCommands::RegisterUpdateType(UpdateType::Update) => {
                        self.update_type_nodes.insert(0, *id);
                    }
                }
            }
        }
    }

    fn render(&mut self) {
        for id in self
            .relations
            .traverse(&(0 as usize), TraversalStrategy::PreOrder)
            .unwrap()
            .iter()
        {
            if *id == 0_usize {
                continue;
            }

            let mut positions: Vector2 = Vector2(0, 0);
            self.relations
                .get_ancestor_ids(id)
                .unwrap()
                .iter()
                .filter(|v| **v != 0_usize)
                .for_each(|i| {
                    let pos = self.nodes.get(i).unwrap().get_position();
                    positions.0 += pos.0;
                    positions.1 += pos.1;
                });

            let node = self.nodes.get_mut(id).unwrap();
            if let Some(size) = node.get_size() {
                let pos = node.get_position();

                let mut ctx = RenderContext {
                    position: *pos,
                    size: size,
                };

                let mut node_buffer = Buffer::new(size.0, size.1);
                node.get_node().render(&mut ctx, &mut node_buffer);
                self.buffer.render_buffer(
                    positions.0 + pos.0,
                    positions.1 + pos.1,
                    &mut node_buffer,
                );
            }
        }
    }

    fn update(&mut self) {
        for id in self.update_type_nodes.iter() {
            let node = self.nodes.get_mut(id).unwrap();
            let mut ctx = UpdateContext {
                position: *node.get_position(),
                size: node.get_size(),
            };
            node.get_node_mut().update(&mut ctx);
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode();

        self.render();
        self.renderer.flip(&mut self.buffer);

        const UPDATE_RATE: Duration = Duration::from_nanos(1_000_000_000 / 15);
        loop {
            // event driven updates
            if poll(UPDATE_RATE)? {
                match read()? {
                    crossterm::event::Event::Key(event) => {
                        if event.code == KeyCode::Esc {
                            break;
                        }

                        if event.code == KeyCode::Char('a') {
                            for id in self.event_type_nodes.iter() {
                                self.nodes.get_mut(id).unwrap().node.event();
                            }
                        }
                    }
                    crossterm::event::Event::Resize(width, height) => {}
                    _ => {}
                }
            }

            self.update();

            self.render();
            self.renderer.update(&mut self.buffer);
        }

        disable_raw_mode();
        Ok(())
    }
}
