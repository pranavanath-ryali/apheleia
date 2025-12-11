use apheleia_core::{buffer::NodeBuffer, style::Style};
use apheleia_ui::{node::{data::NodeData, node::NodeTrait}, rootnode::RootNode};

struct BasicNode;
impl NodeTrait for BasicNode {
    fn update(&mut self) {
    }

    fn render(&self, buf: &mut NodeBuffer) {
        buf.write_line(0, 0, "A", Some(Style::default()));
    }
}
fn main() {
    let mut root = RootNode::new();

    let node_data = NodeData {
        id: 0,

        x: 10,
        y: 3,

        width: 10,
        height: 10,

        node: Box::new(BasicNode {})
    };

    root.add_node(node_data);

    root.start();
}
