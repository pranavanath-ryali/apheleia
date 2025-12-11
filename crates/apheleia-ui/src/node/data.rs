use crate::{NodeId, node::node::NodeTrait};

pub struct NodeData {
    pub id: NodeId,

    pub x: u16,
    pub y: u16,

    pub width: u16,
    pub height: u16,

    pub node: Box<dyn NodeTrait>
}

pub trait NodeDataTrait {
    fn get_x(&self) -> u16;
    fn get_y(&self) -> u16;

    fn set_x(&mut self, x: u16);
    fn set_y(&mut self, y: u16);

    fn get_width(&self) -> u16;
    fn get_height(&self) -> u16;

    fn set_width(&mut self, width: u16);
    fn set_height(&mut self, height: u16);

    fn get_node(&self) -> &dyn NodeTrait;
    fn get_node_mut(&mut self) -> &mut dyn NodeTrait;
}

impl NodeDataTrait for NodeData {
    fn get_x(&self) -> u16 {
        self.x
    }
    fn get_y(&self) -> u16 {
        self.y
    }

    fn set_x(&mut self, x: u16) {
        self.x = x;
    }
    fn set_y(&mut self, y: u16) {
        self.y = y;
    }

    fn get_width(&self) -> u16 {
        self.width
    }
    fn get_height(&self) -> u16 {
        self.height
    }

    fn set_width(&mut self, width: u16) {
        self.width = width;
    }
    fn set_height(&mut self, height: u16) {
        self.height = height;
    }

    fn get_node(&self) -> &dyn NodeTrait {
        &*self.node
    }
    fn get_node_mut(&mut self) -> &mut dyn NodeTrait {
        &mut *self.node
    }
}
