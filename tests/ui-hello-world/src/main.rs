use apheleia_core::types::vector::Vector2;
use apheleia_ui::{KeyCode, contexts::{self, IntialCallCommands}, node::{data::NodeData, node::NodeTrait}, rootnode::RootNode, types::{self, EventType}};

struct TestNode(bool);
impl NodeTrait for TestNode {
    fn initial_setup(&mut self, ctx: &mut contexts::InitialCallContext) {
        ctx.add_command(IntialCallCommands::SetSize(Vector2(5, 1)));
        ctx.add_command(IntialCallCommands::RegisterForEvent(EventType::Keys));

        println!("YAYYY");
    }

    fn event(&mut self, ctx: &mut contexts::EventUpdateContext) {
        match ctx.event_data {
            types::EventData::Keys(event) => {
                if event.code == KeyCode::Char('a') {
                    self.0 = true;

                    ctx.add_command(contexts::EventUpdateCommands::MarkRenderDirty(apheleia_ui::node::data::DirtyRenderLevel::SimpleDirty));
                }
            },
            _ => ()
        }
    }

    fn update(&mut self) {
    }

    fn render(&self, buf: &mut apheleia_core::buffer::Buffer) {
        if self.0 {
            buf.write_line(0, 0, "B", None);
        } else {
            buf.write_line(0, 0, "AAAAAA", None);
        }
    }
}

fn main() {
    let mut root = RootNode::default();

    root.add_node("test_node", "", Box::new(TestNode(false)), NodeData::new(Vector2(20, 0)));

    root.initial_setup();
    root.run();
}
