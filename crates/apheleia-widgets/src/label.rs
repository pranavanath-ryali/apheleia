use apheleia_app::{context::system::SystemContext, node_definer::NodeDefiner};
use apheleia_core::{rich_strings::RichString, types::Vec2};
use apheleia_ecs::{
    constants::FIRST, events::RenderDirty, extensions::Extension, system_params::query::{Query, query_filter::WithEvent}, types::{NodeId, SystemRunStage}
};

#[derive(Default, Debug)]
pub enum TextDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Default, Debug)]
pub enum HorizontalAlignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Default, Debug)]
pub enum VerticalAlignment {
    #[default]
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Default)]
pub struct LabelExtension {
    pub text: RichString,
    render_text: RichString,

    direction: TextDirection,

    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,
}
impl Extension for LabelExtension {}

#[derive(Debug)]
pub struct LabelWidget {
    label_ext: LabelExtension,
}
impl LabelWidget {
    pub fn new(text: RichString) -> Self {
        Self {
            label_ext: LabelExtension {
                text,
                ..Default::default()
            },
        }
    }

    pub fn horizontal_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.label_ext.horizontal_alignment = alignment;
        self
    }
    pub fn vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.label_ext.vertical_alignment = alignment;
        self
    }
}
impl NodeDefiner for LabelWidget {
    fn setup(self: Box<Self>, ctx: &mut apheleia_app::context::node::NodeContext) {
        ctx.add_extension(self.label_ext);
        ctx.add_system(SystemRunStage::Render, FIRST, render_label);
    }
}

fn render_label(query: Query<(NodeId, &LabelExtension), WithEvent<RenderDirty>>, mut ctx: SystemContext) {
    for (id, label_ext) in query.iter() {
        let buffer = ctx.get_buffer(id).unwrap();

        let mut position = Vec2::zero();

        position.x = match label_ext.horizontal_alignment {
            HorizontalAlignment::Left => 0,
            HorizontalAlignment::Center => (buffer.size.x / 2) - (label_ext.text.len() / 2) as u16,
            HorizontalAlignment::Right => buffer.size.x - label_ext.text.len() as u16,
        };
        position.y = match label_ext.vertical_alignment {
            VerticalAlignment::Top => 0,
            VerticalAlignment::Center => buffer.size.y / 2,
            VerticalAlignment::Bottom => buffer.size.y - 1,
        };

        buffer.write_rich_string(position, label_ext.text.clone());
    }
}
