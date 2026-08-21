use apheleia_app::{
    app::App, events::app_events::params::OnKeys, params::local_events::EventEmitter,
    systems::quit_on_ecs,
};
use apheleia_core::rich_strings::RichString;
use apheleia_ecs::{
    constants::{FIRST, STAGE},
    params::{query::Query, query_filters::tag::WithTag, resmut::ResMut},
    runtime_expressions::{
        Constant, Expr, ExprVec, Expression,
        values::{ParentHeight, ParentWidth, TerminalHeight, TerminalWidth, ThisHeight, ThisWidth},
    },
    stores::events::RenderDirty,
    traits::{resource::Resource, tag::TagTrait},
    types::{NodeId, SystemRunStage},
};
use apheleia_widgets::{
    extensions::label::{HorizontalAlignment, LabelExtension},
    widgets::{container::ContainerWidget, label::LabelWidget},
};
use crossterm::event::KeyCode;

#[derive(Debug)]
pub struct CounterRes(pub u32);
impl Resource for CounterRes {}

#[derive(Debug)]
pub struct LabelTag;
impl TagTrait for LabelTag {}

fn main() {
    App::new()
        .add_resource((CounterRes(0),)) // Gotta fix this
        .create_node(|builder| {
            builder
                .position(ExprVec { // IK. UGLY AF
                    x: Expression(Expr::Sub(
                        Box::new(Expr::Divide(
                            Box::new(Expr::Value(Box::new(TerminalWidth))),
                            Box::new(Expr::Value(Box::new(Constant(2)))),
                        )),
                        Box::new(Expr::Divide(
                            Box::new(Expr::Value(Box::new(ThisWidth))),
                            Box::new(Expr::Value(Box::new(Constant(2)))),
                        )),
                    )),
                    y: Expression(Expr::Sub(
                        Box::new(Expr::Divide(
                            Box::new(Expr::Value(Box::new(TerminalHeight))),
                            Box::new(Expr::Value(Box::new(Constant(2)))),
                        )),
                        Box::new(Expr::Divide(
                            Box::new(Expr::Value(Box::new(ThisHeight))),
                            Box::new(Expr::Value(Box::new(Constant(2)))),
                        )),
                    )),
                })
                .size(ExprVec {
                    x: Expression(Expr::Divide(
                        Box::new(Expr::Value(Box::new(TerminalWidth))),
                        Box::new(Expr::Value(Box::new(Constant(2)))),
                    )),
                    y: Expression(Expr::Divide(
                        Box::new(Expr::Value(Box::new(TerminalHeight))),
                        Box::new(Expr::Value(Box::new(Constant(4)))),
                    )),
                })
                .node(
                    ContainerWidget::new().rounded().header(
                        LabelWidget::new("</reverse;slow_blink/>[EXAMPLE COUNTER APP]")
                            .horizontal_alignment(HorizontalAlignment::Center),
                    ),
                )
                .create_child(|builder| {
                    builder
                        .tag(LabelTag)
                        .position(ExprVec {
                            x: Expression(Expr::Sub(
                                Box::new(Expr::Divide(
                                    Box::new(Expr::Value(Box::new(ParentWidth))),
                                    Box::new(Expr::Value(Box::new(Constant(2)))),
                                )),
                                Box::new(Expr::Divide(
                                    Box::new(Expr::Value(Box::new(ThisWidth))),
                                    Box::new(Expr::Value(Box::new(Constant(2)))),
                                )),
                            )),
                            y: Expression(Expr::Sub(
                                Box::new(Expr::Divide(
                                    Box::new(Expr::Value(Box::new(ParentHeight))),
                                    Box::new(Expr::Value(Box::new(Constant(2)))),
                                )),
                                Box::new(Expr::Divide(
                                    Box::new(Expr::Value(Box::new(ThisHeight))),
                                    Box::new(Expr::Value(Box::new(Constant(2)))),
                                )),
                            )),
                        })
                        .size(ExprVec {
                            x: Expression(Expr::Value(Box::new(Constant(20)))),
                            y: Expression(Expr::Value(Box::new(Constant(1)))),
                        })
                        .node(
                            LabelWidget::new("</bold;italic/>COUNTER: 0")
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                })
        })
        .add_system(SystemRunStage::Event, FIRST, quit_on_ecs)
        .add_system(SystemRunStage::Event, STAGE, increment_count)
        .run();
}

pub fn increment_count(
    keys: OnKeys,
    mut counter: ResMut<CounterRes>,
    query: Query<(NodeId, &mut LabelExtension), WithTag<LabelTag>>,
    mut emitter: EventEmitter<RenderDirty>,
) {
    if keys.code == KeyCode::Enter {
        counter.0 += 1;
        for (id, label) in query.iter() {
            label.text = RichString::new(&format!("</bold;italic;fg:blue/>COUNTER: {}", counter.0));
            emitter.mark_parent(id);
            // emitter.mark(id);
        }
    }
}
