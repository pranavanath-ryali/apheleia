use apheleia::{
    Vec2,
    label::{HorizontalAlignment, LabelNode, VerticalAlignment},
    rich_strings::RichString,
    root::Root,
};

fn main() {
    let mut root = Root::default();
    let size = root.get_size();

    root.create_node(|builder| {
        builder.set_size(size).node(
            LabelNode::new(RichString::new("Hello World!"))
                .set_horizontal_align(HorizontalAlignment::Center)
                .set_vertical_align(VerticalAlignment::Center),
        )
    });

    root.run();
}
