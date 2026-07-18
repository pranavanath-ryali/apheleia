use apheleia_app::{
    app::App, events::app_events::params::OnKeys, params::local_events::EventEmitter,
    resources::event_tracker::RenderDirty, setup_logger,
};
use apheleia_core::types::Vec2;
use apheleia_ecs::{
    constants::STAGE,
    runtime_expressions::{Constant, Expr, ExprVec, Expression},
};
use apheleia_widgets::label::LabelWidget;
use crossterm::event::KeyCode;
use log::info;

fn main() {
    setup_logger();
    App::new()
        .build_node(|builder| {
            builder
                .position(ExprVec {
                    x: Expression(Expr::Value(Box::new(
                        Constant(5),
                    ))),
                    y: Expression(Expr::Value(Box::new(
                        Constant(1),
                    ))),
                })
                .size(ExprVec {
                    x: Expression(Expr::Value(Box::new(
                        Constant(10),
                    ))),
                    y: Expression(Expr::Value(Box::new(
                        Constant(1),
                    ))),
                })
                .node(LabelWidget::new("Hello There"))
        })
        .run();
}
