use apheleia_core::types::vector::Vector2;
use apheleia_ui::{FAKE_NODEID, node::data::{NodeData, NodeWrapper}, rootnode::RootNode};
use apheleia_widgets::label::Label;

fn main() {
    let mut root = RootNode::new();
    
    root.add_node(NodeWrapper {
        data: NodeData {
            position: Some(Vector2(0, 0)),
            ..Default::default()
        },
        node: Box::new(Label::new("Hello World"))
    });
    root.add_node(NodeWrapper {
        data: NodeData {
            position: Some(Vector2(20, 10)),
            ..Default::default()
        },
        node: Box::new(Label::new("LABELS ARE WORKING"))
    });
    root.add_node(NodeWrapper {
        data: NodeData {
            position: Some(Vector2(5, 7)),
            ..Default::default()
        },
        node: Box::new(Label::new("WEEEEEEEEEEE"))
    });

    root.initial_setup();
    root.start();
}
