use crate::world::WorldViewForCommands;

pub trait ContextCommand {
    fn execute(self: Box<Self>, world: &mut WorldViewForCommands);
}
