use apheleia_ui::{FAKE_NODEID, node::data::NodeData, rootnode::RootNode};
use apheleia_widgets::label::Label;

fn main() {
    let mut root = RootNode::new();
    
    // TODO: Have options for some of these fields. especially the ids, width and height and let
    // the widget define its own width and height
    root.add_node(NodeData {
        id: FAKE_NODEID,

        x: 0,
        y: 0,

        width: 100,
        height: 1,

        node: Box::new(Label::new("Hello World"))
    });
    let label = Label::new("Hello World");

    root.start();
}
