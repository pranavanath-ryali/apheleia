use apheleia_core::types::vector::Vector2;
use apheleia_ui::{node::data::{NodeData, NodeWrapper}, rootnode::{self, RootNode}};
use apheleia_widgets::{block::Block, label::Label};

fn main() {
    let mut root = RootNode::default();

    let node = root.add_node(NodeWrapper {
        data: NodeData { position: Vector2(0, 0), size: Some(Vector2(100, 30)) },
        node: Box::new(Block {}),
    }, None).unwrap();

    root.add_node(NodeWrapper {
        data: NodeData { position: Vector2(0, 5), size: Some(Vector2(5, 1)) },
        node: Box::new(Label { 
            text: "Hello".to_string(),
            overflow: apheleia_widgets::label::TextOverflow::Ellipses,
        }),
    }, Some(node)).unwrap();

    root.initial_setup();
    root.run();
}
