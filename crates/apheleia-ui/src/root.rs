use std::{
    cell::RefCell,
    io::{self, stdout},
    mem::take,
    rc::Rc,
    time::Duration,
};

use apheleia_core::{buffer::Buffer, renderer::Renderer, types::Vec2};
use crossterm::{
    event::{KeyCode, KeyModifiers, poll, read},
    terminal::{self, enable_raw_mode},
};
use indexmap::IndexSet;
use tree_ds::prelude::{Node, Tree};

use crate::{
    builder::node::NodeBuilder,
    contexts::{node::NodeContext, system::SystemContext, traits::ContextCommand},
    dirty_tracker::DirtyTracker,
    extensions::store::ExtensionStore,
    id_generator::{IdGenerator, IdGeneratorTrait},
    node::store::NodeStore,
    resources::{store::ResourceStore, traits::Resource},
    systems::store::SystemStore,
    types::{EventData, EventType, NodeId},
    world::{SystemView, WorldViewForCommands},
};

pub struct Root {
    fps: u16,
    pub size: Vec2,
    running: bool,

    id_generator: Rc<RefCell<IdGenerator<NodeId>>>,
    relations: Tree<NodeId, NodeId>,

    node_store: NodeStore,
    extension_store: ExtensionStore,
    system_store: SystemStore,
    resource_store: ResourceStore,
    dirty_tracker: DirtyTracker,

    buffer: Buffer,
    renderer: Renderer,
    commands: Vec<Box<dyn ContextCommand>>,
}
impl Default for Root {
    fn default() -> Self {
        let (width, height) = terminal::size().unwrap();
        let size = Vec2 {
            x: width,
            y: height,
        };

        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        _ = relations.add_node(Node::new(0, None), None);

        Root {
            fps: 15,
            running: false,
            size,

            id_generator: Rc::new(RefCell::new(IdGenerator::<NodeId>::new(0))),

            relations,

            node_store: Default::default(),
            extension_store: Default::default(),
            system_store: Default::default(),
            resource_store: Default::default(),
            dirty_tracker: Default::default(),

            buffer: Buffer::new(size),
            renderer: Renderer {
                size,
                stdout: stdout(),
            },
            commands: vec![],
        }
    }
}
impl Root {
    pub fn get_size(&self) -> Vec2 {
        self.size
    }

    // TODO: Add a test to see if these two following functions calculate the correct output
    // TODO: Make this its own function eventually
    fn calculate_global_position(&self, id: NodeId) -> Vec2 {
        let mut position = self.node_store.get_data(id).unwrap().position;
        self.relations
            .get_ancestor_ids(&id)
            .unwrap()
            .iter()
            .filter(|id| **id != 0_usize)
            .for_each(|node_id| {
                let pos = self.node_store.get_data(*node_id).unwrap().position;

                position.x += pos.x;
                position.y += pos.y;
            });

        position
    }
    fn calculate_global_size(&self, id: NodeId) -> Option<Vec2> {
        let size = self
            .node_store
            .get_data(id)
            .unwrap()
            .get_size()
            .unwrap_or(Vec2::zero());
        let mut global_size = size;
        let parent_id = self.relations.get_ancestor_ids(&id).unwrap()[0];

        if parent_id != 0 {
            let parent_global_size = self
                .node_store
                .get_data(parent_id)
                .unwrap()
                .get_global_size()
                .unwrap_or(Vec2::zero());

            if parent_global_size.x == 0 && parent_global_size.y == 0 {
                return None;
            }

            let position = self.node_store.get_data(id).unwrap().get_position();
            if parent_global_size.x != 0 && position.x + size.x > parent_global_size.x - 1 {
                if position.x >= parent_global_size.x {
                    global_size.x = 0;
                } else {
                    global_size.x = parent_global_size.x - position.x;
                }
            }
            if parent_global_size.y != 0 && position.y + size.y > parent_global_size.y - 1 {
                if position.y >= parent_global_size.y {
                    global_size.y = 0;
                } else {
                    global_size.y = parent_global_size.y - position.y;
                }
            }
        }

        if global_size.x == 0 || global_size.y == 0 {
            return None;
        }
        Some(global_size)
    }

    fn run_commands(&mut self) {
        let commands = take(&mut self.commands);
        for command in commands {
            command.execute(&mut WorldViewForCommands {
                relations: &mut self.relations,

                node_storage: &mut self.node_store,
                systems_store: &mut self.system_store,
                extension_store: &mut self.extension_store,
                dirty_tracker: &mut self.dirty_tracker,
                resource_store: &mut self.resource_store,
            });
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
            let data = self.node_store.get_data(id).unwrap();
            let mut ctx = NodeContext::new(id, self.id_generator.clone(), data.position, data.size);
            self.node_store
                .get_node_mut(id)
                .unwrap()
                .initial_setup(&mut ctx);

            self.commands.append(ctx.get_commands());

            let global_position = self.calculate_global_position(id);
            let global_size = self.calculate_global_size(id);
            let data = self.node_store.get_data_mut(id).unwrap();

            data.set_global_position(global_position);
            data.set_global_size(global_size);
        }
        self.run_commands();

        // For nodes that were created from other nodes automatically
        // TODO: Make this run automatically after every commands flush
        // if !self.dirty_tracker.is_setup_empty() {
        //     let ids: Vec<NodeId> = self.dirty_tracker.iter_setup().copied().collect();
        //     for id in ids {
        //         let data = self.node_store.get_data(id).unwrap();
        //         let mut ctx =
        //             NodeContext::new(id, self.id_generator.clone(), data.position, data.size);
        //         self.node_store
        //             .get_node_mut(id)
        //             .unwrap()
        //             .initial_setup(&mut ctx);

        //         let commands = take(ctx.get_commands());

        //         let global_position = self.calculate_global_position(id);
        //         let global_size = self.calculate_global_size(id);
        //         let data = self.node_store.get_data_mut(id).unwrap();

        //         data.set_global_position(global_position);
        //         data.set_global_size(global_size);
        //     }
        // }

        Ok(())
    }

