use apheleia_core::buffer::NodeBuffer;

use crate::node::data::{NodeData, NodeWrapperTrait};

pub trait NodeTrait {
    fn initial_setup(&mut self, data: &mut NodeData);

    fn update(&mut self);
    fn render(&self, buf: &mut NodeBuffer);
}
