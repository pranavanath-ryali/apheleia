pub mod node;
pub mod extension;
pub mod resource;
pub mod system;
pub mod tag;

use core::fmt::Debug;

use crate::world::World;

pub trait ContextCommand: Debug {
    fn execute(self: Box<Self>, world: &mut World);
}
