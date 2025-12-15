use apheleia_core::types::vector::Vector2;
use apheleia_ui::{node::data::{NodeData, NodeWrapper}, rootnode::{self, RootNode}};
use apheleia_widgets::block::Block;

fn main() {
    let mut root = RootNode::default();

    root.add_node(NodeWrapper {
        data: NodeData { position: Vector2(0, 0), size: Some(Vector2(100, 30)) },
        node: Box::new(Block {}),
    }, None);

    root.initial_setup();
    root.run();
}
