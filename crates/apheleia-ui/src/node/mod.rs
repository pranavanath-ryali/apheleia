use std::any::Any;

use apheleia_core::buffer::Buffer;

use crate::contexts::{self, Context};

pub mod data;
pub mod storage;

pub trait NodeTrait: Any {
    fn initial_setup(&mut self, ctx: &mut Context);

    fn event(&mut self, ctx: &mut Context);
    fn update(&mut self, ctx: &mut Context);
    fn render(&self, buf: &mut Buffer, ctx: &mut Context);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct EmptyNode;
impl NodeTrait for EmptyNode {
    fn initial_setup(&mut self, _ctx: &mut contexts::Context) {}
    fn event(&mut self, _ctx: &mut contexts::Context) {}
    fn update(&mut self, _ctx: &mut contexts::Context) {}
    fn render(&self, _buf: &mut apheleia_core::buffer::Buffer, _ctx: &mut contexts::Context) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
