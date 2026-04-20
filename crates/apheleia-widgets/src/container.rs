use std::mem;

use apheleia_core::style::Style;
use apheleia_macros::Extension;
use apheleia_ui::{RichString, contexts::system::SystemContext, node::traits::NodeTrait};

use crate::label::HorizontalAlignment;

#[derive(Clone)]
pub struct BorderStyle {
    pub horizontal: char,
    pub vertical: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,

    pub style: Style,
}

#[derive(Extension, Clone)]
pub struct ContainerExtension {
    pub border_style: Option<BorderStyle>,

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
            border_style: Some(BorderStyle {
                horizontal: '─',
                vertical: '│',
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                style: Style::default(),
            }),

            header_text: None,
            header_margin: 0,
            header_text_alignment: HorizontalAlignment::Center,

            footer_text: None,
            footer_margin: 0,
            footer_text_alignment: HorizontalAlignment::Center,
        }
    }
}

#[derive(Default)]
pub struct ContainerNode {
    pub extension: ContainerExtension,
}
impl NodeTrait for ContainerNode {
    fn initial_setup(&mut self, ctx: &mut apheleia_ui::contexts::node::NodeContext) {
        ctx.add_extension(mem::take(&mut self.extension));
        ctx.add_system(
            apheleia_ui::types::UpdateType::Render,
            0,
            container_render_border,
        );
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn container_render_border(ctx: &mut SystemContext) {
    let border_style = match &ctx.get_extension::<ContainerExtension>().border_style {
        Some(style) => style.clone(),
        None => {
            return;
        }
    };

    let size = ctx.get_size().unwrap();
    let buffer = ctx.get_buffer();

    buffer.write_string(
        0,
        0,
        border_style.top_left.to_string(),
        Some(border_style.style),
    );
    buffer.write_string(
        size.0 - 1,
        0,
        border_style.top_right.to_string(),
        Some(border_style.style),
    );
    buffer.write_string(
        0,
        size.1 - 1,
        border_style.bottom_left.to_string(),
        Some(border_style.style),
    );
    buffer.write_string(
        size.0 - 1,
        size.1 - 1,
        border_style.bottom_right.to_string(),
        Some(border_style.style),
    );
    buffer.write_string(
        1,
        0,
        border_style
            .horizontal
            .to_string()
            .repeat(size.0 as usize - 2),
        Some(border_style.style),
    );
    buffer.write_string(
        1,
        size.1 - 1,
        border_style
            .horizontal
            .to_string()
            .repeat(size.0 as usize - 2),
        Some(border_style.style),
    );
}
