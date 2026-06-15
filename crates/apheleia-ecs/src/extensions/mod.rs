pub(crate) mod store;
mod container;

use std::{any::Any, fmt::Debug};

pub trait Extension : Debug + Any {}
