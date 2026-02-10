use apheleia_core::types::vector::Vector2;
use apheleia_ui::{KeyCode, contexts::{self, Commands, Context, EventData}, node::{data::{self, NodeData}, node::NodeTrait}, rootnode::RootNode};

struct TestNode(bool);
impl NodeTrait for TestNode {
    fn initial_setup(&mut self, ctx: &mut Context, _data: &NodeData) {
        ctx.add_command(Commands::SetSize(Vector2(5, 1)));
        ctx.add_command(Commands::RegisterForEvent(apheleia_ui::types::EventType::Keys));
    }

    fn event(&mut self, ctx: &mut Context, _: &NodeData) {
        match ctx.get_event().as_ref().unwrap() {
            EventData::Keys(event) => {
                if event.code == KeyCode::Char('a') {
                    self.0 = true;

                    ctx.add_command(Commands::MarkRenderDirty(apheleia_ui::node::data::DirtyRenderLevel::SimpleDirty));
                }
            }
            _ => ()
        }
    }

    fn render(&self, buf: &mut apheleia_core::buffer::Buffer, ctx: &Context, _: &NodeData) {
        if self.0 {
            buf.write_line(0, 0, "B", None);
        } else {
            buf.write_line(0, 0, "AAAAAA", None);
        }
    }

    fn update(&mut self, ctx: &mut contexts::Context, _: &NodeData) {
        todo!()
    }
}


fn main() {
    let mut root = RootNode::default();

    root.add_node("test_node", "", Box::new(TestNode(false)), NodeData::new(Vector2(20, 3), None));

    root.initial_setup();
    root.run();
}
