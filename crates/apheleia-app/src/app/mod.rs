use std::{collections::VecDeque, io, mem::take, time::Duration};

use apheleia_core::{buffer::Buffer, renderer::Renderer, terminal, types::Vec2};
use apheleia_ecs::{
    events::RenderDirty,
    systems::system::IntoSystem,
    types::{EventData, NodeId, SystemRunStage},
    world::World,
};
use crossterm::event::{Event, poll, read};
use indexmap::indexset;
use log::{info, warn};
use tree_ds::prelude::{Node, Tree};

use crate::{
    builder::node::NodeBuilder,
    context::node::NodeContext,
    into_resource::IntoResource,
    node_definer::NodeDefiner,
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
        info!("[APP] Creating new app object");
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
        resource.insert_into(&mut self.world);
        self
    }

    pub fn build_node(mut self, f: impl FnOnce(NodeBuilder) -> NodeBuilder) -> Self {
        info!("[APP] building new node");
        let builder = f(NodeBuilder::new(0, &mut self.world));
        let (mut commands, definer) = builder.execute();

        self.world.apppend_commands(&mut commands);
        self.definers.push_back(definer);
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
        self.world.execute_commands();
        info!("[APP] Executing NodeDefiners");
        let mut definers = take(&mut self.definers);
        for definer in definers.iter_mut() {
            let mut ctx = NodeContext::new(definer.0);
            definer.1.setup(&mut ctx);
            self.world.apppend_commands(ctx.get_commands());
        }
        self.world.execute_commands();
    }

    pub fn event(&mut self) -> io::Result<()> {
        info!("[APP] Event Poll");
        if poll(Duration::from_nanos(1_000_000_000 / 15))? {
            self.world.app_event_data = match read()? {
                Event::FocusGained => EventData::FocusGained,
                Event::FocusLost => EventData::FocusLost,
                Event::Key(key_event) => EventData::Keys(key_event),
                Event::Mouse(mouse_event) => EventData::Mouse(mouse_event),
                Event::Paste(_) => EventData::None,
                Event::Resize(width, height) => EventData::Resize(Vec2 { x: width, y: height }),
            };

            info!("[APP] Event Stage");
            self.world.current_stage = SystemRunStage::Event;
            self.world.run_systems_on_stage(SystemRunStage::Event);
        }
        Ok(())
    }
    fn update(&mut self) {
        info!("[APP] Update Stage");
        self.world.current_stage = SystemRunStage::Update;
        self.world.run_systems_on_stage(SystemRunStage::Update);

        self.world.execute_commands();
    }
    fn render_flip(&mut self) {
        info!("[APP] Render Flip");
        _ = self.renderer.render_flip(&mut self.buffer);

        let ids: Vec<NodeId> = self.world.get_registered_nodes().iter().copied().collect();
        for id in &ids {
            self.world.add_event(*id, RenderDirty);
        }
        self.world.current_stage = SystemRunStage::Render;
        self.world.run_systems_on_stage(SystemRunStage::Render);

        warn!("[APP] Rendering all node buffers into Main Buffer");
        for id in ids {
            let data = *self.world.get_nodedata(id).unwrap();
            if let Some(buffer) = self.world.get_buffer(id)
                && let Some(position) = data.global_position
            {
                self.buffer.render_buffer(position, buffer);
            }
        }
        info!("[APP] Rendering Main Buffer to stdout");
        self.renderer.render(&mut self.buffer);
    }
    fn render(&mut self) {
        // TODO: Use event based dirty render
        info!("[APP] Render Stage");
        self.world.current_stage = SystemRunStage::Render;
        self.world.run_systems_on_stage(SystemRunStage::Render);

        warn!("[APP] Rendering node buffers with RENDER_DIRTY event into Main Buffer");
        let set = take(
            self.world
                .get_nodes_with_event::<RenderDirty>()
                .unwrap_or(&mut indexset! {}),
        );
        for id in &set {
            info!("[APP] NodeId: {} was marked RENDER_DIRTY", id);
            let data = *self.world.get_nodedata(*id).unwrap();
            if let Some(buffer) = self.world.get_buffer(*id)
                && let Some(position) = data.global_position
            {
                self.buffer.render_buffer(position, buffer);
            }
        }
        if !set.is_empty() {
            info!("[APP] Rendering Main Buffer to stdout");
            self.renderer.render(&mut self.buffer);
        } else {
            info!("[APP] Skipped Rendering Main Buffer since no nodes are marked dirty");
        }
    }

    pub fn run(&mut self) {
        self.setup();
        _ = self.renderer.init();

        self.render_flip();

        while self.world.running {
            self.world.clear_events();
            self.world.clear_global_events();

            self.event();
            self.update();
            self.render();
        }

        _ = self.renderer.quit();
    }
}
