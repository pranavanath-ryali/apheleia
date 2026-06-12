use apheleia_app_new::{
    app::App, context::system::SystemContext, node_definer::NodeDefiner, params::on_event::OnKeys,
    setup_logger, types::EVENT_KEYS,
};
use apheleia_core::{rich_strings::RichString, types::Vec2};
use apheleia_ecs_new::constants::PRE_STAGE;
use crossterm::event::KeyModifiers;

use crate::widget::BasicTextDefiner;

const MY_TAG: usize = 0;

mod widget {
    use apheleia_app_new::{
        context::system::SystemContext,
        node_definer::NodeDefiner,
        params::{
            extension::{Query, With},
            on_event::{OnEvent, OnKeys},
            resource::{Res, ResMut},
        },
    };
    use apheleia_core::{rich_strings::RichString, style::Style, types::Vec2};
    use apheleia_ecs_new::{
        NodeId,
        constants::{POST_STAGE, STAGE},
        extensions::Extension,
        resources::Resource,
        systems::stages::{self, SystemRunStage::{self, Update}},
        types::NodeData,
    };

    #[derive(Debug)]
    pub struct TestRes {
        value: f32,
    }
    impl Resource for TestRes {}

    #[derive(Debug)]
    pub struct BasicTextDefiner {
        pub text: RichString,
    }
    impl NodeDefiner for BasicTextDefiner {
        fn setup(&mut self, ctx: &mut apheleia_app_new::context::node::NodeContext) {
            ctx.add_system(Update, STAGE, mut_system);
            ctx.add_system(Update, STAGE, update_system);
            ctx.add_system(
                SystemRunStage::Render,
                STAGE,
                render,
            );
            // ctx.add_resource(TestRes { value: 123.0 });

            ctx.add_extension(TestExtension { value: 1 });
        }
    }

    #[derive(Debug)]
    pub struct TestExtension {
        value: i32,
    }
    impl Extension for TestExtension {}

    fn mut_system(mut res: ResMut<TestRes>, ctx: SystemContext) {
        res.value += 1.0;
    }

    fn update_system(res: Res<TestRes>) {}

    fn render(query: Query<(NodeId, &TestExtension)>, mut ctx: SystemContext) {
        for (id, data) in query.iter() {
            let buffer = ctx.get_buffer(id).expect("No Buffer?");
            buffer.write_string(Vec2::zero(), format!("{}", id), None);
        }
    }
}

fn main() {
    setup_logger();
    App::new()
        .add_system(
            apheleia_ecs_new::systems::stages::SystemRunStage::Event,
            PRE_STAGE,
            exit_app,
        )
        .build_node(|builder| {
            builder
                .tag::<MY_TAG>()
                .position(Vec2 { x: 10, y: 10 })
                .size(Vec2 { x: 1, y: 2 })
                .node(BasicTextDefiner {
                    text: RichString::new("YAY"),
                })
        })
        .build_node(|builder| {
            builder
                .tag::<MY_TAG>()
                .position(Vec2 { x: 5, y: 10 })
                .size(Vec2 { x: 1, y: 2 })
        })
        .run();
}

fn exit_app(event: OnKeys) {
    if event.code.is_char('c') && event.modifiers.contains(KeyModifiers::CONTROL) {
        println!("Should Quit!");
    }
}
