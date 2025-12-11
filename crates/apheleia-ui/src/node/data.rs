use apheleia_core::types::vector::Vector2;

use crate::node::node::NodeTrait;

pub struct NodeData {
    pub position: Option<Vector2>,
    pub size: Option<Vector2>,
}
impl Default for NodeData {
    fn default() -> Self {
        NodeData {
            position: None,
            size: None,
        }
    }
}

pub struct NodeWrapper {
    pub data: NodeData,
    pub node: Box<dyn NodeTrait>,
}

pub trait NodeWrapperTrait {
    fn get_position(&self) -> Option<Vector2>;
    fn set_position(&mut self, v: Vector2);

    fn get_size(&self) -> Option<Vector2>;
    fn set_size(&mut self, s: Vector2);

    fn get_node(&self) -> &dyn NodeTrait;
    fn get_node_mut(&mut self) -> &mut dyn NodeTrait;
}

impl NodeWrapperTrait for NodeWrapper {
    fn get_position(&self) -> Option<Vector2> {
        self.data.position
    }
    fn set_position(&mut self, v: Vector2) {
        self.data.position = Some(v)
    }

    fn get_size(&self) -> Option<Vector2> {
        self.data.size
    }
    fn set_size(&mut self, s: Vector2) {
        self.data.size = Some(s);
    }

    fn get_node(&self) -> &dyn NodeTrait {
        &*self.node
    }
    fn get_node_mut(&mut self) -> &mut dyn NodeTrait {
        &mut *self.node
    }
}
