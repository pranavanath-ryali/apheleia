use apheleia_core::{
    style::{Style, StyleFlags},
    types::vector::Vector2,
};
use apheleia_ui::{
    node::data::NodeData,
    rootnode::{self, RootNode},
};
use apheleia_widgets::{
    block::Block,
    label::{self, Label, TextOverflow},
};

fn main() {
    let mut root = RootNode::default();

    root.add_node(
        "background",
        "",
        Block::new().build(),
        NodeData::new(Vector2(0, 0), Some(Vector2(50, 20))),
    );
    root.add_node(
        "label",
        "background",
        Label::new().with_label("My Label").build(),
        NodeData::new(Vector2(1, 1), Some(Vector2(10, 1))),
    );
    root.add_node(
        "label",
        "background",
        Label::new()
            .with_label("Hello World")
            .with_overflow(TextOverflow::Ellipses)
            .with_style(Style {
                fg: apheleia_core::Color::Red,
                bg: apheleia_core::Color::Grey,
                flags: StyleFlags::UnderLined | StyleFlags::Italic,
            })
            .build(),
        NodeData::new(Vector2(1, 2), Some(Vector2(5, 1))),
    );

    root.initial_setup();
    root.run();
}
