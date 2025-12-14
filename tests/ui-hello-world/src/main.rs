use apheleia_core::{buffer::Buffer, style::{Style, StyleFlags}, types::vector::Vector2};
use apheleia_ui::{
    commands::{InitialCallContext, IntialCallCommands::{self, RegisterUpdateType}},
    node::{
        data::{NodeData, NodeWrapper},
        node::NodeTrait,
    },
    rootnode::{RootNode, UpdateType},
};

struct BasicNode(pub bool);
impl NodeTrait for BasicNode {
    fn initial_setup(&mut self, ctx: &mut InitialCallContext) {
        ctx.add_command(IntialCallCommands::SetSize(Vector2(3, 1)));
        ctx.add_command(RegisterUpdateType(UpdateType::EVENT));
    }

    fn event(&mut self) {
        self.0 = true;
    }

    fn update(&mut self) {}

    fn render(&self, buf: &mut Buffer) {
        if self.0 {
            buf.write_line(
                0,
                0,
                "PRESSED A!",
                Some(Style {
                    flags: StyleFlags::UNDERLINED,
                    ..Default::default()
                }),
            );
        }
        buf.write_line(0, 0, "A", Some(Style::default()));
    }
}
fn main() {
    let mut root = RootNode::default();

    let node_data = NodeWrapper {
        data: NodeData {
            position: Some(Vector2(0, 0)),
            ..Default::default()
        },
        node: Box::new(BasicNode(false)),
    };

    root.add_node(node_data);

    root.initial_setup();
    root.run();
}
