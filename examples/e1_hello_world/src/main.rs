use apheleia_ui::{
    contexts::system::SystemContext, node::traits::NodeTrait, root::Root, types::UpdateType,
    vector::Vector2,
};

fn main() {
    // TODO: Instead of creating a custom node. Eventually, use a prebuild LabelNode
    let mut root = Root::default();
    let width = root.width;
    let height = root.height;

    // root.create_node("hello_world_node")
    //     .set_size(Vector2(12, 1))
    //     .set_position(Vector2((width / 2) - 6, height / 2))
    //     .build(HelloWorldNode);

    root.run();
}
