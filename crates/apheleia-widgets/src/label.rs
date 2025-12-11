use apheleia_core::{buffer::NodeBuffer, style::Style};
use apheleia_ui::node::node::NodeTrait;

pub struct Label {
    text: String
}

impl Label {
    pub fn new(text: &str) -> Self {
        Label { text: text.to_string() }
    }
}

impl NodeTrait for Label {
    fn update(&mut self) {    }

    fn render(&self, buf: &mut NodeBuffer) {
        buf.write_line(0, 0, &self.text, Some(Style::default()));
    }
}
