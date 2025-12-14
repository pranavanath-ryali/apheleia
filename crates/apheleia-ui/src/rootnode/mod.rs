use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::commands::{InitialCallContext, IntialCallCommands};
use crate::node::data::NodeWrapper;
use crate::{MAX_NODES, NodeId, node::data::NodeWrapperTrait};
use apheleia_core::style::{Style, StyleFlags};
use apheleia_core::{buffer::Buffer, renderer::Renderer, terminal};
use crossterm::event::{
    DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
    EnableFocusChange, EnableMouseCapture, KeyCode, poll, read,
};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

const UPDATE_RATE: u32 = 30;

pub enum UpdateType {
    EVENT,
    UPDATE,
}

pub struct RootNode {
    width: u16,
    height: u16,

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

        Self {
            width: size.0,
            height: size.1,

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

    pub fn add_node(&mut self, node: NodeWrapper) {
        if let Some(id) = self.get_id() {
            self.nodes.insert(id, node);
        }
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

                    IntialCallCommands::RegisterUpdateType(UpdateType::EVENT) => {
                        self.event_type_nodes.insert(0, *id);
                    }
                    IntialCallCommands::RegisterUpdateType(UpdateType::UPDATE) => {
                        self.update_type_nodes.insert(0, *id);
                    }
                }
            }
        }
    }

    fn render(&mut self) {
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

            // let frame_start_time = Instant::now();
            //
            // // Update shit
            //
            // let elapsed_time = frame_start_time.elapsed();
            // if elapsed_time < UPDATE_RATE {
            //     sleep(UPDATE_RATE - elapsed_time);
            // }

            self.render();
            self.renderer.update(&mut self.buffer);
        }
        disable_raw_mode();
        Ok(())
    }
}
