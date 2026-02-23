use apheleia_core::types::vector::Vector2;
use apheleia_ui::{
    KeyCode,
    contexts::{
        Context, commands::{Command_MarkRenderDirty, Command_RegisterForEvent, Command_SetSizeForId}
    },
    node::{
        data::{self, NodeData},
        node::NodeTrait,
    },
    rootnode::RootNode,
    types::{self, DirtyRenderLevel, EventData, EventType},
};

struct TestNode(bool);
impl NodeTrait for TestNode {
    fn initial_setup(&mut self, ctx: &mut Context) {
        ctx.add_command(Box::new(Command_SetSizeForId(ctx.get_id(), Vector2(10, 1))));
        ctx.add_command(Box::new(Command_RegisterForEvent(EventType::Keys)));
    }

    fn event(&mut self, ctx: &mut Context) {
        match ctx.get_event().as_ref().unwrap() {
            EventData::Keys(event) => {
                if event.code == KeyCode::Char('a') {
                    self.0 = true;

                    ctx.add_command(Box::new(Command_MarkRenderDirty(
                        ctx.get_id(),
                        DirtyRenderLevel::SimpleDirty,
                    )));
                }
            }
            _ => (),
        }
    }

    fn render(&self, buf: &mut apheleia_core::buffer::Buffer, ctx: &mut Context) {
        if self.0 {
            buf.write_line(0, 0, "B", None);
        } else {
            buf.write_line(0, 0, "AAAAAA", None);
        }
    }

    fn update(&mut self, ctx: &mut Context) {
        todo!()
    }
}

fn main() {
    let mut root = RootNode::default();

    root.add_node(
        "test_node",
        "",
        Box::new(TestNode(false)),
        NodeData::new(Vector2(20, 3), None),
    );

    root.initial_setup();
    root.run();
}
