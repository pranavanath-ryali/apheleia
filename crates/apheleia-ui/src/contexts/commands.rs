// TODO: Rewrite default commands

// use std::{cell::RefCell, rc::Rc};

// use apheleia_core::types::Vector2;

// use crate::{
//     contexts::ContextCommand,
//     rootnode::RootNodeData,
//     types::{DirtyRenderLevel, EventType, NodeId, UpdateTypeNode},
// };

use apheleia_core::types::Vec2;
use log::info;
use tree_ds::prelude::Node;

use crate::{
    contexts::traits::ContextCommand,
    extensions::traits::Extension,
    node::{data::NodeData, traits::NodeTrait},
    types::{DirtyRenderLevel, NodeId, System, UpdateType},
};

pub struct CreateNode {
    pub id: NodeId,
    pub class: Option<String>,
    pub parent_id: Option<NodeId>,
    pub parent_class: Option<String>,

    pub position: Vec2,
    pub size: Option<Vec2>,

    pub node: Box<dyn NodeTrait>,
}
pub struct AddExtensionToId(pub NodeId, pub Box<dyn Extension>);
pub struct HookSystemToId {
    pub id: NodeId,
    pub update_type: UpdateType,
    pub priority: isize,
    pub system: System,
}

pub struct SetSize(pub NodeId, pub Vec2);
pub struct SetPosition(pub NodeId, pub Vec2);

pub struct MarkRenderDirty(pub NodeId, pub DirtyRenderLevel);
pub struct MarkUpdateDirty(pub NodeId);

impl ContextCommand for CreateNode {
    fn execute(self: Box<Self>, world: &mut crate::world::WorldViewForCommands) {
        info!(
            "Creating node with class: {}",
            self.class.clone().unwrap_or("".to_string())
        );
        let node_data = NodeData::new(self.position, self.size);
        if let Some(parent_id) = self.parent_id {
            _ = world
                .relations
                .add_node(Node::new(self.id, None), Some(&parent_id));
        } else if let Some(parent_class) = self.parent_class {
            let parent_id = world
                .node_storage
                .get_id(&parent_class)
                .unwrap_or_else(|| panic!("No Node found with class: {}", parent_class));

            _ = world
                .relations
                .add_node(Node::new(self.id, None), Some(parent_id));
        } else {
            _ = world.relations.add_node(Node::new(self.id, None), Some(&0));
        }

        world
            .node_storage
            .add_node(self.id, self.class, self.node, node_data);
        world.dirty_tracker.add_setup(self.id);
    }
}
impl ContextCommand for AddExtensionToId {
    fn execute(self: Box<Self>, world: &mut crate::world::WorldViewForCommands) {
        _ = world.extension_store.add_extension_to_node(self.0, self.1);
    }
}
impl ContextCommand for HookSystemToId {
    fn execute(self: Box<Self>, world: &mut crate::world::WorldViewForCommands) {
        world
            .systems_store
            .add_system(self.id, self.update_type, self.priority, self.system);
    }
}

impl ContextCommand for SetSize {
    fn execute(self: Box<Self>, world: &mut crate::world::WorldViewForCommands) {
        world
            .node_storage
            .get_data_mut(self.0)
            .unwrap_or_else(|| panic!("Node not found with ID: {}", self.0))
            .set_size(self.1);
    }
}
impl ContextCommand for SetPosition {
    fn execute(self: Box<Self>, world: &mut crate::world::WorldViewForCommands) {
        world
            .node_storage
            .get_data_mut(self.0)
            .unwrap_or_else(|| panic!("Node not found with ID: {}", self.0))
            .set_position(self.1);
    }
}

impl ContextCommand for MarkRenderDirty {
    fn execute(self: Box<Self>, world: &mut crate::world::WorldViewForCommands) {
        info!("node id: {} marked render dirty", self.0);
        match self.1 {
            DirtyRenderLevel::SimpleDirty => world.dirty_tracker.add_render(self.0),
            DirtyRenderLevel::SubtreeDirty => {
                for id in world
                    .relations
                    .get_subtree(&self.0, None)
                    .unwrap()
                    .traverse(&self.0, tree_ds::prelude::TraversalStrategy::PreOrder)
                    .unwrap()
                    .iter()
                {
                    world.dirty_tracker.add_render(*id);
                }
            }
        }
    }
}
impl ContextCommand for MarkUpdateDirty {
    fn execute(self: Box<Self>, world: &mut crate::world::WorldViewForCommands) {
        world.dirty_tracker.add_update(self.0);
    }
}

// pub struct SetSize(pub Vec2);
// pub struct SetSizeForNode(pub String, pub Vec2);

// pub struct RegisterForUpdate;
// pub struct RegisterForEvent(pub EventType);

// pub struct MarkRenderDirty(pub NodeId, pub DirtyRenderLevel);
// pub struct MarkUpdateDirty(pub NodeId);

// impl ContextCommand for SetSize {
//     fn execute(self: Box<Self>, id: NodeId, rootnode_data: Rc<RefCell<RootNodeData>>) {
//         rootnode_data
//             .borrow_mut()
//             .node_storage
//             .get_data_mut(id)
//             .unwrap()
//             .set_size(self.0);
//     }
// }
// impl ContextCommand for SetSizeForNode {
//     fn execute(self: Box<Self>, _id: NodeId, rootnode_data: Rc<RefCell<RootNodeData>>) {
//         if let Some(id) = rootnode_data.borrow().node_storage.get_id(self.0.as_str()) {
//             rootnode_data
//                 .borrow_mut()
//                 .node_storage
//                 .get_data_mut(*id)
//                 .unwrap()
//                 .set_size(self.1);
//         }
//     }
// }
// impl ContextCommand for RegisterForUpdate {
//     fn execute(self: Box<Self>, id: NodeId, rootnode_data: Rc<RefCell<RootNodeData>>) {
//         rootnode_data
//             .borrow_mut()
//             .update_tracker
//             .add_node(id, UpdateTypeNode::ConstantUpdate);
//     }
// }
// impl ContextCommand for RegisterForEvent {
//     fn execute(self: Box<Self>, id: NodeId, rootnode_data: Rc<RefCell<RootNodeData>>) {
//         rootnode_data
//             .borrow_mut()
//             .update_tracker
//             .add_node(id, UpdateTypeNode::Event(self.0));
//     }
// }
// impl ContextCommand for MarkRenderDirty {
//     fn execute(self: Box<Self>, id: NodeId, rootnode_data: Rc<RefCell<RootNodeData>>) {
//         match self.1 {
//             DirtyRenderLevel::SimpleDirty => {
//                 rootnode_data.borrow_mut().dirty_tracker.add_render(id);
//             }
//             DirtyRenderLevel::SubtreeDirty => {
//                 for id in rootnode_data
//                     .borrow()
//                     .relations
//                     .get_subtree(&self.0, None)
//                     .unwrap()
//                     .traverse(&self.0, tree_ds::prelude::TraversalStrategy::PreOrder)
//                     .unwrap()
//                     .iter()
//                 {
//                     rootnode_data.borrow_mut().dirty_tracker.add_render(*id);
//                 }
//             }
//         }
//     }
// }
// impl ContextCommand for MarkUpdateDirty {
//     fn execute(self: Box<Self>, id: NodeId, rootnode_data: Rc<RefCell<RootNodeData>>) {
//         rootnode_data.borrow_mut().dirty_tracker.add_update(id);
//     }
// }
