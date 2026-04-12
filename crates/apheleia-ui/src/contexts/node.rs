use std::{cell::RefCell, mem, rc::Rc};

use apheleia_core::types::Vector2;

use crate::{
    contexts::traits::ContextCommand,
    resources::traits::Resource,
    types::{NodeId, System, UpdateType},
    world::WorldViewForNode,
};

pub struct NodeContext<'a> {
    id: NodeId,
    rootnode_data: &'a mut WorldViewForNode<'a>,

    commands: Vec<Box<dyn ContextCommand>>,
}
impl<'a> NodeContext<'a> {
    pub fn new(id: NodeId, rootnode_data: &'a mut WorldViewForNode<'a>) -> NodeContext {
        Self {
            id,
            rootnode_data,
            commands: vec![],
        }
    }

    pub fn get_id(&self) -> NodeId {
        self.id
    }

    // pub fn add_system(&mut self, update_type: UpdateType, priority: isize, system: System) {
    //     let id = self.get_id();
    //     self.rootnode_data
    //         .borrow_mut()
    //         .system_store
    //         .add_system(id, update_type, priority, system);
    // }

    // pub fn add_resource<T: Resource>(&mut self, res: T) {
    //     self.rootnode_data
    //         .borrow_mut()
    //         .resource_store
    //         .add_resource(Box::new(res));
    // }

    // pub fn get_resource<T: Resource>(&self) -> Option<&T> {
    //     if let Some(res) = self
    //         .rootnode_data
    //         .borrow()
    //         .resource_store
    //         .get_resource::<T>()
    //     {
    //         // TODO: Hopefully find a better way
    //         let reference: &T = unsafe { &*(res as *const T) };
    //         return Some(reference);
    //     }
    //     None
    // }
    // pub fn get_resource_mut<T: Resource>(&mut self) -> Option<&mut T> {
    //     if let Some(res) = self
    //         .rootnode_data
    //         .borrow_mut()
    //         .resource_store
    //         .get_resource_mut::<T>()
    //     {
    //         // TODO: Hopefully find a better way
    //         let reference: &mut T = unsafe { &mut *(res as *mut T) };
    //         return Some(reference);
    //     }
    //     None
    // }

    pub(crate) fn run_commands(&mut self) {
        let commands = mem::take(&mut self.commands);
        for command in commands {
            // command.execute(self.rootnode_data.clone());
        }
    }

    // pub fn get_position(&self) -> Vector2 {
    //     self.rootnode_data
    //         .borrow()
    //         .node_storage
    //         .get_data(self.id)
    //         .unwrap()
    //         .get_position()
    // }
    // pub fn set_position(&mut self, position: Vector2) {
    //     self.rootnode_data
    //         .borrow_mut()
    //         .node_storage
    //         .get_data_mut(self.id)
    //         .unwrap()
    //         .set_position(position);
    // }

    // pub fn get_size(&self) -> Option<Vector2> {
    //     self.rootnode_data
    //         .borrow()
    //         .node_storage
    //         .get_data(self.id)
    //         .unwrap()
    //         .get_size()
    // }
    // pub fn set_size(&mut self, size: Vector2) {
    //     self.rootnode_data
    //         .borrow_mut()
    //         .node_storage
    //         .get_data_mut(self.id)
    //         .unwrap()
    //         .set_size(size);
    // }
}
