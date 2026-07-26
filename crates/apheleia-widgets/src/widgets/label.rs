
use apheleia_app::{node_definer::NodeDefiner, params::query_filters::OnRender};
use apheleia_core::{node_buffer::NodeBuffer, rich_strings::RichString, types::Vec2};
use apheleia_ecs::{constants::STAGE, params::query::Query};

use crate::extensions::label::{HorizontalAlignment, LabelExtension, VerticalAlignment};

#[derive(Debug)]
pub struct LabelWidget {
    pub ext: LabelExtension,
}
impl LabelWidget {
    pub fn new(text: &str) -> Self {
        Self {
            ext: LabelExtension {
                text: RichString::new(text),
                ..Default::default()
            },
        }
    }

    pub fn horizontal_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.ext.horizontal_alignment = alignment;
        self
    }

    pub fn vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.ext.vertical_alignment = alignment;
        self
    }
}
impl NodeDefiner for LabelWidget {
    fn setup(self: Box<Self>, ctx: &mut apheleia_app::context::node::NodeContext) {
        ctx.add_extension(self.ext, None);
        ctx.add_system(
            apheleia_ecs::types::SystemRunStage::Render,
            STAGE,
            render_label,
        );
    }
}

pub fn render_label(query: Query<(&LabelExtension, NodeBuffer), OnRender>) {
    for (ext, buffer) in query.iter() {
        let mut position = Vec2::zero();

        position.x = match ext.horizontal_alignment {
            HorizontalAlignment::Left => 0,
            HorizontalAlignment::Center => (buffer.size.x / 2) - (ext.text.len() / 2) as u32,
            HorizontalAlignment::Right => buffer.size.x - ext.text.len() as u32,
        };
        position.y = match ext.vertical_alignment {
            VerticalAlignment::Top => 0,
            VerticalAlignment::Center => buffer.size.y / 2,
            VerticalAlignment::Bottom => buffer.size.y - 1,
        };

        buffer.write_rich_string(position, &ext.text);
    }
}
