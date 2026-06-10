use apheleia_app_new::{app::App, node_definer::NodeDefiner, setup_logger};
use apheleia_core::{rich_strings::RichString, types::Vec2};

use crate::widget::BasicTextDefiner;

const MY_TAG: usize = 0;

mod widget {
    use apheleia_app_new::node_definer::NodeDefiner;
    use apheleia_core::rich_strings::RichString;
    use apheleia_ecs_new::{constants::STAGE, systems::stages::SystemRunStage::Update};

    #[derive(Debug)]
    pub struct BasicTextDefiner {
        pub text: RichString,
    }
    impl NodeDefiner for BasicTextDefiner {
        fn setup(&mut self, ctx: &mut apheleia_app_new::context::node::NodeContext) {
            ctx.add_system(
                Update,
                STAGE,
                update_system,
            );
        }
    }

    fn update_system() {
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
                .node(BasicTextDefiner {
                    text: RichString::new("YAY"),
                })
        })
        .run();
}
