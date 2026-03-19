use std::{cell::RefCell, error::Error, io::stdout, mem, rc::Rc, time::Duration};

use apheleia_core::{buffer::Buffer, renderer::Renderer, types::vector::Vector2};
use crossterm::{
    event::{KeyCode, KeyModifiers, poll, read},
    terminal::{self, enable_raw_mode},
};
use tree_ds::prelude::{Node, Tree};

use crate::{
    NodeId,
    builder::node::NodeBuilder,
    contexts::Context,
    rootnode::{
        data::RootNodeData, dirty_tracker::DirtyTracker, id_generator::NodeIdGenerator,
        node_storage::NodeStorage, update_tracker::UpdateTracker,
    },
    types::{EventData, EventType},
};

pub struct RootNode {
    fps: u16,
    width: u16,
    height: u16,
    running: bool,

    id_generator: Rc<RefCell<NodeIdGenerator>>,

    relations: Tree<NodeId, NodeId>,
    node_storage: Rc<RefCell<NodeStorage>>,
    dirty_tracker: Rc<RefCell<DirtyTracker>>,
    update_tracker: Rc<RefCell<UpdateTracker>>,

    buffer: RefCell<Buffer>,
    renderer: Renderer,
}
impl Default for RootNode {
    fn default() -> Self {
        let (width, height) = terminal::size().unwrap();

        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        _ = relations.add_node(Node::new(0, None), None);

        RootNode {
            fps: 15,
            running: false,
            width,
            height,

            id_generator: Rc::new(RefCell::new(NodeIdGenerator::default())),

            relations,
            node_storage: Rc::new(RefCell::new(NodeStorage::default())),
            dirty_tracker: Rc::new(RefCell::new(DirtyTracker::default())),
            update_tracker: Rc::new(RefCell::new(UpdateTracker::default())),

            buffer: RefCell::new(Buffer::new(width, height)),
            renderer: Renderer {
                width,
                height,
                stdout: stdout(),
            },
        }
    }
}
impl RootNode {
    fn calculate_global_position(&self, id: NodeId) -> Vector2 {
        let mut position = self.node_storage.borrow().get_data(id).unwrap().position;
        self.relations
            .get_ancestor_ids(&id)
            .unwrap()
            .iter()
            .filter(|id| **id != 0_usize)
            .for_each(|node_id| {
                let pos = self
                    .node_storage
                    .borrow()
                    .get_data(*node_id)
                    .unwrap()
                    .position;
                position.0 += pos.0;
                position.1 += pos.1;
            });
        position
    }

