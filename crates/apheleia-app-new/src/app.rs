use std::{collections::VecDeque, io, mem::take, time::Duration};

use apheleia_core::{buffer::Buffer, renderer::Renderer, terminal, types::Vec2};
use apheleia_ecs_new::{
    NodeId,
    systems::{stages::SystemRunStage, system::IntoSystem},
    world::World,
};
use crossterm::event::{KeyCode, KeyModifiers, poll};
use log::{info, warn};
use tree_ds::prelude::{Node, Tree};

use crate::{
    builder::node::NodeBuilder,
    context::node::NodeContext,
    into_resource::IntoResource,
    node_definer::NodeDefiner,
    resources::AppEvents,
    types::{self, EventData, EventType},
};

pub struct App {
    world: World,
    renderer: Renderer,
    buffer: Buffer,

    definers: VecDeque<(NodeId, Box<dyn NodeDefiner>)>,
}
impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        info!("APP: Creating new app object");
        let size = {
            let (width, height) = terminal::size().expect("Failed to get terminal size");
            Vec2 {
                x: width,
                y: height,
            }
        };

        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        _ = relations.add_node(Node::new(0, None), None);

        App {
            world: Default::default(),
            renderer: Default::default(),
            buffer: Buffer::new(size),

            definers: Default::default(),
        }
    }

    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn add_resource(mut self, resource: impl IntoResource) -> Self {
        info!("APP: Added resource: {:#?}", resource);
        resource.insert_into(&mut self.world);
        self
    }

    pub fn build_node(mut self, f: impl FnOnce(NodeBuilder) -> NodeBuilder) -> Self {
        info!("APP: building new node");
        let builder = f(NodeBuilder::new(0, &mut self.world));
        let (mut commands, definer) = builder.execute();
        info!("APP: commands returned from NodeBuilder: {:#?}", commands);

        self.world.apppend_commands(&mut commands);
        self.definers.push_back(definer);
        self
    }

    pub fn add_system<Params: 'static>(
        mut self,
        stage: SystemRunStage,
        priority: u8,
        system: impl IntoSystem<Params>,
    ) -> Self {
        info!(
            "APP: Added system - STAGE: {:?}, PRIORITY: {}",
            stage, priority
        );
        self.world.add_system(stage, priority, system);
        self
    }

    pub fn setup(&mut self) {
        self.world.execute_commands();
        let mut definers = take(&mut self.definers);
        for definer in definers.iter_mut() {
            let mut ctx = NodeContext::new(definer.0);
            definer.1.setup(&mut ctx);
            self.world.apppend_commands(ctx.get_commands());
        }
        self.world.execute_commands();

        // Setup [`World`] for event
        self.world.add_resource(AppEvents {
            data: EventData::None,
            event_type: EventType::None,
        });
    }

    pub fn event(&mut self) -> io::Result<()> {
        // TODO: Implement event function
        if poll(Duration::from_nanos(1_000_000_000 / 15))? {
            let resource = self.world.get_resource_mut::<AppEvents>().unwrap();
            match crossterm::event::read()? {
                crossterm::event::Event::FocusGained => {
                    resource.data = EventData::FocusGained;
                    resource.event_type = types::EventType::FocusGained;
                }
                crossterm::event::Event::FocusLost => {
                    resource.data = EventData::FocusLost;
                    resource.event_type = types::EventType::FocusLost;
                }
                crossterm::event::Event::Key(event) => {
                    resource.data = EventData::Keys(event);
                    resource.event_type = types::EventType::Keys;
                }
                crossterm::event::Event::Mouse(event) => {
                    resource.data = EventData::Mouse(event);
                    resource.event_type = types::EventType::Mouse;
                }
                crossterm::event::Event::Paste(_) => (), // TODO: Implement Paste Event
                crossterm::event::Event::Resize(width, height) => todo!(), // TODO:
                                                          // Implement
                                                          // Resize event
            }
    
            self.world.current_stage = SystemRunStage::Event;
            self.world.run_systems_on_stage(SystemRunStage::Event);
        }
        Ok(())
    }
    fn update(&mut self) {
        self.world.current_stage = SystemRunStage::Update;
        self.world.run_systems_on_stage(SystemRunStage::Update);
    }
    fn render_flip(&mut self) {
        _ = self.renderer.render_flip(&mut self.buffer);
        self.world.current_stage = SystemRunStage::Render;
        self.world.run_systems_on_stage(SystemRunStage::Render);
    }
    fn render(&mut self) {
        self.world.current_stage = SystemRunStage::Render;
        self.world.run_systems_on_stage(SystemRunStage::Render);

        let ids: Vec<NodeId> = self.world.get_registered_nodes().iter().copied().collect();
        for id in ids {
            let data = *self.world.get_nodedata(id).unwrap();
            if let Some(buffer) = self.world.get_buffer(id) && let Some(position) = data.global_position {
                self.buffer.render_buffer(position, buffer);
            }
        }
        self.renderer.render(&mut self.buffer);
    }

    pub fn run(&mut self) {
        self.setup();
        _ = self.renderer.init();

        self.render_flip();
        while self.world.running {
            self.event();
            self.update();
            self.render();
        }

        _ = self.renderer.quit();
    }
}
