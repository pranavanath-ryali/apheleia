use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

use crate::NodeId;
use crate::contexts::{
    EventUpdateCommands, EventUpdateContext, InitialCallContext, IntialCallCommands, RenderContext, UpdateContext
};
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

    fn calculate_global_position_for_id(&mut self, id: NodeId) {
        if let Ok(children) = self.relations.get_subtree(&id, None) {
            children
                .traverse(&id, TraversalStrategy::PreOrder)
                .unwrap()
                .iter()
                .for_each(|child_id| {
                    let mut position = self.id_data.get(&child_id).unwrap().position;
                    self.relations
                        .get_ancestor_ids(&child_id)
                        .unwrap()
                        .iter()
                        .filter(|v| **v != 0_usize)
                        .for_each(|id| {
                            let pos = &self.id_data.get(id).unwrap().position;
                            position.0 += pos.0;
                            position.1 += pos.1;
                        });

                    let data = self
                        .id_data
                        .get_mut(&child_id)
                        .unwrap()
                        .set_global_position(position);
                });
        }
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

            let mut position = self.id_data.get(id).unwrap().position;
            self.relations
                .get_ancestor_ids(&id)
                .unwrap()
                .iter()
                .filter(|id| **id != 0)
                .for_each(|id| {
                    let pos = self.id_data.get(id).unwrap().get_position();
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

            self.render_node(id, false);

            // let data = self.id_data.get_mut(id).unwrap();
            // if let Some(size) = data.size {
            //     let position = data
            //         .global_positon
            //         .unwrap_or_else(|| todo!("Implement calculate_global_position function"));
            //
            //     let mut node_buffer = Buffer::new(size.0, size.1);
            //     self.id_nodes.get_mut(id).unwrap().render(&mut node_buffer);
            //     self.buffer
            //         .render_buffer(position.0, position.1, &mut node_buffer);
            // }
        }

        self.renderer.flip(&mut self.buffer);
    }

    fn render_node(&mut self, id: &NodeId, fill_empty: bool) {
        let mut data = self.id_data.get_mut(id).unwrap();
        if let Some(size) = data.get_size() {
            let position = data.global_positon.unwrap();

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
        
            let mut ctx = RenderContext::new(*id, position, *size);
            self.id_nodes.get(id).unwrap().render(&mut node_buffer, &mut ctx);

            self.buffer
                .render_buffer(position.0, position.1, &mut node_buffer);
            data.dirty.render = DirtyRenderLevel::None;
        }
    }

    fn render(&mut self) {
        let ids = self.dirty_ids.clone();
        self.dirty_ids.clear();
        for id in ids.iter() {
            match self.id_data.get(id).unwrap().dirty.render {
                DirtyRenderLevel::SimpleDirty => {
                    self.render_node(id, false);
                }
                DirtyRenderLevel::SubtreeDirty => {
                    if let Ok(children) = self.relations.get_subtree(id, None) {
                        for child_id in children
                            .traverse(id, TraversalStrategy::PreOrder)
                            .unwrap()
                            .iter()
                        {
                            self.render_node(id, true);

                            // let data = self.id_data.get_mut(child_id).unwrap();
                            // if let Some(size) = data.get_size() {
                            //     let position = data.get_global_position().unwrap();
                            //
                            //     for y in 0..size.1 {
                            //         self.buffer.write_line(
                            //             position.0,
                            //             position.1 + y,
                            //             &" ".repeat(size.0 as usize),
                            //             None,
                            //         );
                            //     }
                            //
                            //     let mut node_buffer = Buffer::new(size.0, size.1);
                            //     self.id_nodes
                            //         .get_mut(child_id)
                            //         .unwrap()
                            //         .render(&mut node_buffer);
                            //     self.buffer
                            //         .render_buffer(position.0, position.1, &mut node_buffer);
                            // }
                        }
                        self.id_data.get_mut(id).unwrap().dirty.render = DirtyRenderLevel::None;
                    }
                }
                _ => {}
            }
        }
        self.renderer.update(&mut self.buffer);
    }

    fn handle_event_ctx_commands(&mut self, id: NodeId, commands: &Vec<EventUpdateCommands>) {
        for command in commands {
            match command {
                EventUpdateCommands::MarkRenderDirty(level) => {
                    let data = self.id_data.get_mut(&id).unwrap();
                    self.dirty_ids.push(id);
                    data.dirty.render = *level;
                }
                EventUpdateCommands::SetSize(size) => {
                    let data = self.id_data.get_mut(&id).unwrap();
                    data.set_size(*size);

                    self.dirty_ids.push(id);
                    data.dirty.render = DirtyRenderLevel::SimpleDirty;
                }
                EventUpdateCommands::SetPosition(position) => {
                    self.id_data.get_mut(&id).unwrap().set_position(*position);

                    self.dirty_ids.push(id);
                    self.id_data.get_mut(&id).unwrap().dirty.render =
                        DirtyRenderLevel::SubtreeDirty;
                    self.calculate_global_position_for_id(id);
                }
            }
        }
    }

    fn event(&mut self) -> Result<(), Box<dyn Error>> {
        let mut commands: HashMap<NodeId, Vec<EventUpdateCommands>> = HashMap::new();

        if poll(Duration::from_nanos(1_000_000_000 / 15))? {
            match read()? {
                crossterm::event::Event::Key(event) => {
                    if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL
                    {
                        self.running = false;
                    }

                    for id in self
                        .id_update_type
                        .get(&UpdateTypeNode::Event(EventType::Keys))
                        .unwrap()
                        .iter()
                    {
                        let position = *self.id_data.get(id).unwrap().get_position();
                        let size = *self.id_data.get(id).unwrap().get_size();
                        let mut ctx =
                            EventUpdateContext::new(*id, &position, &size, EventData::Keys(event));
                        self.id_nodes.get_mut(id).unwrap().event(&mut ctx);
                        commands.insert(*id, ctx.commands);
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

                        commands.insert(*id, ctx.commands);
                    }
                }
                crossterm::event::Event::FocusGained => {}
                crossterm::event::Event::FocusLost => {}
                crossterm::event::Event::Mouse(mouse_event) => {}
                crossterm::event::Event::Paste(_) => {}
            }
        }

        for (id, commands) in commands {
            self.handle_event_ctx_commands(id, &commands);
        }

        Ok(())
    }

    fn update(&mut self) {
        for id in self
            .id_update_type
            .get(&UpdateTypeNode::ConstantUpdate)
            .unwrap()
            .iter()
        {
            let data = self.id_data.get(id).unwrap();
            let mut ctx = UpdateContext::new(*id, *data.get_position(), data.get_size());

            self.id_nodes.get_mut(id).unwrap().update(&mut ctx);
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode();

        self.render_flip();

        self.running = true;
        while (self.running) {
            self.event();
            self.buffer.write_line(
                100,
                0,
                &format!(
                    "{} {}",
                    1,
                    self.id_data
                        .get(&1)
                        .unwrap()
                        .get_global_position()
                        .unwrap()
                        .0
                ),
                None,
            );
            self.update();
            self.render();
        }

        disable_raw_mode();
        Ok(())
    }
}
