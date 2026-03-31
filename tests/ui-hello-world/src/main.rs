use apheleia_core::types::vector::Vector2;
use apheleia_ui::{
    KeyCode,
    contexts::{
        Context,
        commands::{MarkRenderDirty, RegisterForEvent, SetSize},
    },
    extensions::Extension,
    node::{EmptyNode, NodeTrait},
    rootnode::RootNode,
    setup_logger,
    types::{DirtyRenderLevel, EventData, EventType},
};

struct TestNode(bool, String);
impl NodeTrait for TestNode {
    fn initial_setup(&mut self, ctx: &mut Context) {
        ctx.add_command(Box::new(SetSize(Vector2(10, 1))));
        ctx.add_command(Box::new(RegisterForEvent(EventType::Keys)));
    }

    fn event(&mut self, ctx: &mut Context) {
        let EventData::Keys(event) = ctx.get_event().as_ref().unwrap() else {
            return;
        };
        if event.code == KeyCode::Char('a') {
            self.0 = true;

            ctx.add_command(Box::new(MarkRenderDirty(
                ctx.get_id(),
                DirtyRenderLevel::SimpleDirty,
            )));
        }
    }

    fn render(&self, buf: &mut apheleia_core::buffer::Buffer, ctx: &mut Context) {
        if self.0 {
            buf.write_line(0, 0, self.1.as_str(), None);
        } else {
            buf.write_line(0, 0, "AAAAAA", None);
        }
    }

    fn update(&mut self, ctx: &mut Context) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

struct TestExt {
    test: bool,
}
impl Extension for TestExt {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn main() {
    if cfg!(debug_assertions) {
        _ = setup_logger();
    }
    let mut root = RootNode::default();

    root.create_node("parent_node")
        .set_position(Vector2(1, 0))
        .build(EmptyNode);

    root.create_node("child_node")
        .set_position(Vector2(1, 5))
        .build(TestNode(false, "Hello".to_string()));

    root.create_node("child")
        .set_parent("parent_node")
        .set_position(Vector2(2, 10))
        .build(TestNode(false, "!123124".to_string()));

    root.bind_extension_to_classes(
        vec!["child_node", "child"],
        Box::new(TestExt { test: false }),
    );

    root.run();
}
