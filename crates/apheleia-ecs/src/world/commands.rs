use super::*;

impl World {
    /// Add the given [`ContextCommand`] to queue
    ///
    /// # Arguments
    ///
    /// * `command` - The boxed value of the command to queue
    #[inline]
    pub fn add_command(&mut self, command: Box<dyn ContextCommand>) {
        warn!("[ECS] Added command: {:?}", command);
        self.commands.push_back(command);
    }

    /// Consumes and appends the given [`VecDeque<Box<dyn ContextCommand>>`]
    ///
    /// # Arguments
    ///
    /// * `commands` - The array of commands that are consumed and added to [`World`]'s
    ///   command_queue
    #[inline]
    pub fn apppend_commands(&mut self, commands: &mut VecDeque<Box<dyn ContextCommand>>) {
        warn!("[ECS] Commands appended: {:#?}", commands);
        self.commands.append(commands);
    }

    /// Execute all [`ContextCommand`]s that are buffered in [`World`]'s command queue
    pub fn execute_commands(&mut self) {
        warn!("[ECS] Executing commands");
        let commands = take(&mut self.commands);
        for command in commands {
            command.execute(self);
        }
        info!("[ECS] Commands executed");
    }
}
