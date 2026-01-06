use apheleia_core::buffer::Buffer;

use crate::contexts::{EventUpdateContext, InitialCallContext};

pub trait NodeTrait {
    fn initial_setup(&mut self, ctx: &mut InitialCallContext);

    fn event(&mut self, ctx: &EventUpdateContext);
    fn update(&mut self);
    fn render(&self, buf: &mut Buffer);
}
