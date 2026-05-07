pub mod builder;
pub mod commands;
pub mod contexts;
pub mod dirty_tracker;
pub mod node_definer;
pub mod utils;

use std::{collections::VecDeque, io, mem::take, time::Duration};

use apheleia_core::{buffer::Buffer, renderer::Renderer};
use apheleia_ecs::World;
use apheleia_types::{ContextCommand, EventData, EventType, NodeId, vec2::Vec2};
use crossterm::{
    event::{KeyCode, KeyModifiers, poll, read},
    terminal,
};
use rustc_hash::FxHashMap;
use tree_ds::prelude::{Node, Tree};

use crate::{
    builder::node::NodeBuilder,
    contexts::node::NodeContext,
    node_definer::NodeDefiner,
    utils::{calculate_global_position, calculate_global_size},
};

pub struct App {
    pub fps: u16,
    pub size: Vec2,
    pub running: bool,

    relations: Tree<NodeId, NodeId>,
    nodeid_definer: FxHashMap<NodeId, Box<dyn NodeDefiner>>,
    dirty_tracker: DirtyTracker,

    world: World,

    commands: Vec<Box<dyn ContextCommand>>,

    buffer: Buffer,
    renderer: Renderer,
}
impl App {
    pub fn new() -> Self {
        let (width, height) = terminal::size().expect("Failed to get terminal size");
        let size = Vec2 {
            x: width,
            y: height,
        };

        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        _ = relations.add_node(Node::new(0, None), None);

        Self {
            fps: 15,
            size,
            running: true,

            relations,
            nodeid_definer: Default::default(),
            dirty_tracker: Default::default(),

            world: Default::default(),

            commands: vec![],

            buffer: Buffer::new(size),
            renderer: Renderer::default(),
        }
    }

    pub fn setup(&mut self) -> io::Result<()> {
        self.renderer.init()?;

        let ids: Vec<NodeId> = self
            .relations
            .traverse(&0, tree_ds::prelude::TraversalStrategy::PreOrder)
            .unwrap()
            .iter()
            .filter(|v| **v != 0_usize)
            .copied()
            .collect();

        for id in ids {
            let data = self.world.get_data(id).unwrap();
            let mut ctx = NodeContext::new(id, data.get_position(), data.get_size());

            self.nodeid_definer.get_mut(&id).unwrap().setup(&mut ctx);

            // TODO: Append Commmands

            let global_position = calculate_global_position(&self.relations, &self.world, id);
            let global_size = calculate_global_size(&self.relations, &self.world, id);

            let data = self.world.get_data_mut(id).unwrap();
            data.set_global_position(global_position);
            data.set_global_size(global_size);
        }

        Ok(())
    }

    pub fn event(&mut self) -> io::Result<()> {
        // TODO: Implement event function
        let mut event_type: EventType = EventType::None;
        let mut event_data: EventData = EventData::None;
        if poll(Duration::from_nanos(1_000_000_000 / self.fps as u64))? {
            match read()? {
                crossterm::event::Event::FocusGained => event_type = EventType::FocusGained,
                crossterm::event::Event::FocusLost => event_type = EventType::FocusLost,
                crossterm::event::Event::Key(key_event) => {
                    if key_event.modifiers == KeyModifiers::CONTROL
                        && key_event.code == KeyCode::Char('c')
                    {
                        self.running = false;
                    }

                    event_type = EventType::Keys;
                    event_data = EventData::Keys(key_event);
                }
                crossterm::event::Event::Mouse(event) => {
                    event_type = EventType::Mouse;
                    event_data = EventData::Mouse(event)
                }
                crossterm::event::Event::Paste(_) => todo!(),
                crossterm::event::Event::Resize(width, height) => {
                    event_type = EventType::Resize;
                    event_data = EventData::Resize(Vec2 {
                        x: width,
                        y: height,
                    });
                }
            }
        }

        if event_type != EventType::None {
            // TODO: Handle this
            // let mut world = SystemView {
            //     relations: &self.relations,
            //
            //     node_storage: &self.node_store,
            //     extension_store: &mut self.extension_store,
            //     resource_store: &mut self.resource_store,
            // };
            //
            // let mut ctx = SystemContext::new_event(&event_data, &mut world);
            // self.system_store
            //     .run_systems_for_type(crate::types::UpdateType::Event(event_type), &mut ctx);
            // self.commands.append(ctx.get_commands());
        }
        Ok(())
    }

    pub fn update(&mut self) {
        let ids = self.dirty_tracker.take_setup();
        for id in ids {
            // TODO: Handle this
        }

        // TODO: Systems registered for updates
    }

    fn render_node(&mut self, id: NodeId) {}

    pub fn render(&mut self) {
        let ids = self.dirty_tracker.take_render();
        for id in ids {
            self.render_node(id);
        }
    }
    pub fn render_flip(&mut self) -> io::Result<()> {
        let ids: Vec<NodeId> = self
            .relations
            .traverse(&0, tree_ds::prelude::TraversalStrategy::PreOrder)
            .unwrap()
            .to_vec();
        for id in ids {
            self.render_node(id);
        }

        self.renderer.render_flip(&mut self.buffer)?;
        self.dirty_tracker.clear_render();

        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.render_flip()?;

        while self.running {
            self.event()?;
            self.update();
            self.render();
        }

        Ok(())
    }

    // Functions for developers
    pub fn create_node(&mut self, f: impl FnOnce(NodeBuilder) -> NodeBuilder) {
        let mut builder = f(NodeBuilder::default());
        self.commands.append(builder.build());
        // TODO: Deal with commands
    }
}

pub struct EmptyNode;
impl NodeDefiner for EmptyNode {
    fn setup(&mut self, ctx: &mut NodeContext) {}
}
