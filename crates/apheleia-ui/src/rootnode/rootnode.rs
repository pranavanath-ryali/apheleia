use std::{cell::RefCell, error::Error, io::stdout, rc::Rc, time::Duration};

use apheleia_core::{buffer::Buffer, renderer::Renderer, types::vector::Vector2};
use crossterm::{
    event::{poll, read},
    terminal::{self, enable_raw_mode},
};
use tree_ds::prelude::{Node, Tree};

use crate::{
    NodeId,
    builder::node::NodeBuilder,
    contexts::Context,
    rootnode::{data::RootNodeData, dirty_tracker::DirtyTracker, node_storage::NodeStorage},
    utils::calculate_global_position,
};

pub struct RootNodeDup {
    width: u16,
    height: u16,
    running: bool,

    node_count: NodeId,

    relations: Tree<NodeId, NodeId>,
    node_storage: Rc<RefCell<NodeStorage>>,
    dirty_tracker: Rc<RefCell<DirtyTracker>>,

    buffer: Buffer,
    renderer: Renderer,
}
impl Default for RootNodeDup {
    fn default() -> Self {
        let (width, height) = terminal::size().unwrap();

        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        _ = relations.add_node(Node::new(0, None), None);

        RootNodeDup {
            node_count: 0,
            running: false,
            width,
            height,

            relations,
            node_storage: Rc::new(RefCell::new(NodeStorage::default())),
            dirty_tracker: Rc::new(RefCell::new(DirtyTracker::default())),

            buffer: Buffer::new(width, height),
            renderer: Renderer {
                width,
                height,
                stdout: stdout(),
            },
        }
    }
}
impl RootNodeDup {
    fn get_id(&mut self) -> NodeId {
        self.node_count += 1;
        self.node_count
    }

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
                RootNodeData {
                    relations: &mut self.relations,
                    node_storage: self.node_storage.clone(),
                    dirty_tracker: self.dirty_tracker.clone(),
                },
            );
            self.node_storage
                .borrow()
                .get_node_mut(*id)
                .unwrap()
                .initial_setup(&mut ctx);
            ctx.run_commands();
        }

        for (id, data) in self.node_storage.borrow_mut().iter_id_data_mut() {
            if id == &0_usize {
                continue;
            }

            let global_position = self.calculate_global_position(*id);
            data.set_global_position(global_position);
        }
    }
    fn event(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: Implement event function
        if poll(Duration::from_nanos(1_000_000_000 / 15))? {
            match read()? {
                crossterm::event::Event::FocusGained => todo!(),
                crossterm::event::Event::FocusLost => todo!(),
                crossterm::event::Event::Key(key_event) => todo!(),
                crossterm::event::Event::Mouse(mouse_event) => todo!(),
                crossterm::event::Event::Paste(_) => todo!(),
                crossterm::event::Event::Resize(_, _) => todo!(),
            }
        }
        Ok(())
    }
    fn update(&mut self) {}
    fn render_flip(&mut self) {}
    fn render(&mut self) {}
    pub fn run(&mut self) {
        _ = enable_raw_mode();

        self.initial_setup();
        self.render_flip();

        self.running = true;
        while self.running {
            self.event();
            self.update();
            self.render();
        }
    }

    pub fn create_node(&mut self, class: &str) -> NodeBuilder {
        NodeBuilder::new(
            self.get_id(),
            class,
            &mut self.relations,
            self.node_storage.clone(),
        )
    }
}
