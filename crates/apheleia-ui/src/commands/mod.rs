use apheleia_core::types::vector::Vector2;

use crate::rootnode::UpdateType;

#[derive(Default)]
pub struct InitialCallContext {
    commands: Vec<IntialCallCommands>
}

impl InitialCallContext {
    pub fn add_command(&mut self, command: IntialCallCommands) {
        self.commands.insert(0, command);
    }

    pub fn get_commands(&self) -> &Vec<IntialCallCommands> {
        &self.commands
    }
}

pub enum IntialCallCommands {
    SetSize(Vector2),

    RegisterUpdateType(UpdateType),
}
