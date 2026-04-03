use apheleia_ui::{
    contexts::system::SystemContext, node::traits::NodeTrait, root::Root, types::UpdateType,
    vector::Vector2,
};

fn render_hello_world(ctx: &mut SystemContext) {
    let buffer = ctx.get_buffer();

    buffer.write_line(0, 0, "Hello World!", None);
}

struct HelloWorldNode;
impl NodeTrait for HelloWorldNode {
    fn initial_setup(&mut self, ctx: &mut apheleia_ui::contexts::node::NodeContext) {
        ctx.add_system(UpdateType::Render, 0, render_hello_world);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn main() {
    let mut root = Root::default();
    let width = root.width;
    let height = root.height;

    root.create_node("hello_world_node")
        .set_size(Vector2(12, 1))
        .set_position(Vector2((width / 2) - 6, height / 2))
        .build(HelloWorldNode);

    root.run();
}
