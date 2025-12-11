use apheleia_ui::{node::Node, rootnode::RootNode};

fn main() {
    let mut root = RootNode::new();

    let node = Node {
        x: 10,
        y: 10,

        width: 10,
        height: 3,
        ..Default::default()
    };

    root.add_node(Box::new(node));

    root.start();
}
