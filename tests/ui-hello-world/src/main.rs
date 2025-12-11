use apheleia_core::{buffer::NodeBuffer, style::Style, types::vector::Vector2};
use apheleia_ui::{node::{data::{NodeData, NodeWrapper}, node::NodeTrait}, rootnode::RootNode};

struct BasicNode;
impl NodeTrait for BasicNode {
    fn initial_setup(&mut self, data: &mut apheleia_ui::node::data::NodeData) {
        data.size = Some(Vector2(10, 1))
    }

    fn update(&mut self) {
    }

    fn render(&self, buf: &mut NodeBuffer) {
        buf.write_line(0, 0, "A", Some(Style::default()));
    }
}
fn main() {
    let mut root = RootNode::new();

    let node_data = NodeWrapper {
        data: NodeData {
            position: Some(Vector2(0, 0)),
            ..Default::default()
        },
        node: Box::new(BasicNode {})
    };

    root.add_node(node_data);
    
    root.initial_setup();
    root.start();
}
