// use apheleia_core::types::Vector2;
// use apheleia_ui::{
//     contexts::{node::NodeContext, system::SystemContext},
//     extensions::traits::Extension,
//     node::traits::NodeTrait,
//     resources::traits::Resource,
//     root::Root,
//     setup_logger,
// };
// use apheleia_widgets::label::{self, LabelNode, ScrollingTextParams};

// fn test_render(ctx: &mut SystemContext) {
//     let id = ctx.get_id();

//     println!("YAY");

//     // ctx.get_buffer()
//     //     .write_line(0, 0, format!("ID: {}", id).as_str(), None);
// }

// struct TestNode(bool, String);
// impl NodeTrait for TestNode {
//     fn initial_setup(&mut self, ctx: &mut NodeContext) {
//         // ctx.add_command(Box::new(SetSize(Vector2(10, 1))));
//         // ctx.add_command(Box::new(RegisterForEvent(EventType::Keys)));

//         // ctx.add_system(apheleia_ui::types::UpdateTypeNode::Render, 0, test_render);

//         // ctx.add_system(
//         //     apheleia_ui::types::UpdateType::Event(EventType::Resize),
//         //     0,
//         //     test_render,
//         // );

//         let res = ctx.get_resource_mut::<TestRes>().unwrap();
//         println!("MESSAGE: {}", res.message);
//         res.message = "HELLO AGAIN".to_string();
//         println!("MESSAGE: {}", res.message);
//     }

//     // fn event(&mut self, ctx: &mut Context) {
//     //     let EventData::Keys(event) = ctx.get_event().as_ref().unwrap() else {
//     //         return;
//     //     };
//     //     if event.code == KeyCode::Char('a') {
//     //         self.0 = true;

//     //         ctx.add_command(Box::new(MarkRenderDirty(
//     //             ctx.get_id(),
//     //             DirtyRenderLevel::SimpleDirty,
//     //         )));
//     //     }
//     // }

//     // fn render(&self, buf: &mut apheleia_core::buffer::Buffer, ctx: &mut Context) {
//     //     if self.0 {
//     //         buf.write_line(0, 0, self.1.as_str(), None);
//     //     } else {
//     //         buf.write_line(0, 0, "AAAAAA", None);
//     //     }
//     // }

//     // fn update(&mut self, ctx: &mut Context) {}

//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }
//     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
//         self
//     }
// }

// struct TestExt {
//     test: bool,
// }
// impl Extension for TestExt {
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }
// }

// struct TestRes {
//     message: String,
// }
// impl Resource for TestRes {
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }
// }

// fn main() {
//     if cfg!(debug_assertions) {
//         _ = setup_logger();
//     }
//     let mut root = Root::default();

//     root.create_node(|builder| {});

//     // root.add_resource(TestRes {
//     //     message: "HELLO".to_string(),
//     // });

//     // root.create_node("parent_node")
//     //     .set_position(Vector2(1, 0))
//     //     // .build(EmptyNode);
//     //     .build(TestNode(false, "Hello".to_string()));

//     // root.create_node("label").set_size(Vector2(20, 5)).build(
//     //     LabelNode::new("Hello Surya Bitch. YOU FUCKING IDIOT")
//     //         .set_overflow(label::TextOverflow::Scroll(ScrollingTextParams))
//     //         .set_horizontal_align(label::HorizontalAlignment::Left)
//     //         .set_vertical_align(label::VerticalAlignment::Top),
//     // );

//     // root.create_node("child_node")
//     //     .set_position(Vector2(1, 5))
//     //     .build(TestNode(false, "Hello".to_string()));

//     // root.create_node("child")
//     //     .set_parent("parent_node")
//     //     .set_position(Vector2(2, 10))
//     //     .build(TestNode(false, "!123124".to_string()));

//     // root.bind_extension_to_classes(
//     //     vec!["child_node", "child"],
//     //     Box::new(TestExt { test: false }),
//     // );

//     root.run();
// }

use apheleia_ui::{
    RichString, Vector2, contexts::system::SystemContext, root::Root, setup_logger,
    types::UpdateType,
};
use apheleia_widgets::label::{LabelNode, ScrollingTextParams};

fn main() {
    // if cfg!(debug_assertions) {
    //     _ = setup_logger();
    // }

    let mut root = Root::default();

    root.create_node(|builder| {
        builder
            .set_class("scrolling_text")
            .set_size(Vector2(7, 1))
            .node(
                // LabelNode::new(RichString::new("</fg:blue/>0123456</fg:red/>7890123456789"))
                LabelNode::new(RichString::new(
                    "</fg:blue/>0123456</reverse/>7890123456789",
                ))
                .set_overflow(apheleia_widgets::label::TextOverflow::Scroll(
                    ScrollingTextParams {
                        scroll_step: 0.25,
                        wait_step: 0.125,
                    },
                )),
            )
        // .node(LabelNode::new("Hello World. <fg:red>This Text is <under_lined>RED. <fg:blue>This Text is <slow_blink>BLUE").set_overflow(
        //     apheleia_widgets::label::TextOverflow::Scroll(
        //         apheleia_widgets::label::ScrollingTextParams { scroll_step: 0.5 },
        //     ),
        // ))
    });

    root.run();
}
