use apheleia_app::{app::App, events::app_events::params::OnKeys, setup_logger};
use apheleia_core::types::Vec2;
use apheleia_ecs::constants::STAGE;
use apheleia_widgets::label::LabelWidget;
use crossterm::event::KeyCode;
use log::info;

fn main() {
    setup_logger();
    App::new()
        .build_node(|builder| {
            builder.size(Vec2 { x: 20, y: 3 }).node(
                LabelWidget::new("</bg:blue;fg:red;italic;bold;/>Hello World")
                    .horizontal_alignment(apheleia_widgets::label::HorizontalAlignment::Center)
                    .vertical_alignment(apheleia_widgets::label::VerticalAlignment::Center),
            )
        })
        .add_system(
            apheleia_ecs::types::SystemRunStage::Event,
            STAGE,
            quit_on_esc,
        )
        .run();
}

fn quit_on_esc(e: OnKeys) {
    if e.code == KeyCode::Esc {
        info!("YUP WE SHOULD DO IT?!?!!Y!LM@N!@I#N!");
    }
}
