use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    io,
    mem::take,
    time::Duration,
};

use apheleia_core::{buffer::Buffer, renderer::Renderer, terminal, types::Vec2};
use apheleia_ecs::{
    commands::node::{ComputeAllBounds, ComputeAllGlobalBounds},
    stores::{
        events::{EventRegistry, RenderDirty},
        nodebuffer::NodeBufferStore,
        system::function_system::IntoSystem,
        tag::TagRegistry,
    },
    traits::{context_command::ContextCommand, event_marker::EventMarker, tag::TagTrait},
    types::{NodeId, SystemRunStage},
    world::World,
};
use crossterm::event::{Event, poll, read};
use log::{info, warn};
use tree_ds::prelude::{Node, Tree};

use crate::{
    builder::node::NodeBuilder, context::node::NodeContext, into_resource::IntoResource,
    node_definer::NodeDefiner, resources::app_events::AppEvents, types::EventData,
};

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
impl Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").finish()
    }
}
impl TagTrait for App {}
impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        info!("[APP] Creating new app object");
        let size = {
            let (width, height) = terminal::size().expect("Failed to get terminal size");
            Vec2 {
                x: width as u32,
                y: height as u32,
            }
        };

        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        _ = relations.add_node(Node::new(0, None), None);

        App {
            world: World::new(size),
            renderer: Default::default(),
            buffer: Buffer::new(size),

            definers: Default::default(),
        }
    }

    pub(crate) fn get_world(&mut self) -> &mut World {
        &mut self.world
    }
    pub(crate) fn push_command(&mut self, command: Box<dyn ContextCommand>) {
        self.world.add_command(command);
    }
    pub(crate) fn add_definer(&mut self, id: NodeId, definer: Box<dyn NodeDefiner>) {
        self.definers.push_back((id, definer));
    }

    pub fn add_resource(mut self, resource: impl IntoResource) -> Self {
        resource.insert_into(&mut self.world);
        self
    }

    pub fn create_node(mut self, f: impl FnOnce(NodeBuilder) -> NodeBuilder) -> Self {
        info!("[APP] building new node");

        let builder = f(NodeBuilder::new(0, &mut self));

        let (commmands, (id, definer)) = builder.build();
        for command in commmands {
            self.push_command(command);
        }
        self.add_definer(id, definer);

        self
    }

    pub fn add_system<Params: 'static>(
        mut self,
        stage: SystemRunStage,
        priority: u16,
        system: impl IntoSystem<Params>,
    ) -> Self {
        self.world.add_system(stage, priority, system);
        self
    }

    pub fn setup(&mut self) {
        info!("[APP] Setting up Default Resources");
        self.world.add_resource(NodeBufferStore::default());
        self.world.add_resource(AppEvents::default());
        self.world.add_resource(EventRegistry::default());
        self.world.add_resource(TagRegistry::default());

        self.world.execute_commands();
        info!("[APP] Executing NodeDefiners");
        loop {
            let definers = take(&mut self.definers);

            if definers.is_empty() {
                break;
            }

            let mut commands: VecDeque<Box<dyn ContextCommand>> = Default::default();
            for definer in definers {
                let mut ctx = NodeContext::new(definer.0, self);
                definer.1.setup(&mut ctx);
                commands.append(ctx.get_commands());
            }

            self.world.apppend_commands(&mut commands);
            self.world.execute_commands();
        }
    }

    pub fn event(&mut self) -> io::Result<()> {
        info!("[APP] Event Poll");
        let app_events = self.world.get_resource_mut::<AppEvents>().unwrap();
        if poll(Duration::from_nanos(1_000_000_000 / 15))? {
            match read()? {
                Event::FocusGained => {
                    app_events.event_data = EventData::FocusGained;
                }
                Event::FocusLost => {
                    app_events.event_data = EventData::FocusLost;
                }
                Event::Key(key_event) => {
                    app_events.event_data = EventData::Keys(key_event);
                }
                Event::Mouse(mouse_event) => {
                    app_events.event_data = EventData::Mouse(mouse_event);
                }
                Event::Paste(_) => {
                    todo!()
                }
                Event::Resize(x, y) => {
                    app_events.event_data = EventData::Resize(Vec2 {
                        x: x as u32,
                        y: y as u32,
                    });

                    // TODO
                    warn!(
                        "[APP] Terminal Resized. Added command to compute all bounds and global bounds {} {}",
                        x, y
                    );
                    self.world.terminal_size = Vec2 {
                        x: x as u32,
                        y: y as u32,
                    };
                    self.world.add_command(ComputeAllBounds::new());
                    self.world.add_command(ComputeAllGlobalBounds::new());
                    self.world
                        .get_resource_mut::<EventRegistry>()
                        .unwrap()
                        .add_global_event::<App, RenderFlip>();
                }
            };

            info!("[APP] Event Stage");
            self.world.current_stage = SystemRunStage::Event;
            self.world.run_systems_on_stage(SystemRunStage::Event);
        } else {
            app_events.event_data = crate::types::EventData::None;
        }
        Ok(())
    }
    fn update(&mut self) {
        info!("[APP] Update Stage");
        self.world.current_stage = SystemRunStage::Update;
        self.world.run_systems_on_stage(SystemRunStage::Update);
    }
    fn render_flip(&mut self) {
        info!("[APP] Render Flip");

        // TODO: Maybe create a resize and clear function in buffer
        self.buffer = Buffer::new(self.world.terminal_size);
        let mut buffers = self
            .world
            .get_resource_mut::<NodeBufferStore>()
            .unwrap()
            .clear_all_buffers();

        self.world.current_stage = SystemRunStage::RenderFlip;
        self.world.run_systems_on_stage(SystemRunStage::Render);

        warn!("[APP] Rendering all node buffers into Main Buffer");
        let ids: Vec<NodeId> = self.world.get_registered_nodes().iter().copied().collect();
        let buffers = self.world.get_resource_mut::<NodeBufferStore>().unwrap();
        for id in ids {
            if let Some(buffer) = buffers.get_buffer(id) {
                self.buffer.render_buffer(buffer);
            }
        }

        info!("[APP] Rendering Main Buffer to stdout");
        self.renderer.render_flip(&mut self.buffer);
    }
    fn render(&mut self) {
        // TODO: Refactor all this fucking SHIT
        info!("[APP] Render Stage");
        self.world.current_stage = SystemRunStage::Render;
        self.world.run_systems_on_stage(SystemRunStage::Render);

        let Some(set) = self
            .world
            .get_resource::<EventRegistry>()
            .unwrap()
            .get_local_events(RenderDirty)
        else {
            info!("[APP] Skipped rendering since no nodes are marked RenderDirty");
            return;
        };

        warn!("[APP] Rendering node buffers with RENDER_DIRTY event into Main Buffer");

        let mut set = set.iter().copied().collect::<Vec<NodeId>>();
        let mut order_map = HashMap::new();
        {
            let traverse = self
                .world
                .get_relations()
                .traverse(&0, tree_ds::prelude::TraversalStrategy::PreOrder)
                .unwrap();
            for (i, &id) in traverse.iter().enumerate() {
                order_map.insert(i, id);
            }
        }
        set.sort_by_key(|id| order_map.get(id).copied().unwrap_or(usize::MAX));

        {
            let buffers = self.world.get_resource_mut::<NodeBufferStore>().unwrap();
            for &id in set.iter() {
                if let Some(buffer) = buffers.get_buffer(id) {
                    self.buffer.clear_rect(buffer.global_position, buffer.size);
                }
            }
        }

        let mut render_set: Vec<usize> = vec![];
        for id in set.iter() {
            let parent = self.world.get_relations().get_ancestor_ids(id).unwrap()[0];
            if !render_set.contains(&parent) {
                render_set.push(parent);
            }
            if !render_set.contains(id) {
                render_set.push(*id);
            }
        }
        render_set.sort_by_key(|id| order_map.get(id).copied().unwrap_or(usize::MAX));
        {
            let buffers = self.world.get_resource_mut::<NodeBufferStore>().unwrap();
            for id in render_set {
                info!(
                    "[APP] NodeID: {} was marked RenderDirty. Rendering its NodeBuffer into main terminal buffer",
                    id
                );

                if let Some(buffer) = buffers.get_buffer(id) {
                    self.buffer.render_buffer(buffer);
                }
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
            self.world.execute_commands();
            self.update();
            self.world.execute_commands();

            if self
                .world
                .get_resource_mut::<EventRegistry>()
                .unwrap()
                .is_global_event::<App, RenderFlip>()
            {
                self.render_flip();
            } else {
                self.render();
            }
            self.world.execute_commands();

            let registry = self.world.get_resource_mut::<EventRegistry>().unwrap();
            if registry.is_global_event::<App, Quit>() {
                self.world.running = false;
            }
            self.world
                .get_resource_mut::<EventRegistry>()
                .unwrap()
                .clear();
            self.world
                .get_resource_mut::<NodeBufferStore>()
                .unwrap()
                .clear_first_access();
        }

        _ = self.renderer.quit();
    }
}
