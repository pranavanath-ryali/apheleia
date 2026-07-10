pub(crate) mod store;
pub mod buffer_store;

use std::{any::Any, fmt::Debug};

pub trait Resource : Any + Debug {}
