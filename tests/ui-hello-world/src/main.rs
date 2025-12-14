use apheleia_core::{
    buffer::Buffer,
    style::{Style, StyleFlags},
    types::vector::Vector2,
};
use apheleia_ui::{
    commands::{
        InitialCallContext,
        IntialCallCommands::{self, RegisterUpdateType},
    },
    node::{
        data::{NodeData, NodeWrapper},
        node::NodeTrait,
    },
    rootnode::{RootNode, UpdateType},
};

#[derive(Default)]
struct IDKWhatImDoingNode {
    i: u16,
}
impl NodeTrait for IDKWhatImDoingNode {
    fn initial_setup(&mut self, ctx: &mut InitialCallContext) {
        ctx.add_command(IntialCallCommands::SetSize(Vector2(3, 1)));
        ctx.add_command(RegisterUpdateType(UpdateType::Update));
    }

    fn event(&mut self) {}
    fn update(&mut self) {
        self.i += 1;
    }

    fn render(&self, buf: &mut Buffer) {
        buf.write_line(0, 0, &self.i.to_string(), Some(Style::default()));
    }
}

struct BasicNode(pub bool);
impl NodeTrait for BasicNode {
    fn initial_setup(&mut self, ctx: &mut InitialCallContext) {
        ctx.add_command(IntialCallCommands::SetSize(Vector2(3, 1)));
        ctx.add_command(RegisterUpdateType(UpdateType::Event));
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
        buf.write_line(0, 0, "A", Some(Style { fg: apheleia_core::Color::Red, flags: StyleFlags::BOLD | StyleFlags::ITALIC, ..Default::default() }));
    }
}
fn main() {
    let mut root = RootNode::default();

    let wid = root.add_node(NodeWrapper {
        data: NodeData {
            position: Vector2(10, 0),
            ..Default::default()
        },
        node: Box::new(BasicNode(false)),
    }, None);
    root.add_node(NodeWrapper {
        data: NodeData {
            position: Vector2(10, 10),
            ..Default::default()
        },
        node: Box::new(IDKWhatImDoingNode::default()),
    }, Some(wid.unwrap()));

    root.initial_setup();
    root.run();
}
