use std::mem::take;

use apheleia_ui::{
    RichString, Vector2, contexts::system::SystemContext, extensions::traits::Extension,
    node::traits::NodeTrait,
};

#[derive(Clone, Copy)]
pub struct ScrollingTextParams {
    pub scroll_step: f32,
}

#[derive(Clone, Copy)]
pub enum TextOverflow {
    DoNothing,
    Ellipses(usize, char),
    Scroll(ScrollingTextParams),
}

#[derive(Clone, Copy)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}
#[derive(Clone, Copy)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

pub struct LabelExtension {
    pub text: RichString,

    pub overflow: TextOverflow,
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,

    pub scroll_i: usize,
    pub scroll_dir: isize,
    pub scroll_counter: f32,
}
impl Extension for LabelExtension {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct LabelNode {
    pub text: RichString,

    pub overflow: TextOverflow,
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,
}
impl LabelNode {
    pub fn new(text: RichString) -> Self {
        Self {
            text,

            overflow: TextOverflow::Ellipses(3, '.'),
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
        ctx.add_extension(LabelExtension {
            text: take(&mut self.text),
            overflow: self.overflow,
            horizontal_alignment: self.horizontal_alignment,
            vertical_alignment: self.vertical_alignment,

            scroll_i: 0,
            scroll_dir: 1,
            scroll_counter: 0.0,
        });
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
    let ext = ctx.get_extension_mut::<LabelExtension>();

    if let TextOverflow::Scroll(scroll_params) = ext.overflow
        && ext.text.len() >= size.0 as usize
    {
        ext.scroll_counter += scroll_params.scroll_step;
        if ext.scroll_counter >= 1.0 {
            ext.scroll_counter = 0.0;

            if ext.scroll_dir == 1 {
                ext.scroll_i += 1;
                if ext.scroll_i >= ext.text.len() - size.0 as usize {
                    ext.scroll_i = ext.text.len() - size.0 as usize;
                    ext.scroll_dir = -1;
                }
            } else if ext.scroll_i == 0 {
                ext.scroll_dir = 1;
                ext.scroll_i = 1;
            } else {
                ext.scroll_i -= 1;
            }
            ctx.mark_render_dirty(apheleia_ui::types::DirtyRenderLevel::SimpleDirty);
        }
    }
}

fn render(ctx: &mut SystemContext) {
    let mut text: RichString;
    let mut position = Vector2(0, 0);

    {
        let size = ctx.get_size().expect("A size is expected for Label");
        let ext = ctx.get_extension::<LabelExtension>();

        if ext.text.len() <= size.0 as usize {
            text = ext.text.clone();
            match ext.horizontal_alignment {
                HorizontalAlignment::Left => position.0 = 0,
                HorizontalAlignment::Center => {
                    position.0 = (size.0 / 2) - (ext.text.len() / 2) as u16
                }
                HorizontalAlignment::Right => position.0 = size.0 - ext.text.len() as u16,
            }
        } else {
            match ext.overflow {
                TextOverflow::DoNothing => {
                    // text = ext.text.to_string().split_at(size.0 as usize).0.to_string();
                    text = ext.text.slice(0, size.0 as usize);
                }
                TextOverflow::Ellipses(len, c) => {
                    text = ext.text.slice(0, size.0 as usize - len);
                    text.add_text(c.to_string().repeat(len).as_str(), None);
                    // text = ext
                    //     .text
                    //     .to_string()
                    //     .split_at(size.0 as usize - len)
                    //     .0
                    //     .to_string();
                    // text += c.to_string().repeat(len).as_str();
                }
                TextOverflow::Scroll(_) => {
                    // println!("\n\n\rI: {}", ext.scroll_i);
                    text = ext.text.slice(ext.scroll_i, ext.scroll_i + size.0 as usize);
                    // text = ext
                    //     .text
                    //     .to_string()
                    //     .split_at(ext.scroll_i)
                    //     .1
                    //     .split_at(size.0 as usize)
                    //     .0
                    //     .to_string();
                }
            }
        }
        match ext.vertical_alignment {
            VerticalAlignment::Top => position.1 = 0,
            VerticalAlignment::Center => position.1 = size.1 / 2,
            VerticalAlignment::Bottom => position.1 = size.1 - 1,
        }
    }

    ctx.get_buffer()
        .write_rich_string(position.0, position.1, text);
    // ctx.get_buffer()
    //     .write_string(position.0, position.1, text, None);
}
