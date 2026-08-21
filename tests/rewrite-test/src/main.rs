use apheleia_app::{app::App, setup_logger, systems::quit_on_ecs};
use apheleia_ecs::{
    constants::LAST,
    runtime_expressions::{
        Constant, Expr, ExprVec, Expression,
        values::{TerminalHeight, TerminalWidth},
    },
    types::SystemRunStage,
};
use apheleia_widgets::widgets::label::LabelWidget;

fn main() {
    setup_logger();

    // The text we want to center
    let text = "Hello World";
    let text_len = text.len();

    App::new()
        .create_node(|builder| {
            builder
                .position(ExprVec {
                    x: Expression(Expr::Divide(
                        Box::new(Expr::Sub(
                            Box::new(Expr::Value(Box::new(TerminalWidth))),
                            Box::new(Expr::Value(Box::new(Constant(
                                text_len.try_into().unwrap(),
                            )))),
                        )),
                        Box::new(Expr::Value(Box::new(Constant(2)))),
                    )),
                    y: Expression(Expr::Divide(
                        Box::new(Expr::Value(Box::new(TerminalHeight))),
                        Box::new(Expr::Value(Box::new(Constant(2)))),
                    )),
                })
                .size(ExprVec {
                    x: Expression(Expr::Value(Box::new(Constant(
                        text_len.try_into().unwrap(),
                    )))),
                    y: Expression(Expr::Value(Box::new(Constant(1)))),
                })
                .node(LabelWidget::new(text))
        })
        .add_system(SystemRunStage::Event, LAST, quit_on_ecs)
        .run();
}
