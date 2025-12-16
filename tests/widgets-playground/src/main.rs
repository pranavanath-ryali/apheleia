use apheleia_core::{
    style::{Style, StyleFlags},
    types::vector::Vector2,
};
use apheleia_ui::{
    node::data::{NodeData, NodeWrapper},
    rootnode::{self, RootNode},
};
use apheleia_widgets::{
    block::Block,
    label::{Label, LabelAlignment, TextOverflow},
};

fn main() {
    let mut root = RootNode::default();

    let node = root
        .add_node(
            NodeWrapper {
                data: NodeData {
                    position: Vector2(0, 0),
                    size: Some(Vector2(100, 30)),
                },
                node: Box::new(Block {}),
            },
            None,
        )
        .unwrap();

    root.add_node(
        NodeWrapper {
            data: NodeData {
                position: Vector2(0, 5),
                size: Some(Vector2(50, 1)),
            },
            node: Box::new(Label::new(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris finibus vestibulum justo in rhoncus. Fusce commodo tincidunt nulla non vulputate. Maecenas nisl turpis, faucibus et tincidunt ac, eleifend vitae ex. Praesent porttitor commodo lacus, eleifend sodales magna fermentum sed. Vestibulum ac ultricies est. Lorem ipsum dolor sit amet, consectetur adipiscing.",
                Some(Style {
                    flags: StyleFlags::ITALIC | StyleFlags::BOLD,
                    ..Default::default()
                }),
                None,
                Some(TextOverflow::Scoll(1, 10)),
            )),
        },
        Some(node),
    )
    .unwrap();

    root.add_node(
        NodeWrapper {
            data: NodeData {
                position: Vector2(0, 10),
                size: Some(Vector2(20, 1)),
            },
            node: Box::new(Label::new(
                "Label Alignment",
                Some(Style {
                    fg: apheleia_core::Color::Black,
                    bg: apheleia_core::Color::Blue,
                    flags: StyleFlags::ITALIC | StyleFlags::BOLD,
                    ..Default::default()
                }),
                Some(LabelAlignment::Center),
                Some(TextOverflow::Scoll(1, 10)),
            )),
        },
        Some(node),
    )
    .unwrap();

    root.initial_setup();
    root.run();
}
