use apheleia_app::{app::App, context::system::SystemContext, setup_logger};
use apheleia_core::{rich_strings::RichString, types::Vec2};
use apheleia_ecs::{
    constants::{EVENT_KEYS, STAGE},
    system_params::{
        query::{self, Query, query_filter::WithTag},
    },
    tags::TagTrait,
    types::NodeId,
};

#[derive(Debug)]
pub struct MyTag;
impl TagTrait for MyTag {}

fn main() {
    setup_logger();
    App::new()
        .build_node(|builder| {
            builder
                .tag(MyTag)
                .position(Vec2 { x: 0, y: 2 })
                .size(Vec2 { x: 20, y: 1 })
        })
        .build_node(|builder| {
            builder
                .tag(MyTag)
                .position(Vec2 { x: 0, y: 3 })
                .size(Vec2 { x: 20, y: 1 })
        })
        .build_node(|builder| {
            builder
                .position(Vec2 { x: 0, y: 4 })
                .size(Vec2 { x: 20, y: 1 })
        })
        .add_system(
            apheleia_ecs::types::SystemRunStage::Event,
            STAGE,
            do_something,
        )
        .run();
}

fn do_something(
    // _: OnAppEvent<EVENT_KEYS>,
    // q: Query<(NodeId, &mut LabelExtension), WithTag<MyTag>>,
) {
    // let (id, ext) = q.get_single().unwrap();
    // ext.text = RichString::new("WEEE");
}
