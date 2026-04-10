use std::{cell::RefCell, mem, rc::Rc};

use apheleia_core::{buffer::Buffer, types::Vector2};

use crate::{
    contexts::traits::ContextCommand,
    extensions::traits::Extension,
    node::traits::NodeTrait,
    resources::traits::Resource,
    types::{EventData, NodeId},
    world::World,
};

pub struct SystemContext<'a> {
    id: Option<NodeId>,
    event_data: Option<&'a EventData>,
    buffer: Option<&'a mut Buffer>,

    rootnode_data: Rc<RefCell<World>>,
    commands: Vec<Box<dyn ContextCommand>>,
}
impl<'a> SystemContext<'a> {
    pub fn set_id(&mut self, id: NodeId) {
        self.id = Some(id);
    }
    pub fn get_id(&self) -> NodeId {
        self.id.unwrap()
    }

    pub fn new(rootnode_data: Rc<RefCell<World>>) -> Self {
        Self {
            id: None,
            event_data: None,
            buffer: None,
            rootnode_data,
            commands: vec![],
        }
    }

    pub fn new_event(event_data: &'a EventData, rootnode_data: Rc<RefCell<World>>) -> Self {
        Self {
            id: None,
            event_data: Some(event_data),
            buffer: None,
            rootnode_data,
            commands: vec![],
        }
    }

    pub fn new_render(buffer: &'a mut Buffer, rootnode_data: Rc<RefCell<World>>) -> Self {
        Self {
            id: None,
            event_data: None,
            buffer: Some(buffer),
            rootnode_data,
            commands: vec![],
        }
    }

    pub(crate) fn run_commands(&mut self) {
        let commands = mem::take(&mut self.commands);
        for command in commands {
            command.execute(self.get_id(), self.rootnode_data.clone());
        }
    }

    // ExtensionStore Functions
    pub fn get_extension<E: Extension>(&self) -> &E {
        return unsafe {
            &*(self
                .rootnode_data
                .borrow()
                .extension_store
                .get_extension::<E>(self.get_id()) as *const E)
        };
    }
    pub fn get_extension_mut<E: Extension>(&mut self) -> &mut E {
        return unsafe {
            &mut *(self
                .rootnode_data
                .borrow_mut()
                .extension_store
                .get_extension_mut::<E>(self.get_id()) as *mut E)
        };
    }

    // ResourceStore Functions
    pub fn get_resource<R: Resource>(&self) -> &R {
        return unsafe {
            &*(self
                .rootnode_data
                .borrow()
                .resource_store
                .get_resource::<R>()
                .unwrap() as *const R)
        };
    }
    pub fn get_resource_mut<R: Resource>(&mut self) -> &mut R {
        return unsafe {
            &mut *(self
                .rootnode_data
                .borrow_mut()
                .resource_store
                .get_resource_mut::<R>()
                .unwrap() as *mut R)
        };
    }

    // DirtyTracker Functions
    pub fn mark_render_dirty(&mut self) {
        self.rootnode_data
            .borrow_mut()
            .dirty_tracker
            .add_render(self.get_id());
    }
    pub fn mark_update_dirty(&mut self) {
        self.rootnode_data
            .borrow_mut()
            .dirty_tracker
            .add_update(self.get_id());
    }

    pub fn mark_render_dirty_for_node(&mut self, class: &str) {
        if let Some(id) = self.rootnode_data.borrow().node_storage.get_id(class) {
            self.rootnode_data
                .borrow_mut()
                .dirty_tracker
                .add_render(*id);
        }
    }
    pub fn mark_update_dirty_for_node(&mut self, class: &str) {
        if let Some(id) = self.rootnode_data.borrow().node_storage.get_id(class) {
            self.rootnode_data
                .borrow_mut()
                .dirty_tracker
                .add_update(*id);
        }
    }

    // NodeStore Functions
    pub fn get_node<T: NodeTrait>(&self) -> &T {
        return unsafe {
            &*(self
                .rootnode_data
                .borrow()
                .node_storage
                .get_node_as::<T>(self.get_id())
                .unwrap() as *const T)
        };
    }
    pub fn get_node_mut<T: NodeTrait>(&mut self) -> &mut T {
        return unsafe {
            &mut *(self
                .rootnode_data
                .borrow_mut()
                .node_storage
                .get_node_mut_as::<T>(self.get_id())
                .unwrap() as *mut T)
        };
    }

    pub fn get_position(&self) -> Vector2 {
        self.rootnode_data
            .borrow()
            .node_storage
            .get_data(self.get_id())
            .unwrap()
            .get_position()
    }
    pub fn set_position(&mut self, position: Vector2) {
        self.rootnode_data
            .borrow_mut()
            .node_storage
            .get_data_mut(self.get_id())
            .unwrap()
            .set_position(position);
    }

    pub fn get_size(&self) -> Option<Vector2> {
        self.rootnode_data
            .borrow()
            .node_storage
            .get_data(self.get_id())
            .unwrap()
            .get_size()
    }
    pub fn set_size(&mut self, size: Vector2) {
        self.rootnode_data
            .borrow_mut()
            .node_storage
            .get_data_mut(self.get_id())
            .unwrap()
            .set_size(size);
    }

    pub fn get_buffer(&mut self) -> &mut Buffer {
        self.buffer
            .as_mut()
            .expect("SystemContext.get_buffer() is used outside of render context.")
    }
}
