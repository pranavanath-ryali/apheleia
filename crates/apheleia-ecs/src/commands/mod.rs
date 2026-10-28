pub mod resource;
pub mod node;
pub mod extension;
pub mod nodedata;

use crate::world::World;

pub trait WorldCommand {
    fn execute(self: Box<Self>, world: &mut World);
}
