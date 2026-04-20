use std::mem;

use apheleia_core::style::Style;
use apheleia_macros::Extension;
use apheleia_ui::{
    RichString, Vector2,
    contexts::{node::NodeContext, system::SystemContext},
    node::traits::NodeTrait,
};

use crate::label::{HorizontalAlignment, LabelNode};

#[derive(Extension, Clone)]
pub struct BorderStyle {
    pub horizontal: char,
    pub vertical: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,

    pub style: Style,
}
impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            horizontal: '─',
            vertical: '│',
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            style: Style::default(),
        }
    }
}
impl BorderStyle {
    pub fn boxed() -> Self {
        Self {
            horizontal: '─',
            vertical: '│',
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            style: Style::default(),
        }
    }

    pub fn rounded() -> Self {
        Self {
            horizontal: '─',
            vertical: '│',
            top_left: '╭',
            top_right: '╮',
            bottom_left: '╰',
            bottom_right: '╯',
            style: Style::default(),
        }
    }

    pub fn heavy() -> Self {
        Self {
            horizontal: '━',
            vertical: '┃',
            top_left: '┏',
            top_right: '┓',
            bottom_left: '┗',
            bottom_right: '┛',
            style: Style::default(),
        }
    }

    pub fn double() -> Self {
        Self {
            horizontal: '═',
            vertical: '║',
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
            style: Style::default(),
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

pub struct ContainerNode {
    pub border_style: Option<BorderStyle>,

    pub header_label: Option<LabelNode>,
    pub header_margin: u16,
    pub header_len: u16,

    pub footer_label: Option<LabelNode>,
    pub footer_margin: u16,
    pub footer_len: u16,
}
impl ContainerNode {
    pub fn border_style(mut self, border_style: Option<BorderStyle>) -> Self {
        self.border_style = border_style;
        self
    }

    pub fn set_header(mut self, margin: u16, len: u16, label: LabelNode) -> Self {
        self.header_margin = margin;
        self.header_len = len;
        self.header_label = Some(label);

        self
    }
    pub fn set_footer(mut self, margin: u16, len: u16, label: LabelNode) -> Self {
        self.footer_margin = margin;
        self.footer_len = len;
        self.footer_label = Some(label);

        self
    }
}
impl Default for ContainerNode {
    fn default() -> Self {
        Self {
            border_style: Some(BorderStyle::default()),

            header_margin: 1,
            header_len: 20,
            header_label: None,

            footer_margin: 1,
            footer_len: 20,
            footer_label: None,
        }
    }
}
impl NodeTrait for ContainerNode {
    fn initial_setup(&mut self, ctx: &mut apheleia_ui::contexts::node::NodeContext) {
        if let Some(border_style) = &mut self.border_style {
            ctx.add_extension(mem::take(border_style));
            ctx.add_system(
                apheleia_ui::types::UpdateType::Render,
                0,
                container_render_border,
            );
        }

        fn setup_label(
            ctx: &mut NodeContext,
            container_size: Vector2,
            label: &mut LabelNode,
            label_margin: u16,
            label_len: u16,
            y: u16,
        ) {
            let label_pos: u16;
            let label_size: u16;

            if label_len > (container_size.0 - 1) - (2 * label_margin) {
                label_pos = label_margin;
                label_size = (container_size.0 - 1) - (2 * label_margin);
            } else {
                label_size = label_len;
                match label.horizontal_alignment {
                    HorizontalAlignment::Left => {
                        label_pos = label_margin;
                    }
                    HorizontalAlignment::Center => {
                        label_pos = (container_size.0 / 2) - (label_len / 2) - 2;
                    }
                    HorizontalAlignment::Right => {
                        label_pos = (container_size.0 - 1) - label_margin - label_len;
                    }
                    HorizontalAlignment::Justify => {
                        label_pos = label_margin;
                    }
                };
            }

            let node = mem::replace(label, LabelNode::new(RichString::new("")));
            ctx.create_node(|builder| {
                builder
                    .set_position(Vector2(label_pos, y))
                    .set_size(Vector2(label_size, 1))
                    .node(node)
            });
        }

        let size = ctx.get_size().expect("No size given to container");
        if let Some(header_label) = &mut self.header_label {
            setup_label(
                ctx,
                size,
                header_label,
                self.header_margin,
                self.header_len,
                0,
            );
        }
        if let Some(footer_label) = &mut self.footer_label {
            setup_label(
                ctx,
                size,
                footer_label,
                self.footer_margin,
                self.footer_len,
                size.1 - 1,
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

fn container_render_border(ctx: &mut SystemContext) {
    let border_style = ctx.get_extension::<BorderStyle>().clone();
    let size = ctx.get_size().unwrap();
    let buffer = ctx.get_buffer();

    // TopLeft
    buffer.write_string(
        0,
        0,
        border_style.top_left.to_string(),
        Some(border_style.style),
    );
    // TopRight
    buffer.write_string(
        size.0 - 1,
        0,
        border_style.top_right.to_string(),
        Some(border_style.style),
    );
    // BottomLeft
    buffer.write_string(
        0,
        size.1 - 1,
        border_style.bottom_left.to_string(),
        Some(border_style.style),
    );
    // BottomRight
    buffer.write_string(
        size.0 - 1,
        size.1 - 1,
        border_style.bottom_right.to_string(),
        Some(border_style.style),
    );
    // Top
    buffer.write_string(
        1,
        0,
        border_style
            .horizontal
            .to_string()
            .repeat(size.0 as usize - 2),
        Some(border_style.style),
    );
    // Bottom
    buffer.write_string(
        1,
        size.1 - 1,
        border_style
            .horizontal
            .to_string()
            .repeat(size.0 as usize - 2),
        Some(border_style.style),
    );

    let mut vert_text = border_style.vertical.to_string();
    vert_text.push('\n');
    vert_text = vert_text.repeat(size.1 as usize - 2);
    // Left
    buffer.write_string(0, 1, vert_text.to_string(), Some(border_style.style));
    // Right
    buffer.write_string(size.0 - 1, 1, vert_text, Some(border_style.style));
}
