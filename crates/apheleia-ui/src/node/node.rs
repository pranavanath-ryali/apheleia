use apheleia_core::buffer::Buffer;

use crate::{commands::InitialCallContext, contexts::{RenderContext, UpdateContext}, node::data::{NodeData, NodeWrapperTrait}};

pub trait NodeTrait {
    fn initial_setup(&mut self, ctx: &mut InitialCallContext);

    fn event(&mut self);
    fn update(&mut self, ctx: &mut UpdateContext);
    fn render(&self, ctx: &mut RenderContext, buf: &mut Buffer);
}
