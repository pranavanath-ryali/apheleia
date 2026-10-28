use std::mem::take;

use crate::{commands::WorldCommand, world::World};

impl World {
    #[inline]
    pub fn insert_command(&mut self, command: Box<dyn WorldCommand>) {
        self.commands.push(command);
    }

    #[inline]
    pub fn execute_commands(&mut self) {
        let commands = take(&mut self.commands);
        for command in commands {
            command.execute_commands(self);
        }
    }
}
