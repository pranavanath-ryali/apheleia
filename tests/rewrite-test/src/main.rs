use apheleia_app_new::{app::App, node_definer::NodeDefiner, setup_logger};
use apheleia_core::types::Vec2;
use apheleia_ecs_new::{constants::STAGE, systems::{stages::SystemRunStage, system::SystemParam}};

const MY_TAG: usize = 0;

#[derive(Debug)]
struct CustomDefiner {
    pub f: f32,
}
impl NodeDefiner for CustomDefiner {
    fn setup(&mut self, ctx: &mut apheleia_app_new::context::node::NodeContext) {}
}

struct TestParam;
impl SystemParam for TestParam {
    unsafe fn fetch(world: *mut apheleia_ecs_new::world::World) -> Option<Self> {
        Some(TestParam)
    }
}

fn test_system(_: TestParam) {
    println!("HELLO");
}
fn another_test_system() {
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
        .add_system(SystemRunStage::Render, STAGE, test_system)
        .add_system(SystemRunStage::Render, STAGE, another_test_system)
        .run();
}
