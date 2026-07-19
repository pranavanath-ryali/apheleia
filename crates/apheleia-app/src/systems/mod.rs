use crossterm::event::KeyCode;

use crate::{app::{App, Quit}, events::app_events::params::OnKeys, params::global_emitter::GlobalEmitter};

pub fn quit_on_ecs(
    keys: OnKeys,
    mut emitter: GlobalEmitter<App, Quit>,
) {
    if KeyCode::Esc == keys.code {
        emitter.emit();
    }
}
