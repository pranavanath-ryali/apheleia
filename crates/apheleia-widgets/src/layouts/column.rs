use apheleia_core::buffer::{self, Buffer};
use apheleia_ui::{contexts::{self, Context}, node::{self, data::{self, NodeData}, node::NodeTrait}};

pub struct ColumnLayout {

}
impl NodeTrait for ColumnLayout {
    fn initial_setup(&mut self, ctx: &mut Context, data: &NodeData) {
    }

    fn event(&mut self, ctx: &mut Context, data: &NodeData) {
        todo!()
    }

    fn update(&mut self, ctx: &mut Context, data: &NodeData) {}

    fn render(&self, buf: &mut Buffer, ctx: &Context, data: &NodeData) {}
}
