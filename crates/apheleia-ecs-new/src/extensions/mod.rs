pub(crate) mod store;
mod container;

use std::any::Any;

pub trait Extension : Any {}
