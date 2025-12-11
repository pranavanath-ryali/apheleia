use crate::{FAKE_NODEID, NodeId};
use apheleia_core::{Color, buffer::NodeBuffer, style::Style};

pub trait NodeTrait {
    fn get_x(&self) -> u16;
    fn get_y(&self) -> u16;

    fn get_width(&self) -> u16;
    fn get_height(&self) -> u16;

    fn render(&self, buf: &mut NodeBuffer);
}

pub struct Node {
    pub id: NodeId,

    pub x: u16,
    pub y: u16,

    pub width: u16,
    pub height: u16,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            id: FAKE_NODEID,
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        }
    }
}
impl NodeTrait for Node {
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

    fn render(&self, buf: &mut NodeBuffer) {
        buf.write_line(0, 0, "A".repeat(self.width as usize).as_str(),  Some(Style::default()));
        buf.write_line(0, self.height - 1, "A".repeat(self.width as usize).as_str(),  Some(Style::default()));
    }
}
