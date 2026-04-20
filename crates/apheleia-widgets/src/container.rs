use std::mem;

use apheleia_core::style::Style;
use apheleia_macros::Extension;
use apheleia_ui::{RichString, Vector2, contexts::system::SystemContext, node::traits::NodeTrait};

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

#[derive(Extension, Clone)]
pub struct ContainerExtension {
    pub header_text: Option<RichString>,
    pub header_margin: u16,
    pub header_text_alignment: HorizontalAlignment,

    pub footer_text: Option<RichString>,
    pub footer_margin: u16,
    pub footer_text_alignment: HorizontalAlignment,
}
impl Default for ContainerExtension {
    fn default() -> Self {
        Self {
            header_text: None,
            header_margin: 0,
            header_text_alignment: HorizontalAlignment::Center,

            footer_text: None,
            footer_margin: 0,
            footer_text_alignment: HorizontalAlignment::Center,
        }
    }
}

pub struct ContainerNode {
    pub border_style: Option<BorderStyle>,
    pub container: Option<ContainerExtension>,
}
impl ContainerNode {
    pub fn set_header(
        mut self,
        text: RichString,
        margin: u16,
        alignment: HorizontalAlignment,
    ) -> Self {
        let mut container = self.container.unwrap_or_default();
        container.header_text = Some(text);
        container.header_margin = margin;
        container.header_text_alignment = alignment;

        self.container = mem::take(&mut Some(container));

        self
    }
    pub fn set_footer(
        mut self,
        text: RichString,
        margin: u16,
        alignment: HorizontalAlignment,
    ) -> Self {
        let mut container = self.container.unwrap_or_default();
        container.footer_text = Some(text);
        container.footer_margin = margin;
        container.footer_text_alignment = alignment;

        self.container = mem::take(&mut Some(container));

        self
    }
}
impl Default for ContainerNode {
    fn default() -> Self {
        Self {
            border_style: Some(BorderStyle::default()),
            container: None,
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
        if let Some(container) = &mut self.container {
            let size = ctx.get_size().expect("No size given to container");
            if let Some(header_text) = &mut container.header_text {
                ctx.create_node(|builder| {
                    builder
                        .set_position(Vector2(container.header_margin, 0))
                        .set_size(Vector2(size.0 - (container.header_margin * 2), 1))
                        .node(
                            LabelNode::new(mem::take(header_text))
                                .set_horizontal_align(container.header_text_alignment),
                        )
                });
            }
            if let Some(footer_text) = &mut container.footer_text {
                ctx.create_node(|builder| {
                    builder
                        .set_position(Vector2(container.footer_margin, size.1 - 1))
                        .set_size(Vector2(size.0 - (container.footer_margin * 2), 1))
                        .node(
                            LabelNode::new(mem::take(footer_text))
                                .set_horizontal_align(container.footer_text_alignment),
                        )
                });
            }
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
