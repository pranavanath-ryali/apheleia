use apheleia_types::{Extension, NodeId, Resource, node_data::NodeData, vec2::Vec2};

use crate::{
    extensions::store::ExtensionStore, node::store::NodeDataStore, resources::store::ResourceStore,
};

mod extensions;
mod node;
mod resources;
pub mod commands;

pub struct World {
    data_store: NodeDataStore,
    extension_store: ExtensionStore,
    resource_store: ResourceStore,
}
impl World {
    pub fn new() -> Self {
        Self {
            data_store: NodeDataStore::new(),
            extension_store: Default::default(),
            resource_store: Default::default(),
        }
    }

    // Functions related to Node creation and NodeData
    pub fn create_node(&mut self, data: NodeData) -> NodeId {
        self.data_store.create_node(&data)
    }

    pub fn get_position(&self, id: NodeId) -> &Vec2 {
        self.data_store.get_position(id)
    }
    pub fn set_position(&mut self, id: NodeId, position: Vec2) {
        self.data_store.set_position(id, position);
    }

    pub fn get_size(&self, id: NodeId) -> &Vec2 {
        self.data_store.get_size(id)
    }
    pub fn set_size(&mut self, id: NodeId, size: Vec2) {
        self.data_store.set_size(id, size);
    }

    pub fn get_global_position(&self, id: NodeId) -> &Option<Vec2> {
        self.data_store.get_global_position(id)
    }
    pub fn set_global_position(&mut self, id: NodeId, global_position: Vec2) {
        self.data_store.set_global_position(id, global_position);
    }

    pub fn get_global_size(&self, id: NodeId) -> &Option<Vec2> {
        self.data_store.get_global_size(id)
    }
    pub fn set_global_size_mut(&mut self, id: NodeId, global_size: Vec2) {
        self.data_store.set_global_size(id, global_size);
    }

    // Functions related to extensions
    pub fn add_extension_to_node<T: Extension>(&mut self, node_id: NodeId, extension: T) {
        self.extension_store.add_extension_to_node(node_id, extension);
    }

    pub fn get_extension<T: Extension>(&self, node_id: NodeId) -> Option<&T> {
        self.extension_store.get_extension(node_id)
    }
    pub fn get_extension_mut<T: Extension>(&mut self, node_id: NodeId) -> Option<&mut T> {
        self.extension_store.get_extension_mut(node_id)
    }

    // Functions related to resources
    pub fn add_resource<T: Resource>(&mut self, res: T) {
        self.resource_store.add_resource(Box::new(res));
    }

    pub fn get_resource<T: Resource>(&self) -> Option<&T> {
        self.resource_store.get_resource::<T>()
    }
    pub fn get_resource_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.resource_store.get_resource_mut::<T>()
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_nodedata_store() {
        let mut world = World::default();

        let node = world.create_node(NodeData {
            position: Vec2 { x: 10, y: 5 },
            size: Vec2::zero(),
        });
        assert_eq!(*world.get_position(node), Vec2 { x: 10, y: 5 });

        world.set_position(node, Vec2 { x: 1, y: 1 });
        assert_eq!(*world.get_position(node), Vec2 { x: 1, y: 1 });
    }

    struct TestResource {
        value: u16,
    }
    impl Resource for TestResource {}

    #[test]
    fn test_world_resource() {
        let mut world = World::default();

        world.add_resource(TestResource { value: 123 });
        assert_eq!(world.get_resource::<TestResource>().unwrap().value, 123);
        world.get_resource_mut::<TestResource>().unwrap().value = 64;
        assert_eq!(world.get_resource::<TestResource>().unwrap().value, 64);
    }

    struct TestExt {
        value: u32 
    }
    impl Extension for TestExt {}

    #[test]
    fn test_world_extensions() {
        let mut world = World::default();

        let node1 = world.create_node(NodeData { position: Vec2::zero(), size: Vec2::zero() });
        let node2 = world.create_node(NodeData { position: Vec2::zero(), size: Vec2::zero() });

        world.add_extension_to_node(node1, TestExt { value: 10 });
        world.add_extension_to_node(node2, TestExt { value: 55 });

        assert_eq!(world.get_extension::<TestExt>(node1).unwrap().value, 10);
        assert_eq!(world.get_extension::<TestExt>(node2).unwrap().value, 55);
    }
}
