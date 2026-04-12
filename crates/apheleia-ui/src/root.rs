use std::{cell::RefCell, error::Error, io::stdout, mem::take, rc::Rc, time::Duration};

use apheleia_core::{buffer::Buffer, renderer::Renderer, types::Vector2};
use crossterm::{
    event::{KeyCode, KeyModifiers, poll, read},
    terminal::{self, enable_raw_mode},
};
use log::info;
use tree_ds::prelude::{Node, Tree};

use crate::{
    builder::node::NodeBuilder,
    contexts::{node::NodeContext, system::SystemContext, traits::ContextCommand},
    dirty_tracker::DirtyTracker,
    extensions::{store::ExtensionStore, traits::Extension},
    id_generator::{IdGenerator, IdGeneratorTrait},
    node::{store::NodeStore, traits::NodeTrait},
    resources::{store::ResourceStore, traits::Resource},
    systems::store::SystemStore,
    types::{EventData, EventType, NodeId},
    world::{BuilderView, SystemView, WorldViewForCommands, WorldViewForNode},
};

pub struct Root {
    fps: u16,
    pub width: u16,
    pub height: u16,
    running: bool,

    nodeid_gen: Rc<RefCell<IdGenerator<NodeId>>>,
    // data: Rc<RefCell<World>>,
    relations: Tree<NodeId, NodeId>,

    node_storage: NodeStore,
    extension_store: ExtensionStore,
    dirty_tracker: DirtyTracker,
    system_store: SystemStore,
    resource_store: ResourceStore,

    buffer: Buffer,
    renderer: Renderer,
}
impl Default for Root {
    fn default() -> Self {
        let (width, height) = terminal::size().unwrap();

        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        _ = relations.add_node(Node::new(0, None), None);

        info!("RootNode initialization started");

        Root {
            fps: 15,
            running: false,
            width,
            height,

            nodeid_gen: Rc::new(RefCell::new(IdGenerator::<NodeId>::new(0))),

            relations,

            node_storage: NodeStore::default(),
            extension_store: ExtensionStore::default(),
            dirty_tracker: DirtyTracker::default(),
            system_store: SystemStore::default(),
            resource_store: ResourceStore::default(),

            buffer: Buffer::new(width, height),
            renderer: Renderer {
                size: (width, height),
                stdout: stdout(),
            },
        }
    }
}
impl Root {
    fn calculate_global_position(&self, id: NodeId) -> Vector2 {
        let mut position = self.node_storage.get_data(id).unwrap().position;
        self.relations
            .get_ancestor_ids(&id)
            .unwrap()
            .iter()
            .filter(|id| **id != 0_usize)
            .for_each(|node_id| {
                let pos = self.node_storage.get_data(*node_id).unwrap().position;

                position.0 += pos.0;
                position.1 += pos.1;
            });

        position
    }

    fn run_commands(&mut self, commands: Vec<Box<dyn ContextCommand>>) {
        for command in commands {
            command.execute(&mut WorldViewForCommands {
                node_storage: &mut self.node_storage,
                systems_store: &mut self.system_store,
                extension_store: &mut self.extension_store,
                dirty_tracker: &mut self.dirty_tracker,
                resource_store: &mut self.resource_store,
            });
        }
    }

    fn initial_setup(&mut self) {
        info!("RootNode inital_setup started");

        let ids: Vec<NodeId> = self
            .relations
            .traverse(&0, tree_ds::prelude::TraversalStrategy::PreOrder)
            .unwrap()
            .iter()
            .filter(|v| **v != 0_usize)
            .copied()
            .collect();

        for id in ids {
            let node = self.node_storage.get_node_mut(id).unwrap();
            let mut world = WorldViewForNode {
                extension_store: &mut self.extension_store,
                system_store: &mut self.system_store,
                resource_store: &mut self.resource_store,
            };
            let mut ctx = NodeContext::new(id, &mut world);
            node.initial_setup(&mut ctx);

            let commands = take(ctx.get_commands());
            self.run_commands(commands);

            let global_position = self.calculate_global_position(id);
            self.node_storage
                .get_data_mut(id)
                .unwrap()
                .set_global_position(global_position);
        }

        info!("RootNode intial_setup ended");
    }

    fn event(&mut self) -> Result<(), Box<dyn Error>> {
        info!("RootNode event started");
        // TODO: Implement event function
        let mut event_type: EventType = EventType::None;
        let mut event_data: EventData = EventData::None;
        if poll(Duration::from_nanos(1_000_000_000 / self.fps as u64))? {
            match read()? {
                crossterm::event::Event::FocusGained => todo!(),
                crossterm::event::Event::FocusLost => todo!(),
                crossterm::event::Event::Key(key_event) => {
                    if key_event.modifiers == KeyModifiers::CONTROL
                        && key_event.code == KeyCode::Char('c')
                    {
                        self.running = false;
                    }

                    event_type = EventType::Keys;
                    event_data = EventData::Keys(key_event);
                }
                crossterm::event::Event::Mouse(_) => todo!(),
                crossterm::event::Event::Paste(_) => todo!(),
                crossterm::event::Event::Resize(width, height) => {
                    event_type = EventType::Resize;
                    event_data = EventData::Resize(Vector2(width, height));
                }
            }
        }
        info!("RootNode Event triggered: {:?}", event_type);

        if event_type != EventType::None {
            info!("RootNode Event data: {:?}", event_data);

            let mut world = SystemView {
                relations: &mut self.relations,

                node_storage: &mut self.node_storage,
                dirty_tracker: &mut self.dirty_tracker,
                extension_store: &mut self.extension_store,
                resource_store: &mut self.resource_store,
            };

            let mut ctx = SystemContext::new_event(&event_data, &mut world);
            self.system_store
                .run_systems_for_type(crate::types::UpdateType::Event(event_type), &mut ctx);
            let commands = take(ctx.get_commands());
            self.run_commands(commands);
        }
        info!("RootNode event ended");
        Ok(())
    }

