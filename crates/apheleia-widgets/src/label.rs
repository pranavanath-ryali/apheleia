use apheleia_core::{buffer::NodeBuffer, style::Style, types::vector::Vector2};
use apheleia_ui::node::{
    data::{self, NodeData},
    node::NodeTrait,
};

pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Label {
            text: text.to_string(),
        }
    }
}

impl NodeTrait for Label {
    fn initial_setup(&mut self, data: &mut NodeData) {
        match data.size {
            None => {
                data.size = Some(Vector2(self.text.len() as u16, 1));
            },
            _ => {}
        }
    }

    fn update(&mut self) {}
    fn render(&self, buf: &mut NodeBuffer) {
        buf.write_line(0, 0, &self.text, Some(Style::default()));
    }
}
