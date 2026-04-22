use std::fmt::format;

use apheleia::{
    DirtyRenderLevel, EmptyNode, Extension, ExtensionMacro, KeyCode, Style, StyleFlags, Vector2,
    container::ContainerNode,
    label::{HorizontalAlignment, LabelExtension, LabelNode, ScrollingTextParams},
    rich_strings::RichString,
    root::Root,
    setup_logger,
    system::SystemContext,
};
use log::info;

struct CounterExtension {
    text: RichString,
    count: usize,
}
impl Extension for CounterExtension {}

fn main() {
    if cfg!(debug_assertions) {
        _ = setup_logger();
    }

    let mut root = Root::default();

    let width = root.width;
    let height = root.height;

    root.create_node(|builder| {
        builder
            .set_class("container")
            .set_position(Vector2(width / 4, 3 * (height / 8)))
            .set_size(Vector2(width / 2, height / 4))
            .node(
                ContainerNode::default().set_header(
                    2,
                    20,
                    LabelNode::new(RichString::to_rich(
                        "Simple Clicker App",
                        Style {
                            flags: StyleFlags::ITALIC,
                            ..Default::default()
                        },
                    ))
                    .set_horizontal_align(HorizontalAlignment::Center),
                ),
            )
    });
    root.create_node(|builder| {
        builder
            .set_class("count_label")
            .set_parent("container")
            .set_position(Vector2(2, 2))
            .set_size(Vector2(30, 10))
            .add_extension(Box::new(CounterExtension {
                text: RichString::new("Count: "),
                count: 0,
            }))
            .add_system(
                apheleia::UpdateType::Event(apheleia::EventType::Keys),
                100,
                handle_event_count,
            )
            .node(LabelNode::new(RichString::new("Count: ")).set_overflow(
                apheleia::label::TextOverflow::Scroll(ScrollingTextParams {
                    scroll_step: 0.25,
                    wait_step: 0.125,
                }),
            ))
    });

    root.run();
}

fn handle_event_count(ctx: &mut SystemContext) {
    if let apheleia::EventData::Keys(keys) = ctx.get_event_data()
        && keys.code == KeyCode::Enter
    {
        let count: usize;
        {
            let ext = ctx.get_extension_mut::<CounterExtension>();
            ext.count += 1;
            count = ext.count;
        }

        ctx.get_extension_mut::<LabelExtension>().text =
            RichString::new(&format!("Count: {}", count).to_string());
        ctx.mark_render_dirty(DirtyRenderLevel::SimpleDirty);
    };
}
