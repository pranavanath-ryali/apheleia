use std::{cell::RefCell, error::Error, io::stdout, rc::Rc, time::Duration};

use apheleia_core::{buffer::Buffer, renderer::Renderer, types::vector::Vector2};
use crossterm::{
    event::{KeyCode, KeyModifiers, poll, read},
    terminal::{self, enable_raw_mode},
};
use log::info;
use tree_ds::prelude::{Node, Tree};

use crate::{
    builder::node::NodeBuilder,
    contexts::{node::NodeContext, system::SystemContext},
    extensions::traits::Extension,
    id_generator::{IdGenerator, IdGeneratorTrait},
    node::traits::NodeTrait,
    types::{EventData, EventType, NodeId},
    world::World,
};

pub struct Root {
    fps: u16,
    width: u16,
    height: u16,
    running: bool,

    nodeid_gen: Rc<RefCell<IdGenerator<NodeId>>>,
    data: Rc<RefCell<World>>,

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
            data: Rc::new(RefCell::new(World::default())),

            buffer: Buffer::new(width, height),
            renderer: Renderer {
                width,
                height,
                stdout: stdout(),
            },
        }
    }
}
impl Root {
    fn calculate_global_position(&self, id: NodeId) -> Vector2 {
        let relations = &self.data.borrow().relations;
        let node_storage = &self.data.borrow().node_storage;

        let mut position = node_storage.get_data(id).unwrap().position;
        relations
            .get_ancestor_ids(&id)
            .unwrap()
            .iter()
            .filter(|id| **id != 0_usize)
            .for_each(|node_id| {
                let pos = node_storage.get_data(*node_id).unwrap().position;

                position.0 += pos.0;
                position.1 += pos.1;
            });

        position
    }

    fn initial_setup(&mut self) {
        info!("RootNode inital_setup started");

        let ids: Vec<NodeId> = self
            .data
            .borrow()
            .relations
            .traverse(&0, tree_ds::prelude::TraversalStrategy::PreOrder)
            .unwrap()
            .iter()
            .filter(|v| **v != 0_usize)
            .copied()
            .collect();

        for id in ids {
            let node = {
                self.data
                    .borrow_mut()
                    .node_storage
                    .get_node_mut(id)
                    .unwrap() as *mut Box<dyn NodeTrait>
            };
            let mut ctx = NodeContext::new(id, self.data.clone());
            unsafe {
                (*node).initial_setup(&mut ctx);
            }
            ctx.run_commands();

            let global_position = self.calculate_global_position(id);
            self.data
                .borrow_mut()
                .node_storage
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

            let mut ctx = SystemContext::new_event(&event_data, self.data.clone());
            self.data
                .borrow_mut()
                .system_store
                .run_systems_for_type(crate::types::UpdateType::Event(event_type), &mut ctx);
            ctx.run_commands();
        }
        info!("RootNode event ended");
        Ok(())
    }
    fn update(&mut self) {
        // Update Nodes marked dirty
        let ids: Vec<NodeId> = self
            .data
            .borrow()
            .dirty_tracker
            .iter_update()
            .copied()
            .collect();
        for id in ids {
            let mut ctx = SystemContext::new(self.data.clone());
            self.data
                .borrow_mut()
                .system_store
                .run_systems_for_node_with_type(
                    crate::types::UpdateType::ConstantUpdate,
                    id,
                    &mut ctx,
                );
            // self.data
            //     .borrow_mut()
            //     .node_storage
            //     .get_node_mut(id)
            //     .unwrap()
            //     .update(&mut ctx);
            ctx.run_commands();
        }
        self.data.borrow_mut().dirty_tracker.clear_update();

        // Update Nodes registered for constant update
        let mut ctx = SystemContext::new(self.data.clone());
        self.data
            .borrow_mut()
            .system_store
            .run_systems_for_type(crate::types::UpdateType::ConstantUpdate, &mut ctx);
        ctx.run_commands();
        // if self
        //     .data
        //     .borrow()
        //     .update_tracker
        //     .is_empty(crate::types::UpdateTypeNode::ConstantUpdate)
        // {
        //     let ids: Vec<NodeId> = self
        //         .data
        //         .borrow()
        //         .update_tracker
        //         .iter(crate::types::UpdateTypeNode::ConstantUpdate)
        //         .unwrap()
        //         .copied()
        //         .collect();
        //     for id in ids {
        //         let mut ctx = Context::new(id, self.data.clone());
        //         self.data
        //             .borrow_mut()
        //             .node_storage
        //             .get_node_mut(id)
        //             .unwrap()
        //             .update(&mut ctx);
        //         ctx.run_commands();
        //     }
        // }
    }
    fn render_node(&mut self, id: NodeId) {
        let size = self
            .data
            .borrow()
            .node_storage
            .get_data(id)
            .unwrap()
            .get_size();
        if let Some(size) = size {
            info!("RootNode Render node begins: {}", id);
            let position = self
                .data
                .borrow_mut()
                .node_storage
                .get_data(id)
                .unwrap()
                .get_global_position()
                .unwrap_or(Vector2(0, 0));

            let mut node_buffer = Buffer::new(size.0, size.1);
            let mut ctx = SystemContext::new_render(&mut node_buffer, self.data.clone());
            // self.data
            //     .borrow_mut()
            //     .node_storage
            //     .get_node_mut(id)
            //     .unwrap()
            //     .render(&mut node_buffer, &mut ctx);

            self.data
                .borrow_mut()
                .system_store
                .run_systems_for_node_with_type(crate::types::UpdateType::Render, id, &mut ctx);

            self.buffer
                .render_buffer(position.0, position.1, &mut node_buffer);
            info!("RootNode Render node ends: {}", id);
        }
    }
    fn render_flip(&mut self) {
        self.renderer.clear(&mut self.buffer);

        let ids: Vec<NodeId> = self
            .data
            .borrow()
            .relations
            .traverse(&0, tree_ds::prelude::TraversalStrategy::PreOrder)
            .unwrap()
            .to_vec();

        for id in ids {
            if id == 0_usize {
                continue;
            }
            self.render_node(id);
        }

        self.renderer.render(&mut self.buffer);
        self.data.borrow_mut().dirty_tracker.clear_render();
    }

    fn render(&mut self) {
        let ids: Vec<NodeId> = self
            .data
            .borrow()
            .dirty_tracker
            .iter_render()
            .copied()
            .collect();

        for id in ids {
            info!("Apparently id {} is marked dirty", id);
            self.render_node(id);
        }

        self.renderer.render(&mut self.buffer);
        self.data.borrow_mut().dirty_tracker.clear_render();
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
    pub fn create_node(&mut self, class: &str) -> NodeBuilder {
        NodeBuilder::new(
            self.nodeid_gen.borrow_mut().next(),
            class,
            self.data.clone(),
        )
    }

    pub fn bind_extension_to_classes<T: Extension>(
        &mut self,
        classes: Vec<&str>,
        extension: Box<T>,
    ) {
        let ext_id = self.data.borrow_mut().extension_store.get_id();
        self.data
            .borrow_mut()
            .extension_store
            .add_extension(ext_id, extension);

        for class in classes {
            let id = self.data.borrow_mut().node_storage.get_id(class).cloned();
            if let Some(id) = id {
                _ = self
                    .data
                    .borrow_mut()
                    .extension_store
                    .bind_extension::<T>(id, ext_id);
            }
        }
    }
}
