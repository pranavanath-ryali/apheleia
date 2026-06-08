pub mod node;

use core::fmt::Debug;
use crate::app::App;

pub trait ContextCommand: Debug {
    fn execute(&self, app: &mut App);
}
