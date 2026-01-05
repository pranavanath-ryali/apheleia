use apheleia_core::buffer::Buffer;

use crate::contexts::{EventUpdateContext, InitialCallContext, RenderContext, UpdateContext};

pub trait NodeTrait {
    fn initial_setup(&mut self, ctx: &mut InitialCallContext);

    fn event(&mut self, ctx: &EventUpdateContext);
    fn update(&mut self, ctx: &mut UpdateContext);
    fn render(&self, ctx: &mut RenderContext, buf: &mut Buffer);
}
