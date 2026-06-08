use core::fmt::Debug;
use crate::app::App;

pub trait Command: Debug {
    fn execute(&self, app: &mut App);
}
