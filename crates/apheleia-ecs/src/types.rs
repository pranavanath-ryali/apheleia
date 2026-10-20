use std::any::Any;

use num_traits::Num;

pub type NodeId = u16;
pub type ExtensionId = u16;

pub trait Extension : Any {}
