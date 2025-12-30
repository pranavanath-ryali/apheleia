use apheleia_core::{
    buffer::Buffer,
    style::{Style, StyleFlags},
    types::vector::Vector2,
};
use apheleia_ui::{
    KeyCode, commands::{
        InitialCallContext,
        IntialCallCommands::{self},
    }, contexts::{RenderContext, UpdateContext}, node::{
        data::{NodeData, NodeWrapper},
        node::NodeTrait,
    }, rootnode::{self, EventType, RootNode}
};

#[derive(Default)]
struct IDKWhatImDoingNode {
    i: u16,
}
impl NodeTrait for IDKWhatImDoingNode {
    fn initial_setup(&mut self, ctx: &mut InitialCallContext) {
        ctx.add_command(IntialCallCommands::SetSize(Vector2(3, 1)));
        ctx.add_command(IntialCallCommands::RegisterUpdate);
    }

    fn event(&mut self, ctx: &apheleia_ui::contexts::EventContext) {}

    fn update(&mut self, ctx: &mut UpdateContext) {
        self.i += 1;
    }

    fn render(&self, ctx: &mut RenderContext, buf: &mut Buffer) {
        buf.write_line(0, 0, &self.i.to_string(), Some(Style::default()));
    }
}

struct BasicNode(pub bool);
impl NodeTrait for BasicNode {
    fn initial_setup(&mut self, ctx: &mut InitialCallContext) {
        ctx.add_command(IntialCallCommands::SetSize(Vector2(10, 10)));
        ctx.add_command(IntialCallCommands::RegisterEvent(EventType::Keys));
    }

    fn event(&mut self, ctx: &apheleia_ui::contexts::EventContext) {
        match ctx.data {
            apheleia_ui::contexts::EventData::Keys(code) => {
                if code == KeyCode::Char('a') {
                    self.0 = true;
                }
            },
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateContext) {}

    fn render(&self, ctx: &mut RenderContext, buf: &mut Buffer) {
        buf.write_line(
            0,
            0,
            "AAAAAAAAAA",
            Some(Style {
                fg: apheleia_core::Color::Blue,
                ..Default::default()
            }),
        );
        buf.write_line(
            0,
            1,
            "BBBBBBBBBB",
            Some(Style {
                fg: apheleia_core::Color::Blue,
                ..Default::default()
            }),
        );

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
    }
}
fn main() {
    let mut root = RootNode::default();

    let wid = root.add_node(
        NodeWrapper {
            data: NodeData {
                position: Vector2(10, 2),
                ..Default::default()
            },
            node: Box::new(BasicNode(false)),
        },
        None,
    );
    root.add_node(
        NodeWrapper {
            data: NodeData {
                position: Vector2(0, 3),
                ..Default::default()
            },
            node: Box::new(IDKWhatImDoingNode::default()),
        },
        Some(wid.unwrap()),
    );

    root.initial_setup();
    root.run();
}
