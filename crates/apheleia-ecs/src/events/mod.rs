use std::fmt::Debug;


pub(crate) mod tracker;

pub trait EventTrait : Debug + 'static {}

#[derive(Debug)]
pub struct RenderDirty;
impl EventTrait for RenderDirty {}
