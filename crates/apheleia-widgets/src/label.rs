use apheleia_ui::{contexts::system::SystemContext, node::traits::NodeTrait};

pub struct ScrollingTextParams;

pub enum TextOverflow {
    DoNothing,
    Ellipses,
    Scroll(ScrollingTextParams),
}

pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

pub struct LabelNode {
    pub text: String,

    pub overflow: TextOverflow,
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,
}
impl LabelNode {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),

            overflow: TextOverflow::Ellipses,
            horizontal_alignment: HorizontalAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
        }
    }

    pub fn set_overflow(mut self, overflow: TextOverflow) -> Self {
        self.overflow = overflow;
        self
    }

    pub fn set_horizontal_align(mut self, alignment: HorizontalAlignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    pub fn set_vertical_align(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }
}
impl NodeTrait for LabelNode {
    fn initial_setup(&mut self, ctx: &mut apheleia_ui::contexts::node::NodeContext) {
        ctx.add_system(apheleia_ui::types::UpdateType::Render, 0, render);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn render(ctx: &mut SystemContext) {}
