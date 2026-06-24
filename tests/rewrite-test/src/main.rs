use apheleia_app::{app::App, context::system::SystemContext, setup_logger};
use apheleia_core::{rich_strings::RichString, types::Vec2};
use apheleia_ecs::{
    constants::{EVENT_KEYS, STAGE},
    system_params::{
        app_event::OnAppEvent,
        query::{self, Query, query_filter::WithTag},
    },
    tags::TagTrait,
    types::NodeId,
};
use apheleia_widgets::label::{self, HorizontalAlignment, LabelExtension, LabelWidget};

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
                .node(
                    LabelWidget::new(RichString::new("Hello </fg:red/>World"))
                        .horizontal_alignment(HorizontalAlignment::Left),
                )
        })
        .build_node(|builder| {
            builder
                .tag(MyTag)
                .position(Vec2 { x: 0, y: 3 })
                .size(Vec2 { x: 20, y: 1 })
                .node(
                    LabelWidget::new(RichString::new("Hello </fg:red/>World"))
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
        })
        .build_node(|builder| {
            builder
                .position(Vec2 { x: 0, y: 4 })
                .size(Vec2 { x: 20, y: 1 })
                .node(
                    LabelWidget::new(RichString::new("Hello </fg:red/>World"))
                        .horizontal_alignment(HorizontalAlignment::Right),
                )
        })
        .add_system(
            apheleia_ecs::types::SystemRunStage::Event,
            STAGE,
            do_something,
        )
        .run();
}

fn do_something(
    _: OnAppEvent<EVENT_KEYS>,
    q: Query<(NodeId, &mut LabelExtension), WithTag<MyTag>>,
    mut ctx: SystemContext,
) {
    let (id, ext) = q.get_single().unwrap();
    ext.text = RichString::new("WEEE");

    ctx.mark_render_dirty(id);
}
