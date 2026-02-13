use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::mem;
use std::time::Duration;

use crate::NodeId;
use crate::contexts::{Context, EventData};
use crate::node::data::{DirtyRenderLevel, NodeData};
use crate::node::node::NodeTrait;
use crate::types::{EventType, UpdateTypeNode};
use apheleia_core::types::vector::Vector2;
use apheleia_core::{buffer::Buffer, renderer::Renderer, terminal};
use crossterm::event::{Event, KeyModifiers};
use crossterm::{
    event::{poll, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use indexmap::{IndexSet, indexset};
use tree_ds::prelude::{Node, TraversalStrategy, Tree};

pub struct RootNodeData<'a> {
    pub relations: &'a mut Tree<NodeId, NodeId>,

    pub id_data: &'a mut HashMap<NodeId, NodeData>,
    pub class_id: &'a mut HashMap<String, NodeId>,

    pub id_update_type: &'a mut HashMap<UpdateTypeNode, Vec<NodeId>>,

    pub id_dirty_update: &'a mut IndexSet<NodeId>,
    pub id_dirty_render: &'a mut IndexSet<NodeId>,
}

pub enum Dirty {
    Render_SimpleDirty,
    Render_SubtreeDirty,

    Update_SimpleDirty,
    Update_SubtreeDirty,
}

pub struct RootNode {
    running: bool,

    node_count: NodeId,

    width: u16,
    height: u16,

    relations: Tree<NodeId, NodeId>,

    id_nodes: HashMap<NodeId, Box<dyn NodeTrait>>,
    id_data: HashMap<NodeId, NodeData>,
    class_id: HashMap<String, NodeId>,

    id_update_type: HashMap<UpdateTypeNode, Vec<NodeId>>,

    id_dirty_update: IndexSet<NodeId>,
    id_dirty_render: IndexSet<NodeId>,

    buffer: Buffer,
    renderer: Renderer,
}
impl Default for RootNode {
    fn default() -> Self {
        let size = terminal::size().unwrap();

        let mut relations: Tree<NodeId, NodeId> = Tree::new(None);
        relations.add_node(Node::new(0, None), None);

        let mut id_update_type: HashMap<UpdateTypeNode, Vec<NodeId>> = HashMap::new();
        id_update_type.insert(UpdateTypeNode::ConstantUpdate, vec![]);
        id_update_type.insert(UpdateTypeNode::Event(EventType::Keys), vec![]);
        id_update_type.insert(UpdateTypeNode::Event(EventType::Resize), vec![]);

        Self {
            running: false,

            node_count: 0,

            width: size.0,
            height: size.1,

            relations,
            id_update_type,

            id_nodes: HashMap::new(),
            id_data: HashMap::new(),
            class_id: HashMap::new(),

            id_dirty_update: indexset! {},
            id_dirty_render: indexset! {},

            buffer: Buffer::new(size.0, size.1),
            renderer: Renderer::default(),
        }
    }
}

impl RootNode {
    fn get_id(&mut self) -> NodeId {
        self.node_count += 1;
        self.node_count
    }

    pub fn add_node(
        &mut self,
        class: &str,
        parent_class: &str,
        node: Box<dyn NodeTrait>,
        data: NodeData,
    ) {
        let id = self.get_id();
        self.class_id.insert(class.to_string(), id);
        self.id_nodes.insert(id, node);
        self.id_data.insert(id, data);

        if parent_class.is_empty() {
            self.relations.add_node(Node::new(id, None), Some(&0));
        } else {
            self.relations.add_node(
                Node::new(id, None),
                Some(
                    self.class_id
                        .get(parent_class)
                        .expect("Given parent class doesn't exist"),
                ),
            );
        }
    }

    fn calculate_global_position_for_id(&mut self, id: NodeId) {
        if let Ok(children) = self.relations.get_subtree(&id, None) {
            children
                .traverse(&id, TraversalStrategy::PreOrder)
                .unwrap()
                .iter()
                .for_each(|child_id| {
                    let mut position = self.id_data.get(child_id).unwrap().position;
                    self.relations
                        .get_ancestor_ids(child_id)
                        .unwrap()
                        .iter()
                        .filter(|v| **v != 0_usize)
                        .for_each(|id| {
                            let pos = &self.id_data.get(id).unwrap().position;
                            position.0 += pos.0;
                            position.1 += pos.1;
                        });

                    self.id_data
                        .get_mut(child_id)
                        .unwrap()
                        .set_global_position(position);
                });
        }
    }

    pub fn initial_setup(&mut self) {
        for id in self
            .relations
            .traverse(&0, TraversalStrategy::PreOrder)
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
                    id_data: &mut self.id_data,
                    class_id: &mut self.class_id,
                    id_update_type: &mut self.id_update_type,
                    id_dirty_update: &mut self.id_dirty_update,
                    id_dirty_render: &mut self.id_dirty_render,
                },
            );
            self.id_nodes.get_mut(id).unwrap().initial_setup(&mut ctx);
            ctx.run_commands();
        }

        for (id, _) in self.id_nodes.iter_mut() {
            if *id == 0_usize {
                continue;
            }

            let mut position = self.id_data.get(id).unwrap().position;
            self.relations
                .get_ancestor_ids(id)
                .unwrap()
                .iter()
                .filter(|id| **id != 0)
                .for_each(|id| {
                    let pos = self.id_data.get(id).unwrap().get_position();
                    position.0 += pos.0;
                    position.1 += pos.1;
                });

            let data = self.id_data.get_mut(id).unwrap();
            data.set_global_position(position);
        }
    }

    fn render_flip(&mut self) {
        self.renderer.clear(&mut self.buffer);

        for id in self
            .relations
            .traverse(&0, TraversalStrategy::PreOrder)
            .unwrap()
            .iter()
        {
            if *id == 0_usize {
                continue;
            }

            self.render_node(id, false);
        }

        self.id_dirty_render.clear();
        self.renderer.update(&mut self.buffer);
    }

    fn render_node(&mut self, id: &NodeId, fill_empty: bool) {
        if let Some(size) = self.id_data.get(id).unwrap().get_size() {
            let position = self.id_data.get(id).unwrap().global_positon.unwrap();

            for y in 0..size.1 {
                self.buffer.write_line(
                    position.0,
                    position.1 + y,
                    &" ".repeat(size.0 as usize),
                    None,
                );
            }

            if fill_empty {
                for y in 0..size.1 {
                    self.buffer.write_line(
                        position.0,
                        position.1 + y,
                        &" ".repeat(size.0 as usize),
                        None,
                    );
                }
            }

            let mut node_buffer = Buffer::new(size.0, size.1);

            let ctx = Context::new(
                *id,
                RootNodeData {
                    relations: &mut self.relations,
                    id_data: &mut self.id_data,
                    class_id: &mut self.class_id,
                    id_update_type: &mut self.id_update_type,
                    id_dirty_update: &mut self.id_dirty_update,
                    id_dirty_render: &mut self.id_dirty_render,
                },
            );
            self.id_nodes
                .get(id)
                .unwrap()
                .render(&mut node_buffer, &ctx);

            self.buffer
                .render_buffer(position.0, position.1, &mut node_buffer);
            self.id_dirty_render.shift_remove(id);
        }
    }

    fn render(&mut self) {
        let ids = self.id_dirty_render.clone(); // I Don't really like the look of
        // this clone function.
        // TODO: Maybe find a better way
        // for this?
        for id in ids.iter() {
            self.render_node(id, false);
        }

        self.id_dirty_render.clear();
        self.renderer.update(&mut self.buffer);
    }

    fn event_node(&mut self, id: NodeId, event_data: EventData) {
        let mut ctx = Context::new_event(
            id,
            event_data,
            RootNodeData {
                relations: &mut self.relations,
                id_data: &mut self.id_data,
                class_id: &mut self.class_id,
                id_update_type: &mut self.id_update_type,
                id_dirty_update: &mut self.id_dirty_update,
                id_dirty_render: &mut self.id_dirty_render,
            },
        );
        self.id_nodes.get_mut(&id).unwrap().event(&mut ctx);
        ctx.run_commands();
    }

    fn event(&mut self) -> Result<(), Box<dyn Error>> {
        if poll(Duration::from_nanos(1_000_000_000 / 15))? {
            match read()? {
                Event::FocusGained => todo!(),
                Event::FocusLost => todo!(),
                Event::Key(key_event) => {
                    if key_event.modifiers.contains(KeyModifiers::CONTROL)
                        && key_event.code.is_char('c')
                    {
                        self.running = false;
                    }

                    for id in self
                        .id_update_type
                        .get(&UpdateTypeNode::Event(EventType::Keys))
                        .unwrap()
                        .to_owned()
                    {
                        self.event_node(id, EventData::Keys(key_event));
                    }
                }
                Event::Mouse(mouse_event) => todo!(),
                Event::Paste(_) => todo!(),
                Event::Resize(width, height) => {
                    for id in self
                        .id_update_type
                        .get(&UpdateTypeNode::Event(EventType::Resize))
                        .unwrap()
                        .to_owned()
                    {
                        self.width = width;
                        self.height = height;
                        self.event_node(id, EventData::Resize(Vector2(width, height)));
                    }
                }
            }
        }

        Ok(())
    }

    fn update(&mut self) {
        for id in self.id_dirty_update.to_owned() {
            let mut ctx = Context::new(
                id,
                RootNodeData {
                    relations: &mut self.relations,
                    id_data: &mut self.id_data,
                    class_id: &mut self.class_id,
                    id_update_type: &mut self.id_update_type,
                    id_dirty_update: &mut self.id_dirty_update,
                    id_dirty_render: &mut self.id_dirty_render,
                },
            );
            self.id_nodes.get_mut(&id).unwrap().update(&mut ctx);
            ctx.run_commands();
        }
        self.id_dirty_update.clear();
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode();

        self.render_flip();

        self.running = true;
        while self.running {
            self.event();
            self.update();
            self.render();
        }

        disable_raw_mode();
        Ok(())
    }
}
