use apheleia_core::{buffer::Buffer, types::Vec2};

use crate::{
    contexts::{
        commands::{MarkRenderDirty, MarkUpdateDirty, SetPosition, SetSize},
        traits::ContextCommand,
    },
    extensions::traits::Extension,
    resources::traits::Resource,
    types::{DirtyRenderLevel, EventData, NodeId},
    world::SystemView,
};

pub struct SystemContext<'a> {
    id: Option<NodeId>,
    event_data: Option<&'a EventData>,
    buffer: Option<&'a mut Buffer>,

    world: &'a mut SystemView<'a>,
    commands: Vec<Box<dyn ContextCommand>>,
}
impl<'a> SystemContext<'a> {
    pub fn set_id(&mut self, id: NodeId) {
        self.id = Some(id);
    }
    pub fn get_id(&self) -> NodeId {
        self.id.unwrap()
    }

    pub(crate) fn new(world: &'a mut SystemView<'a>) -> Self {
        Self {
            id: None,
            event_data: None,
            buffer: None,
            world,
            commands: vec![],
        }
    }

    pub(crate) fn new_event(event_data: &'a EventData, world: &'a mut SystemView<'a>) -> Self {
        Self {
            id: None,
            event_data: Some(event_data),
            buffer: None,
            world,
            commands: vec![],
        }
    }

    pub(crate) fn new_render(buffer: &'a mut Buffer, world: &'a mut SystemView<'a>) -> Self {
        Self {
            id: None,
            event_data: None,
            buffer: Some(buffer),
            world,
            commands: vec![],
        }
    }

    pub fn add_command(&mut self, command: Box<dyn ContextCommand>) {
        if self.buffer.is_some() {
            panic!("Command added during render");
        }
        self.commands.push(command);
    }

    pub(crate) fn get_commands(&mut self) -> &mut Vec<Box<dyn ContextCommand>> {
        &mut self.commands
    }

    pub fn get_event_data(&mut self) -> &EventData {
        self.event_data
            .expect("SystemContext.get_event_data() is used outside of event context.")
    }

    pub fn get_buffer(&mut self) -> &mut Buffer {
        self.buffer
            .as_mut()
            .expect("SystemContext.get_buffer() is used outside of render context.")
    }

    // Relations
    pub fn get_children_ids(&mut self) -> Vec<NodeId> {
        self.world
            .relations
            .get_node_by_id(&self.get_id())
            .unwrap()
            .get_children_ids()
            .unwrap_or_default()
    }

    // ExtensionStore Functions
    // pub fn add_extension<E: Extension>(&mut self, extension: E) {
    //     self.world
    //         .extension_store
    //         .add_extension(self.get_id(), Box::new(extension));
    // }
    pub fn get_extension<E: Extension>(&self) -> &E {
        self.world.extension_store.get_extension::<E>(self.get_id())
    }
    pub fn get_extension_mut<E: Extension>(&mut self) -> &mut E {
        self.world
            .extension_store
            .get_extension_mut::<E>(self.get_id())
    }

    // ResourceStore Functions
    pub fn get_resource<R: Resource>(&self) -> &R {
        self.world
            .resource_store
            .get_resource::<R>()
            .expect("No Resource Found")
    }
    pub fn get_resource_mut<R: Resource>(&mut self) -> &mut R {
        self.world
            .resource_store
            .get_resource_mut::<R>()
            .expect("No Resource Found")
    }

    // DirtyTracker Functions
    pub fn mark_render_dirty(&mut self, dirty_level: DirtyRenderLevel) {
        self.add_command(Box::new(MarkRenderDirty(self.get_id(), dirty_level)));
    }
    pub fn mark_update_dirty(&mut self) {
        self.add_command(Box::new(MarkUpdateDirty(self.get_id())));
    }

    pub fn mark_render_dirty_for_node(&mut self, class: &str, dirty_level: DirtyRenderLevel) {
        let id = self
            .world
            .node_storage
            .get_id(class)
            .unwrap_or_else(|| panic!("No Node found with class: {}", class));

        self.add_command(Box::new(MarkRenderDirty(*id, dirty_level)));
    }
    pub fn mark_update_dirty_for_node(&mut self, class: &str) {
        let id = self
            .world
            .node_storage
            .get_id(class)
            .unwrap_or_else(|| panic!("No Node found with class: {}", class));

        self.add_command(Box::new(MarkUpdateDirty(*id)));
    }

    // // NodeStore Functions
    // pub fn get_node<T: NodeTrait>(&self) -> &T {
    //     // return unsafe {
    //     //     &*(self
    //     //         .rootnode_data
    //     //         .borrow()
    //     //         .node_storage
    //     //         .get_node_as::<T>(self.get_id())
    //     //         .unwrap() as *const T)
    //     // };
    // }
    // pub fn get_node_mut<T: NodeTrait>(&mut self) -> &mut T {
    //     return unsafe {
    //         &mut *(self
    //             .rootnode_data
    //             .borrow_mut()
    //             .node_storage
    //             .get_node_mut_as::<T>(self.get_id())
    //             .unwrap() as *mut T)
    //     };
    // }

    pub fn get_position(&self) -> Vec2 {
        self.world
            .node_storage
            .get_data(self.get_id())
            .unwrap()
            .get_position()
    }
    pub fn set_position(&mut self, position: Vec2) {
        self.add_command(Box::new(SetPosition(self.get_id(), position)));
    }

    pub fn get_size(&self) -> Option<Vec2> {
        self.world
            .node_storage
            .get_data(self.get_id())
            .unwrap()
            .get_size()
    }
    pub fn set_size(&mut self, size: Vec2) {
        self.add_command(Box::new(SetSize(self.get_id(), size)));
    }
}
