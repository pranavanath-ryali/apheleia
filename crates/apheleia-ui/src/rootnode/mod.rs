use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

use crate::NodeId;
use crate::contexts::{EventUpdateCommands, EventUpdateContext, InitialCallContext, IntialCallCommands};
use crate::node::data::{DirtyRenderLevel, NodeData};
use crate::node::node::NodeTrait;
use crate::types::{EventData, EventType, UpdateTypeNode};
use apheleia_core::types::vector::Vector2;
use apheleia_core::{buffer::Buffer, renderer::Renderer, terminal};
use crossterm::event::KeyModifiers;
use crossterm::{
    event::{KeyCode, poll, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tree_ds::prelude::{Node, TraversalStrategy, Tree};

pub struct RootNode {
    running: bool,

    node_count: NodeId,

    width: u16,
    height: u16,

    relations: Tree<NodeId, NodeId>,

    id_nodes: HashMap<NodeId, Box<dyn NodeTrait>>,
    id_data: HashMap<NodeId, NodeData>,
    class_id: HashMap<String, NodeId>,

    // event_resize_nodes: Vec<NodeId>,
    // event_keys_nodes: Vec<NodeId>,
    //
    // update_type_nodes: Vec<NodeId>,
    id_update_type: HashMap<UpdateTypeNode, Vec<NodeId>>,

    dirty_ids: Vec<NodeId>,

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

            dirty_ids: vec![],

            // event_resize_nodes: vec![],
            // event_keys_nodes: vec![],
            // update_type_nodes: vec![],
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

        if parent_class == "" {
            self.relations
                .add_node(Node::new(id, None), Some(&(0 as usize)));
        } else {
            self.relations.add_node(
                Node::new(id, None),
                Some(
                    self.class_id
                        .get(&parent_class.to_string())
                        .expect("Given parent class doesn't exist"),
                ),
            );
        }
    }

    fn calculate_global_position_for_id(&mut self, id: NodeId) -> Vector2 {
        let mut position = Vector2(0, 0);
        self.relations
            .get_ancestor_ids(&id)
            .unwrap()
            .iter()
            .filter(|v| **v != 0_usize)
            .for_each(|id| {
                let pos = &self.id_data.get(id).unwrap().position;
                position.0 += pos.0;
                position.1 += pos.1;
            });

        let data = self.id_data.get_mut(&id).unwrap();
        data.set_global_position(position);

        position
    }

    pub fn initial_setup(&mut self) {
        for id in self
            .relations
            .traverse(&(0 as usize), TraversalStrategy::PreOrder)
            .unwrap()
            .iter()
        {
            if *id == 0_usize {
                continue;
            }

            let data = self.id_data.get_mut(id).unwrap();
            let node = self.id_nodes.get_mut(id).unwrap();

            let mut ctx = InitialCallContext::new(&data.position, &data.size);
            node.initial_setup(&mut ctx);

            for command in ctx.get_commands().iter() {
                match command {
                    IntialCallCommands::SetSize(size) => data.size = Some(*size),
                    IntialCallCommands::RegisterForUpdate => self
                        .id_update_type
                        .get_mut(&UpdateTypeNode::ConstantUpdate)
                        .unwrap()
                        .push(*id),
                    IntialCallCommands::RegisterForEvent(event_type) => {
                        self.id_update_type
                            .get_mut(&UpdateTypeNode::Event(*event_type))
                            .unwrap()
                            .push(*id);

                    }
                }
            }
        }

        for (id, _) in self.id_nodes.iter_mut() {
            if *id == 0_usize {
                continue;
            }

            let mut position = Vector2(0, 0);
            self.relations
                .get_ancestor_ids(&id)
                .unwrap()
                .iter()
                .filter(|v| **v != 0_usize)
                .for_each(|id| {
                    let pos = &self.id_data.get(id).unwrap().position;
                    position.0 += pos.0;
                    position.1 += pos.1;
                });

            let data = self.id_data.get_mut(&id).unwrap();
            data.set_global_position(position);
        }
    }

    fn render_flip(&mut self) {
        // TODO: Clear Buffer before all this stuff

        for id in self
            .relations
            .traverse(&(0 as usize), TraversalStrategy::PreOrder)
            .unwrap()
            .iter()
        {
            if *id == 0_usize {
                continue;
            }

            let data = self.id_data.get_mut(id).unwrap();
            if let Some(size) = data.size {
                let position = data
                    .global_positon
                    .unwrap_or_else(|| todo!("Implement calculate_global_position function"));

                let mut node_buffer = Buffer::new(size.0, size.1);
                self.id_nodes
                    .get_mut(id)
                    .unwrap()
                    .render(&mut node_buffer);
                self.buffer
                    .render_buffer(position.0, position.1, &mut node_buffer);
            }
        }

        self.renderer.flip(&mut self.buffer);
    }

    fn render(&mut self) {
        for id in self.dirty_ids.iter() {
            let mut data = self.id_data.get_mut(id).unwrap();
            if let Some(size) = data.get_size() {
                match data.dirty.render {
                    DirtyRenderLevel::SimpleDirty => {
                        let position = data.global_positon.unwrap();

                        for y in 0..size.1 {
                            self.buffer.write_line(position.0, position.1 + y, &" ".repeat(size.0 as usize), None);
                        }

                        let mut node_buffer = Buffer::new(size.0, size.1);
                        self.id_nodes.get_mut(id)
                            .unwrap().render(&mut node_buffer);
                        self.buffer.render_buffer(position.0, position.1, &mut node_buffer);
                    },
                    _ => {}
                }
            }
        }
        self.renderer.update(&mut self.buffer);
    }

    fn event(&mut self) -> Result<(), Box<dyn Error>> {
        fn handle_ctx(root: &mut RootNode, ctx: &EventUpdateContext) {
            for command in ctx.get_commands() {
                match command {
                    EventUpdateCommands::SetSize(vector2) => todo!(),
                    EventUpdateCommands::SetPosition(vector2) => todo!(),
                    EventUpdateCommands::MarkRenderDirty(dirty_render_level) => {
                        root.dirty_ids.push(ctx.id);
                        root.id_data.get_mut(&ctx.id).unwrap().dirty.render = *dirty_render_level;
                    },
                }
            }
        }

        // event driven updates
        if poll(Duration::from_nanos(1_000_000_000 / 15))? {
            match read()? {
                crossterm::event::Event::Key(event) => {
                    if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL
                    {
                        self.running = false;
                    }
                    
                    // TODO: Find a cleaner implementation for handling ctx commands
                    for id in self
                        .id_update_type
                        .get_mut(&UpdateTypeNode::Event(EventType::Keys))
                        .unwrap()
                        .iter()
                    {
                        let node = self.id_nodes.get_mut(id).unwrap();
                        let data = self.id_data.get_mut(id).unwrap();

                        let mut ctx = EventUpdateContext::new(
                            *id,
                            &data.position,
                            &data.size,
                            EventData::Keys(event),
                        );
                        node.event(&mut ctx);

                        // handle ctx commands
                        for command in ctx.get_commands() {
                            match command {
                                EventUpdateCommands::MarkRenderDirty(level) => {
                                    self.dirty_ids.push(*id);
                                    data.dirty.render = *level;
                                },
                                _ => ()
                            }
                        }
                    }
                }
                crossterm::event::Event::Resize(width, height) => {
                    for id in self
                        .id_update_type
                        .get_mut(&UpdateTypeNode::Event(EventType::Resize))
                        .unwrap()
                        .iter()
                    {
                        let node = self.id_nodes.get_mut(id).unwrap();
                        let data = self.id_data.get_mut(id).unwrap();

                        let mut ctx = EventUpdateContext::new(
                            *id,

                            &data.position,
                            &data.size,
                            EventData::Resize(Vector2(width, height)),
                        );
                        node.event(&mut ctx);

                        // handle ctx commands
                    }
                }
                crossterm::event::Event::FocusGained => {}
                crossterm::event::Event::FocusLost => {}
                crossterm::event::Event::Mouse(mouse_event) => {}
                crossterm::event::Event::Paste(_) => {}
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode();

        self.render_flip();

        self.running = true;
        while (self.running) {
            self.event();
            // self.update();
            self.render();
        }

        disable_raw_mode();
        Ok(())
    }
}
