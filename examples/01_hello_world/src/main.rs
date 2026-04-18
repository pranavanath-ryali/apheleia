use apheleia::{Vector2, label::LabelNode, rich_strings::RichString, root::Root};

fn main() {
    let mut root = Root::default();

    let width = root.width;
    let height = root.height;

    root.create_node(|builder| {
        builder.set_size(Vector2(width, height)).node(
            LabelNode::new(RichString::new("Hello World!"))
                .set_horizontal_align(apheleia::label::HorizontalAlignment::Center)
                .set_vertical_align(apheleia::label::VerticalAlignment::Center),
        )
    });

    root.run();
}