    fn update(&mut self) {
        // Update Nodes marked dirty
        let ids: Vec<NodeId> = self.dirty_tracker.iter_update().copied().collect();
        for id in ids {
            let mut world = SystemView {
                relations: &mut self.relations,

                dirty_tracker: &mut self.dirty_tracker,
                extension_store: &mut self.extension_store,
                node_storage: &mut self.node_storage,
                resource_store: &mut self.resource_store,
            };

            let mut ctx = SystemContext::new(&mut world);
            self.system_store.run_systems_for_node_with_type(
                crate::types::UpdateType::ConstantUpdate,
                id,
                &mut ctx,
            );
            let commands = take(ctx.get_commands());
            self.run_commands(commands);
        }
        self.dirty_tracker.clear_update();

        // Update Nodes registered for constant update
        // TODO: Add a check to see if there are any systems registered for constant update
        let mut world = SystemView {
            relations: &mut self.relations,

            dirty_tracker: &mut self.dirty_tracker,
            extension_store: &mut self.extension_store,
            node_storage: &mut self.node_storage,
            resource_store: &mut self.resource_store,
        };
        let mut ctx = SystemContext::new(&mut world);
        self.system_store
            .run_systems_for_type(crate::types::UpdateType::ConstantUpdate, &mut ctx);
        let commands = take(ctx.get_commands());
        self.run_commands(commands);
    }
    fn render_node(&mut self, id: NodeId) {
        let size = self.node_storage.get_data(id).unwrap().get_size();
        if let Some(size) = size {
            info!("RootNode Render node begins: {}", id);

            let mut node_buffer = Buffer::new(size.0, size.1);
            let mut world = SystemView {
                relations: &mut self.relations,

                dirty_tracker: &mut self.dirty_tracker,
                extension_store: &mut self.extension_store,
                node_storage: &mut self.node_storage,
                resource_store: &mut self.resource_store,
            };
            let mut ctx = SystemContext::new_render(&mut node_buffer, &mut world);
            self.system_store.run_systems_for_node_with_type(
                crate::types::UpdateType::Render,
                id,
                &mut ctx,
            );

            let position = self
                .node_storage
                .get_data(id)
                .unwrap()
                .get_global_position()
                .unwrap_or(Vector2(0, 0));
            self.buffer
                .render_buffer(position.0, position.1, &mut node_buffer);

            info!("RootNode Render node ends: {}", id);
        }
    }
    fn render_flip(&mut self) {
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

        self.renderer.render_flip(&mut self.buffer);
        self.dirty_tracker.clear_render();
    }

    fn render(&mut self) {
        let ids: Vec<NodeId> = self.dirty_tracker.iter_render().copied().collect();

        for id in ids {
            info!("Apparently id {} is marked dirty", id);
            self.render_node(id);
        }

        self.renderer.render(&mut self.buffer);
        self.dirty_tracker.clear_render();
    }

    pub fn run(&mut self) {
        _ = enable_raw_mode();

        self.initial_setup();
        self.render_flip();

        self.running = true;
        while self.running {
            _ = self.event();
            self.update();
            self.render();
        }
    }

    // Functions for Developers
    // pub fn create_node<'a>(&mut self, class: &str) -> &'a mut NodeBuilder {
    //     let mut world = WorldViewForBuilder {
    //         relations: &mut self.relations,
    //         node_storage: &mut self.node_storage,
    //         extension_store: &mut self.extension_store,
    //         system_store: &mut self.system_store,
    //         resource_store: &mut self.resource_store,
    //     };
    //     let mut builder = NodeBuilder::new(self.nodeid_gen.borrow_mut().next(), class);

    //     &mut builder
    // }

    // pub fn add_resource<T: Resource>(&mut self, res: T) {
    //     self.resource_store.add_resource(Box::new(res));
    // }

    // pub fn bind_extension_to_classes<T: Extension>(
    //     &mut self,
    //     classes: Vec<&str>,
    //     extension: Box<T>,
    // ) {
    //     let ext_id = self.extension_store.get_id();
    //     self.extension_store.add_extension(ext_id, extension);

    //     for class in classes {
    //         let id = self.node_storage.get_id(class).cloned();
    //         if let Some(id) = id {
    //             _ = self.extension_store.bind_extension::<T>(id, ext_id);
    //         }
    //     }
    // }
}
