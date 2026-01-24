use apheleia_core::buffer::Buffer;

use crate::contexts::{EventUpdateContext, InitialCallContext, RenderContext, UpdateContext};

pub trait NodeTrait {
    fn initial_setup(&mut self, ctx: &mut InitialCallContext);

    fn event(&mut self, ctx: &mut EventUpdateContext);
    fn update(&mut self, ctx: &mut UpdateContext);
    fn render(&self, buf: &mut Buffer, ctx: &mut RenderContext);
}
