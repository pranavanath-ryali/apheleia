use apheleia_ui::{contexts::system::SystemContext, node::traits::NodeTrait, vector::Vector2};

#[derive(Clone, Copy)]
pub struct ScrollingTextParams;

pub enum TextOverflow {
    DoNothing,
    Ellipses(usize, char),
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

    scroll_i: usize,
    scroll_dir: isize,
    scroll_counter: f32,
    scroll_counter_step: f32,
}
impl LabelNode {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),

            overflow: TextOverflow::Ellipses(3, '.'),
            horizontal_alignment: HorizontalAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,

            scroll_i: 0,
            scroll_dir: 1,
            scroll_counter: 0.0,
            scroll_counter_step: 1.0,
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
        if let TextOverflow::Scroll(_) = self.overflow {
            ctx.add_system(
                apheleia_ui::types::UpdateType::ConstantUpdate,
                0,
                scroll_update,
            );
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn scroll_update(ctx: &mut SystemContext) {
    let size = ctx.get_size().expect("A size is expected for Label");
    let node = ctx.get_node_mut::<LabelNode>();

    if let TextOverflow::Scroll(scroll_params) = node.overflow {
        node.scroll_counter += node.scroll_counter_step;
        if node.scroll_counter >= 1.0 {
            node.scroll_counter = 0.0;

            if node.scroll_dir == 1 {
                node.scroll_i += 1;
                if node.scroll_i >= node.text.len() - size.0 as usize {
                    node.scroll_i = node.text.len() - size.0 as usize;
                    node.scroll_dir = -1;
                }
            } else if node.scroll_i == 0 {
                node.scroll_dir = 1;
                node.scroll_i = 1;
            } else {
                node.scroll_i -= 1;
            }
        }
        ctx.mark_render_dirty();
    }
}

fn render(ctx: &mut SystemContext) {
    let mut text: String;
    let mut position = Vector2(0, 0);

    {
        let size = ctx.get_size().expect("A size is expected for Label");
        let node = ctx.get_node::<LabelNode>();

        if node.text.len() <= size.0 as usize {
            text = node.text.to_string();
            match node.horizontal_alignment {
                HorizontalAlignment::Left => position.0 = 0,
                HorizontalAlignment::Center => {
                    position.0 = (size.0 / 2) - (node.text.len() / 2) as u16
                }
                HorizontalAlignment::Right => position.0 = size.0 - node.text.len() as u16,
            }
        } else {
            match node.overflow {
                TextOverflow::DoNothing => {
                    text = node
                        .text
                        .to_string()
                        .split_at(size.0 as usize)
                        .0
                        .to_string();
                }
                TextOverflow::Ellipses(len, c) => {
                    text = node
                        .text
                        .to_string()
                        .split_at(size.0 as usize - len)
                        .0
                        .to_string();
                    text += c.to_string().repeat(len).as_str();
                }
                TextOverflow::Scroll(_) => {
                    text = node
                        .text
                        .to_string()
                        .split_at(node.scroll_i)
                        .1
                        .split_at(size.0 as usize)
                        .0
                        .to_string();
                }
            }
        }
        match node.vertical_alignment {
            VerticalAlignment::Top => position.1 = 0,
            VerticalAlignment::Center => position.1 = size.1 / 2,
            VerticalAlignment::Bottom => position.1 = size.1 - 1,
        }
    }

    ctx.get_buffer()
        .write_line(position.0, position.1, text.as_str(), None);
}
