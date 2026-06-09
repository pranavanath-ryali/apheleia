pub mod node;
pub mod resource;

use core::fmt::Debug;
use crate::app::App;

pub trait ContextCommand: Debug {
    fn execute(self: Box<Self>, app: &mut App);
}