    fn dirty_setup(&mut self) {
        if !self.dirty_tracker.is_setup_empty() {
            let ids: IndexSet<NodeId> = take(&mut self.dirty_tracker.dirty_setups);
            for id in ids {
                let data = self.node_store.get_data(id).unwrap();
                let mut ctx =
                    NodeContext::new(id, self.id_generator.clone(), data.position, data.size);
                self.node_store
                    .get_node_mut(id)
                    .unwrap()
                    .initial_setup(&mut ctx);

                self.commands.append(ctx.get_commands());

                let global_position = self.calculate_global_position(id);
                let global_size = self.calculate_global_size(id);
                let data = self.node_store.get_data_mut(id).unwrap();

                data.set_global_position(global_position);
                data.set_global_size(global_size);
            }
        }
        self.run_commands();
    }

    fn event(&mut self) -> io::Result<()> {
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
            let mut world = SystemView {
                relations: &self.relations,

                node_storage: &self.node_store,
                extension_store: &mut self.extension_store,
                resource_store: &mut self.resource_store,
            };

            let mut ctx = SystemContext::new_event(&event_data, &mut world);
            self.system_store
                .run_systems_for_type(crate::types::UpdateType::Event(event_type), &mut ctx);
            self.commands.append(ctx.get_commands());
        }
        Ok(())
    }

    fn update(&mut self) {
        // Update Nodes marked dirty
        let ids: IndexSet<NodeId> = take(&mut self.dirty_tracker.dirty_updates);
        for id in ids {
            let mut world = SystemView {
                relations: &self.relations,

                node_storage: &self.node_store,
                extension_store: &mut self.extension_store,
                resource_store: &mut self.resource_store,
            };

            let mut ctx = SystemContext::new(&mut world);
            self.system_store.run_systems_for_node_with_type(
                crate::types::UpdateType::ConstantUpdate,
                id,
                &mut ctx,
            );
            self.commands.append(ctx.get_commands());
        }

        // Update Nodes registered for constant update
        // TODO: Add a check to see if there are any systems registered for constant update
        let mut world = SystemView {
            relations: &self.relations,

            node_storage: &self.node_store,
            extension_store: &mut self.extension_store,
            resource_store: &mut self.resource_store,
        };
        let mut ctx = SystemContext::new(&mut world);
        self.system_store
            .run_systems_for_type(crate::types::UpdateType::ConstantUpdate, &mut ctx);
        self.commands.append(ctx.get_commands());
    }

    fn render_node(&mut self, id: NodeId) {
        let size = self.node_store.get_data(id).unwrap().get_global_size();
        if let Some(global_size) = size {
            let size = self.node_store.get_data(id).unwrap().get_size().unwrap();
            let mut node_buffer = Buffer::new(size);
            let mut world = SystemView {
                relations: &self.relations,

                node_storage: &self.node_store,
                extension_store: &mut self.extension_store,
                resource_store: &mut self.resource_store,
            };
            let mut ctx = SystemContext::new_render(&mut node_buffer, &mut world);
            self.system_store.run_systems_for_node_with_type(
                crate::types::UpdateType::Render,
                id,
                &mut ctx,
            );

            node_buffer.resize(global_size);

            let position = self
                .node_store
                .get_data(id)
                .unwrap()
                .get_global_position()
                .unwrap();
            self.buffer.render_buffer(position, &mut node_buffer);
        }
    }

    fn render_flip(&mut self) -> io::Result<()> {
        let ids: Vec<NodeId> = self
            .relations
            .traverse(&0, tree_ds::prelude::TraversalStrategy::PreOrder)
            .unwrap()
            .iter()
            .filter(|v| **v != 0_usize)
            .copied()
            .collect();

        for id in ids {
            self.render_node(id);
        }

        self.renderer.render_flip(&mut self.buffer)?;
        self.dirty_tracker.clear_render();

        Ok(())
    }

    fn render(&mut self) -> io::Result<()> {
        let ids: IndexSet<NodeId> = take(&mut self.dirty_tracker.dirty_renders);
        for id in ids {
            self.render_node(id);
        }

        self.renderer.render(&mut self.buffer)?;
        self.dirty_tracker.clear_render();

        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        _ = enable_raw_mode();

        self.dirty_setup();
        self.render_flip()?;

        self.running = true;
        while self.running {
            _ = self.event();
            self.update();
            self.render()?;

            self.run_commands();
        }

        self.renderer.quit()?;

        Ok(())
    }

    // Functions for Developers
    pub fn create_node(&mut self, f: impl FnOnce(NodeBuilder) -> NodeBuilder) {
        let id = self.id_generator.borrow_mut().next();
        let mut builder = f(NodeBuilder::new(id, self.id_generator.clone()));
        self.commands.append(&mut builder.build());
    }

    pub fn add_resource<T: Resource>(&mut self, res: T) {
        self.resource_store.add_resource(Box::new(res));
    }
}
