use crate::{FAKE_NODEID, NodeId};
use apheleia_core::{Color, buffer::NodeBuffer, renderer, style::Style};

pub trait NodeDataTrait {
    fn get_x(&self) -> u16;
    fn get_y(&self) -> u16;

    fn get_width(&self) -> u16;
    fn get_height(&self) -> u16;

    fn get_node(&self) -> &dyn NodeTrait;
    fn get_node_mut(&mut self) -> &mut dyn NodeTrait;
}
pub struct NodeData {
    pub id: NodeId,

    pub x: u16,
    pub y: u16,

    pub width: u16,
    pub height: u16,

    pub node: Box<dyn NodeTrait>,
}
impl NodeDataTrait for NodeData {
    fn get_x(&self) -> u16 {
        self.x
    }
    fn get_y(&self) -> u16 {
        self.y
    }

    fn get_width(&self) -> u16 {
        self.width
    }
    fn get_height(&self) -> u16 {
        self.height
    }

    fn get_node(&self) -> &dyn NodeTrait {
        &*self.node
    }
    fn get_node_mut(&mut self) -> &mut dyn NodeTrait {
        &mut *self.node
    }
}

pub trait NodeTrait {
    fn update(&self);
    fn render(&self, buf: &mut NodeBuffer);
}

pub struct BasicNode;
impl NodeTrait for BasicNode {
    fn update(&self) {
        
    }
    fn render(&self, buf: &mut NodeBuffer) {
        buf.write_line(0, 0, "A", Some(Style::default()));
    }
}
