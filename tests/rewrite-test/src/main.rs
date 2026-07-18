use apheleia_app::{
    app::App, events::app_events::params::OnKeys, params::local_events::EventEmitter,
    resources::event_tracker::{EventMarker, RenderDirty}, setup_logger,
};
use apheleia_core::{rich_strings::RichString, types::Vec2};
use apheleia_ecs::{
    constants::STAGE,
    params::query::{
        Query,
        query_filter::{With, WithTag},
    },
    runtime_expressions::{
        Constant, Expr, ExprVec, Expression,
        values::{ParentHeight, ParentWidth, TerminalHeight, TerminalWidth},
    },
    tags::TagTrait,
    types::{NodeId, SystemRunStage},
};
use apheleia_widgets::{
    container::ContainerWidget,
    label::{HorizontalAlignment, LabelExtension, LabelWidget, VerticalAlignment},
};
use crossterm::event::KeyCode;
use log::info;

#[derive(Debug)]
pub struct MyTag;
impl TagTrait for MyTag {}

fn main() {
    setup_logger();
    App::new()
        .create_node(|builder| {
            builder
                .size(ExprVec {
                    x: Expression(Expr::Value(Box::new(TerminalWidth))),
                    y: Expression(Expr::Value(Box::new(TerminalHeight))),
                })
                .node(
                    ContainerWidget::new()
                        .rounded()
                        .header(LabelWidget::new("This is a header")),
                )
                .create_child(|builder| {
                    builder
                        .position(ExprVec {
                            x: Expression(Expr::Value(Box::new(Constant(1)))),
                            y: Expression(Expr::Value(Box::new(Constant(1)))),
                        })
                        .size(ExprVec {
                            x: Expression(Expr::Sub(
                                Box::new(Expr::Divide(
                                    Box::new(Expr::Value(Box::new(ParentWidth))),
                                    Box::new(Expr::Value(Box::new(Constant(2)))),
                                )),
                                Box::new(Expr::Value(Box::new(Constant(1)))),
                            )),
                            y: Expression(Expr::Sub(
                                Box::new(Expr::Value(Box::new(ParentHeight))),
                                Box::new(Expr::Value(Box::new(Constant(2)))),
                            )),
                        })
                        .node(
                            ContainerWidget::new()
                                .boxed()
                                .header(LabelWidget::new("This is Container 1!!!!!!")),
                        )
                        .create_child(|builder| {
                            builder
                                .tag(MyTag)
                                .position(ExprVec {
                                    x: Expression(Expr::Value(Box::new(Constant(1)))),
                                    y: Expression(Expr::Value(Box::new(Constant(1)))),
                                })
                                .size(ExprVec {
                                    x: Expression(Expr::Value(Box::new(ParentWidth))),
                                    y: Expression(Expr::Value(Box::new(Constant(1)))),
                                })
                                .node(LabelWidget::new("Hello From Label"))
                        })
                })
                .create_child(|builder| {
                    builder
                        .position(ExprVec {
                            x: Expression(Expr::Divide(
                                Box::new(Expr::Value(Box::new(ParentWidth))),
                                Box::new(Expr::Value(Box::new(Constant(2)))),
                            )),
                            y: Expression(Expr::Value(Box::new(Constant(1)))),
                        })
                        .size(ExprVec {
                            x: Expression(Expr::Sub(
                                Box::new(Expr::Divide(
                                    Box::new(Expr::Value(Box::new(ParentWidth))),
                                    Box::new(Expr::Value(Box::new(Constant(2)))),
                                )),
                                Box::new(Expr::Value(Box::new(Constant(1)))),
                            )),
                            y: Expression(Expr::Sub(
                                Box::new(Expr::Value(Box::new(ParentHeight))),
                                Box::new(Expr::Value(Box::new(Constant(2)))),
                            )),
                        })
                        .node(
                            ContainerWidget::new()
                                .boxed()
                                .header(LabelWidget::new("This is Container 2!!!!!!!")),
                        )
                })
        })
        .add_system(SystemRunStage::Event, STAGE, change_label_on_key)
        .run();
}

fn change_label_on_key(
    _: OnKeys,
    query: Query<(NodeId, &mut LabelExtension)>,
    mut emitter: EventEmitter<RenderDirty>,
) {
    for (id, ext) in query.iter() {
        ext.text = RichString::new("CHANGED!");
        emitter.mark(id);
    }
}
