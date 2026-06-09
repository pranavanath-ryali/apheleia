use apheleia_app_new::{app::App, node_definer::NodeDefiner, setup_logger};
use apheleia_core::types::Vec2;

const MY_TAG: usize = 0;

#[derive(Debug)]
struct CustomDefiner {
    pub f: f32,
}
impl NodeDefiner for CustomDefiner {
    fn setup(&mut self, ctx: &mut apheleia_app_new::context::node::NodeContext) {
    }
}

fn main() {
    setup_logger();
    App::new()
        .build_node(|builder| {
            builder
                .tag::<MY_TAG>()
                .position(Vec2 { x: 10, y: 10 })
                .size(Vec2 { x: 1, y: 2 })
                .node(CustomDefiner { f: 1.23 })
        })
        .run();
}