    fn initial_setup(&mut self) {
        for id in self
            .relations
            .traverse(&0, tree_ds::prelude::TraversalStrategy::PreOrder)
            .unwrap()
            .iter()
        {
            if *id == 0_usize {
                continue;
            }

            let mut ctx = Context::new(
                *id,
                self.node_storage
                    .clone()
                    .borrow()
                    .get_data(*id)
                    .unwrap()
                    .clone(),
                RootNodeData {
                    relations: &mut self.relations,
                    node_storage: self.node_storage.clone(),
                    dirty_tracker: self.dirty_tracker.clone(),
                    update_tracker: self.update_tracker.clone(),
                },
            );
            self.node_storage
                .borrow_mut()
                .get_node_mut(*id)
                .unwrap()
                .initial_setup(&mut ctx);
            ctx.run_commands();

            let global_position = self.calculate_global_position(*id);
            self.node_storage
                .borrow_mut()
                .get_data_mut(*id)
                .unwrap()
                .set_global_position(global_position);
        }
    }
    fn event(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: Implement event function
        let mut event_type: Option<EventType> = None;
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

                    event_type = Some(EventType::Keys);
                    event_data = EventData::Keys(key_event);
                }
                crossterm::event::Event::Mouse(_) => todo!(),
                crossterm::event::Event::Paste(_) => todo!(),
                crossterm::event::Event::Resize(_, _) => todo!(),
            }
        }

        if let Some(event_type) = event_type {
            if let Some(ids) = self
                .update_tracker
                .borrow()
                .iter(crate::types::UpdateTypeNode::Event(event_type))
            {
                for id in ids {
                    let mut ctx = Context::new_event(
                        *id,
                        self.node_storage
                            .clone()
                            .borrow()
                            .get_data(*id)
                            .unwrap()
                            .clone(),
                        mem::take(&mut event_data),
                        RootNodeData {
                            relations: &mut self.relations,
                            node_storage: self.node_storage.clone(),
                            dirty_tracker: self.dirty_tracker.clone(),
                            update_tracker: self.update_tracker.clone(),
                        },
                    );
                    self.node_storage
                        .borrow_mut()
                        .get_node_mut(*id)
                        .unwrap()
                        .event(&mut ctx);
                    ctx.run_commands();
                }
            }
        }
        Ok(())
    }
    fn update(&mut self) {
        // Update Nodes marked dirty
        for id in self.dirty_tracker.borrow().iter_update() {
            let mut ctx = Context::new(
                *id,
                self.node_storage
                    .clone()
                    .borrow()
                    .get_data(*id)
                    .unwrap()
                    .clone(),
                RootNodeData {
                    relations: &mut self.relations,
                    node_storage: self.node_storage.clone(),
                    dirty_tracker: self.dirty_tracker.clone(),
                    update_tracker: self.update_tracker.clone(),
                },
            );
            self.node_storage
                .borrow_mut()
                .get_node_mut(*id)
                .unwrap()
                .update(&mut ctx);
            ctx.run_commands();
        }
        self.dirty_tracker.borrow_mut().clear_update();

        // Update Nodes registered for constant update
        if let Some(ids) = self
            .update_tracker
            .borrow()
            .iter(crate::types::UpdateTypeNode::ConstantUpdate)
        {
            for id in ids {
                let mut ctx = Context::new(
                    *id,
                    self.node_storage
                        .clone()
                        .borrow()
                        .get_data(*id)
                        .unwrap()
                        .clone(),
                    RootNodeData {
                        relations: &mut self.relations,
                        node_storage: self.node_storage.clone(),
                        dirty_tracker: self.dirty_tracker.clone(),
                        update_tracker: self.update_tracker.clone(),
                    },
                );
                self.node_storage
                    .borrow_mut()
                    .get_node_mut(*id)
                    .unwrap()
                    .update(&mut ctx);
                ctx.run_commands();
            }
        }
    }
    fn render_node(&mut self, id: NodeId) {
        let size = self
            .node_storage
            .borrow()
            .get_data(id)
            .unwrap()
            .get_size()
            .clone();
        if let Some(size) = size {
            let position = self
                .node_storage
                .borrow_mut()
                .get_data(id)
                .unwrap()
                .get_global_position()
                .unwrap_or(Vector2(0, 0));

            let mut node_buffer = Buffer::new(size.0, size.1);
            let mut ctx = Context::new(
                id,
                self.node_storage
                    .clone()
                    .borrow()
                    .get_data(id)
                    .unwrap()
                    .clone(),
                RootNodeData {
                    relations: &mut self.relations,
                    node_storage: self.node_storage.clone(),
                    dirty_tracker: self.dirty_tracker.clone(),
                    update_tracker: self.update_tracker.clone(),
                },
            );
            self.node_storage
                .borrow_mut()
                .get_node_mut(id)
                .unwrap()
                .render(&mut node_buffer, &mut ctx);

            self.buffer
                .borrow_mut()
                .render_buffer(position.0, position.1, &mut node_buffer);
        }
    }
    fn render(&mut self, flip: bool) {
        if flip {
            self.renderer.clear(&mut self.buffer.borrow_mut());
            for id in self
                .relations
                .traverse(&0, tree_ds::prelude::TraversalStrategy::PreOrder)
                .unwrap()
                .iter()
            {
                if *id == 0_usize {
                    continue;
                }
                self.render_node(*id);
            }
        } else {
            let tracker = self.dirty_tracker.clone();
            for id in tracker.borrow().iter_render() {
                self.render_node(*id);
            }
        }
        self.renderer.render(&mut self.buffer.borrow_mut());
        self.dirty_tracker.borrow_mut().clear_render();
    }
    pub fn run(&mut self) {
        _ = enable_raw_mode();

        self.initial_setup();
        self.render(true);

        self.running = true;
        while self.running {
            _ = self.event();
            self.update();
            self.render(false);
        }
    }

    pub fn create_node<'a>(&'a mut self, class: &str) -> NodeBuilder<'a> {
        NodeBuilder::new(
            self.id_generator.borrow_mut().next(),
            class,
            self.id_generator.clone(),
            &mut self.relations,
            self.node_storage.clone(),
        )
    }
}
