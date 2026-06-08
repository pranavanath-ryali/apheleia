use std::{collections::VecDeque, io, mem::take, time::Duration};

use apheleia_core::{buffer::Buffer, renderer::Renderer, terminal, types::Vec2};
use apheleia_ecs_new::{
    NodeId,
    world::{World, world_cell::UnsafeWorldCellMut},
};
use crate::id_generator::{IdGenerator, IdGeneratorTrait};
use crossterm::event::{KeyCode, KeyModifiers, poll};
use log::info;
use tree_ds::prelude::{Node, Tree};

use crate::{
    builder::node::NodeBuilder, commands::ContextCommand, context::node::NodeContext,
    into_resource::IntoResource, node_definer::NodeDefiner, tag::tag_registry::TagRegistry,
};

pub struct App {
    pub is_running: bool,

    id_gen: IdGenerator<NodeId>,
    tag_registry: TagRegistry,

    relations: Tree<NodeId, NodeId>,
    world: World,
    renderer: Renderer,
    buffer: Buffer,

    definers: VecDeque<(NodeId, Box<dyn NodeDefiner>)>,
    commands: VecDeque<Box<dyn ContextCommand>>,
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
            is_running: true,
            
            id_gen: IdGenerator::new(0),
            tag_registry: Default::default(),

            relations,
            world: Default::default(),
            renderer: Default::default(),
            buffer: Buffer::new(size),

            definers: Default::default(),
            commands: Default::default(),
        }
    }

    pub fn get_relation_mut(&mut self) -> &mut Tree<NodeId, NodeId> {
        &mut self.relations
    }

    pub fn add_command(&mut self, command: Box<dyn ContextCommand>) {
        info!("COMMAND: Added new command - {:#?}", command);
        self.commands.push_back(command);
    }

    pub(crate) fn execute_commands(&mut self) {
        let commands = take(&mut self.commands);
        for command in commands {
            command.execute(self);
        }
    }

    pub fn add_resource<T: IntoResource>(mut self, resource: T) -> Self {
        resource.insert_into(UnsafeWorldCellMut::from(&mut self.world));
        self
    }

    pub fn build_node(mut self, f: impl FnOnce(NodeBuilder) -> NodeBuilder) -> Self {
        info!("APP: building new node");
        let mut builder = f(NodeBuilder::new(&mut self.id_gen));
        let mut commands = take(builder.get_commands());
        info!("APP: commands returned from NodeBuilder: {:#?}", commands);
        self.commands.append(&mut commands);
        self
    }

    pub fn setup(&mut self) {
        self.execute_commands();
        // let mut definers = take(&mut self.definers);
        // for definer in definers.iter_mut() {
        //     let mut ctx = NodeContext::new(definer.0);
        //     definer.1.setup(&mut ctx);
        // }
    }

    pub fn event(&mut self) -> io::Result<()> {
        // TODO: Implement event function
        // let mut event_type: EventType = EventType::None;
        // let mut event_data: EventData = EventData::None;
        if poll(Duration::from_nanos(1_000_000_000 / 15 as u64))? {
            match crossterm::event::read()? {
                // crossterm::event::Event::FocusGained => event_type = EventType::FocusGained,
                // crossterm::event::Event::FocusLost => event_type = EventType::FocusLost,
                crossterm::event::Event::Key(key_event) => {
                    if key_event.modifiers == KeyModifiers::CONTROL
                        && key_event.code == KeyCode::Char('c')
                    {
                        self.is_running = false;
                    }

                    // event_type = EventType::Keys;
                    // event_data = EventData::Keys(key_event);
                }
                // crossterm::event::Event::Mouse(event) => {
                //     event_type = EventType::Mouse;
                //     event_data = EventData::Mouse(event)
                // }
                // crossterm::event::Event::Paste(_) => todo!(),
                // crossterm::event::Event::Resize(width, height) => {
                //     event_type = EventType::Resize;
                //     event_data = EventData::Resize(Vec2 {
                //         x: width,
                //         y: height,
                //     });
                // }
                _ => (),
            }
        }

        // if event_type != EventType::None {
        //     // TODO: Handle this
        //     // let mut world = SystemView {
        //     //     relations: &self.relations,
        //     //
        //     //     node_storage: &self.node_store,
        //     //     extension_store: &mut self.extension_store,
        //     //     resource_store: &mut self.resource_store,
        //     // };
        //     //
        //     // let mut ctx = SystemContext::new_event(&event_data, &mut world);
        //     // self.system_store
        //     //     .run_systems_for_type(crate::types::UpdateType::Event(event_type), &mut ctx);
        //     // self.commands.append(ctx.get_commands());
        // }
        Ok(())
    }
    fn update(&mut self) {}
    fn render(&mut self) {
        _ = self.renderer.render_flip(&mut self.buffer);
    }

    pub fn run(&mut self) {
        self.setup();
        _ = self.renderer.init();

        while self.is_running {
            self.event();
            self.update();
            self.render();
        }

        _ = self.renderer.quit();
    }
}
