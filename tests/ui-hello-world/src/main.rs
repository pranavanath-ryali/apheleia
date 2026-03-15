use apheleia_core::types::vector::Vector2;
use apheleia_ui::{
    KeyCode,
    contexts::{
        Context,
        commands::{MarkRenderDirty, RegisterForEvent, SetSizeForId},
    },
    node::{data::NodeData, node::NodeTrait},
    rootnode::rootnode::RootNode,
    types::{DirtyRenderLevel, EventData, EventType},
};

struct TestNode(bool);
impl NodeTrait for TestNode {
    fn initial_setup(&mut self, ctx: &mut Context) {
        ctx.add_command(Box::new(SetSizeForId(ctx.get_id(), Vector2(10, 1))));
        ctx.add_command(Box::new(RegisterForEvent(EventType::Keys)));
    }

    fn event(&mut self, ctx: &mut Context) {
        match ctx.get_event().as_ref().unwrap() {
            EventData::Keys(event) => {
                if event.code == KeyCode::Char('a') {
                    self.0 = true;

                    ctx.add_command(Box::new(MarkRenderDirty(
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

    fn update(&mut self, _ctx: &mut Context) {
        todo!()
    }
}

fn main() {
    let mut root = RootNode::default();

    root.create_node("parent_node")
        .set_position(Vector2(5, 5))
        .build();

    root.create_node("child_node")
        .set_parent("parent_node")
        .set_position(Vector2(10, 10))
        .node(Box::new(TestNode(false)))
        .build();

    // root.add_node(
    //     "test_node",
    //     "",
    //     Box::new(TestNode(false)),
    //     NodeData::new(Vector2(20, 3), None),
    // );

    root.run();
}
