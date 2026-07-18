use crate::runtime_expressions::ExprValue;

pub struct TerminalWidth;
impl ExprValue for TerminalWidth {
    fn result(&self, world: &crate::world::World) -> u32 {
        world.terminal_size.x as u32
    }
}

pub struct TerminalHeight;
impl ExprValue for TerminalHeight {
    fn result(&self, world: &crate::world::World) -> u32 {
        world.terminal_size.y as u32
    }
}
