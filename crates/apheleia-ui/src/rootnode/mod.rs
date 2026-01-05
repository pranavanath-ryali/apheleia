use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::time::Duration;

use crate::commands::{InitialCallContext, IntialCallCommands};
use crate::contexts::{EventContext, EventData, RenderContext, UpdateContext};
use crate::node::data::{NodeData, NodeWrapper};
use crate::node::node::NodeTrait;
use crate::{MAX_NODES, NodeId, node::data::NodeWrapperTrait};
use apheleia_core::types::vector::Vector2;
use apheleia_core::{buffer::Buffer, renderer::Renderer, terminal};
use crossterm::event::KeyModifiers;
use crossterm::{
    event::{KeyCode, poll, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tree_ds::prelude::{self, Node, TraversalStrategy, Tree};

pub enum EventType {
    Resize,
    Keys,
}

struct Relation {
    pub id: NodeId,
    pub children: Vec<Relation>,
}

pub struct RootNode {
    running: bool,

    node_count: NodeId,

    width: u16,
    height: u16,

    relations: Tree<NodeId, NodeId>,

    id_nodes: HashMap<NodeId, Box<dyn NodeTrait>>,
    id_data: HashMap<NodeId, NodeData>,
    class_id: HashMap<String, NodeId>,

    event_resize_nodes: Vec<NodeId>,
    event_keys_nodes: Vec<NodeId>,

    update_type_nodes: Vec<NodeId>,

    buffer: Buffer,
    renderer: Renderer,
}

impl Default for RootNode {
    fn default() -> Self {
        let size = terminal::size().unwrap();

        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        relations.add_node(Node::new(0, None), None);

        Self {
            running: false,

            node_count: 0,

            width: size.0,
            height: size.1,

            relations,

            id_nodes: HashMap::new(),
            id_data: HashMap::new(),
            class_id: HashMap::new(),

            event_resize_nodes: vec![],
            event_keys_nodes: vec![],
            update_type_nodes: vec![],

            buffer: Buffer::new(size.0, size.1),
            renderer: Renderer::default(),
        }
    }
}

impl RootNode {
    fn get_id(&mut self) -> NodeId {
        self.node_count += 1;
        self.node_count
    }

    pub fn add_node(
        &mut self,
        class: String,
        parent_class: String,
        node: Box<dyn NodeTrait>,
        data: NodeData,
    ) {
        let id = self.get_id();
        self.class_id.insert(class, id);
        self.id_nodes.insert(id, node);
        self.id_data.insert(id, data);

        if parent_class == "" {
            self.relations
                .add_node(Node::new(id, None), Some(&(0 as usize)));
        } else {
            self.relations.add_node(
                Node::new(id, None),
                Some(
                    self.class_id
                        .get(&parent_class)
                        .expect("Given parent class doesn't exist"),
                ),
            );
        }
    }

    // pub fn add_node(&mut self, node: NodeWrapper, parent_id: Option<NodeId>) -> Option<NodeId> {
    //     if let Some(id) = self.get_id() {
    //         self.nodes.insert(id, node);
    //
    //         if let Some(parent) = &parent_id {
    //             self.relations.add_node(Node::new(id, None), Some(parent));
    //         } else {
    //             self.relations
    //                 .add_node(Node::new(id, None), Some(&(0 as usize)));
    //         }
    //
    //         return Some(id);
    //     }
    //
    //     None
    // }

    pub fn initial_setup(&mut self) {
        for (id, data) in self.nodes.iter_mut() {
            let mut ctx = InitialCallContext::default();
            data.node.initial_setup(&mut ctx);

            for command in ctx.get_commands() {
                match command {
                    IntialCallCommands::SetSize(s) => {
                        data.set_size(*s);
                    }

                    IntialCallCommands::RegisterUpdate => {
                        self.update_type_nodes.insert(0, *id);
                    }
                    IntialCallCommands::RegisterEvent(EventType::Resize) => {
                        self.event_resize_nodes.insert(0, *id);
                    }
                    IntialCallCommands::RegisterEvent(EventType::Keys) => {
                        self.event_keys_nodes.insert(0, *id);
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

    fn event(&mut self) -> Result<(), Box<dyn Error>> {
        // event driven updates
        if poll(Duration::from_nanos(1_000_000_000 / 15))? {
            match read()? {
                crossterm::event::Event::Key(event) => {
                    if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL
                    {
                        self.running = false;
                    }

                    for id in self.event_keys_nodes.iter() {
                        let node = self.nodes.get_mut(id).unwrap();
                        let mut event_ctx = EventContext {
                            data: EventData::Keys(event),
                            position: *node.get_position(),
                            size: node.get_size(),
                        };
                        node.get_node_mut().event(&event_ctx);
                    }
                }
                crossterm::event::Event::Resize(width, height) => {
                    for id in self.event_keys_nodes.iter() {
                        let node = self.nodes.get_mut(id).unwrap();
                        let mut event_ctx = EventContext {
                            data: EventData::Resize(Vector2(width, height)),
                            position: *node.get_position(),
                            size: node.get_size(),
                        };
                        node.get_node_mut().event(&event_ctx);
                    }
                }
                crossterm::event::Event::FocusGained => {}
                crossterm::event::Event::FocusLost => {}
                crossterm::event::Event::Mouse(mouse_event) => {}
                crossterm::event::Event::Paste(_) => {}
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode();

        self.render();
        self.renderer.flip(&mut self.buffer);

        self.running = true;
        while (self.running) {
            self.event();
            self.update();

            self.render();
            self.renderer.update(&mut self.buffer);
        }

        disable_raw_mode();
        Ok(())
    }
}
