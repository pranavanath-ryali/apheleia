use apheleia_ui::{rootnode::RootNode};
use apheleia_ui::node::{BasicNode, NodeData};

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
