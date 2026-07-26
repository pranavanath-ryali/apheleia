use apheleia_app::{builder, node_definer::NodeDefiner, params::query_filters::OnRender};
use apheleia_core::{Color, node_buffer::NodeBuffer, style::Style, types::Vec2};
use apheleia_ecs::{
    constants::{FIRST, STAGE},
    params::query::Query,
    runtime_expressions::{Constant, Expr, ExprVec, Expression, values::ParentWidth},
    types::SystemRunStage,
};

use crate::{
    extensions::{background::BackgroundExtension, container::BorderExtension},
    widgets::label::LabelWidget,
};

#[derive(Debug)]
pub struct ContainerWidget {
    bg_color: Option<Color>,

    border_ext: Option<BorderExtension>,
    border_style: Style,

    header_label_ext: Option<LabelWidget>,
    header_margin: u16,
}
impl ContainerWidget {
    pub fn new() -> Self {
        Self {
            bg_color: None,

            border_ext: Some(BorderExtension::default()),
            border_style: Style::default(),

            header_label_ext: None,
            header_margin: 1,
        }
    }

    pub fn background(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    pub fn header(mut self, label: LabelWidget) -> Self {
        self.header_label_ext = Some(label);
        self
    }
    pub fn header_margin(mut self, margin: u16) -> Self {
        self.header_margin = margin;
        self
    }

    pub fn none(mut self) -> Self {
        self.border_ext = None;
        self
    }

    pub fn boxed(mut self) -> Self {
        self.border_ext = Some(BorderExtension::boxed());
        self
    }

    pub fn rounded(mut self) -> Self {
        self.border_ext = Some(BorderExtension::rounded());
        self
    }

    pub fn heavy(mut self) -> Self {
        self.border_ext = Some(BorderExtension::heavy());
        self
    }

    pub fn double(mut self) -> Self {
        self.border_ext = Some(BorderExtension::double());
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }
}

impl NodeDefiner for ContainerWidget {
    fn setup(self: Box<Self>, ctx: &mut apheleia_app::context::node::NodeContext) {
        if let Some(color) = self.bg_color {
            ctx.add_extension(BackgroundExtension { color }, None);
            ctx.add_system(SystemRunStage::Render, FIRST, render_background);
        }

        if let Some(mut border_ext) = self.border_ext {
            border_ext.style = self.border_style;
            ctx.add_extension(border_ext, None);
            ctx.add_system(SystemRunStage::Render, STAGE, render_border);
        }

        if let Some(header_label) = self.header_label_ext {
            ctx.create_node(|builder| {
                builder
                    .position(ExprVec {
                        x: Expression(Expr::Value(Box::new(Constant(self.header_margin as u32)))),
                        y: Expression(Expr::Value(Box::new(Constant(0)))),
                    })
                    .size(ExprVec {
                        x: Expression(Expr::Sub(
                            Box::new(Expr::Value(Box::new(ParentWidth))),
                            Box::new(Expr::Multiply(
                                Box::new(Expr::Value(Box::new(Constant(
                                    self.header_margin as u32,
                                )))),
                                Box::new(Expr::Value(Box::new(Constant(2)))),
                            )),
                        )),
                        y: Expression(Expr::Value(Box::new(Constant(1)))),
                    })
                    .node(header_label)
            });
        }
    }
}

pub fn render_background(query: Query<(&BackgroundExtension, NodeBuffer), OnRender>) {
    for (bg_ext, buffer) in query.iter() {
        let color = bg_ext.color;
        for y in 0..buffer.size.y {
            buffer.write_string(
                Vec2 { x: 0, y },
                &(" ".repeat(buffer.size.y as usize)),
                Some(Style {
                    bg: color,
                    ..Default::default()
                }),
            );
        }
    }
}

pub fn render_border(query: Query<(&BorderExtension, NodeBuffer), OnRender>) {
    for (border_style, buffer) in query.iter() {
        let size = buffer.size;

        // TopLeft
        buffer.write_string(
            Vec2::zero(),
            &border_style.top_left.to_string(),
            Some(border_style.style),
        );
        // TopRight
        buffer.write_string(
            Vec2 {
                x: size.x - 1,
                y: 0,
            },
            &border_style.top_right.to_string(),
            Some(border_style.style),
        );
        // BottomLeft
        buffer.write_string(
            Vec2 {
                x: 0,
                y: size.y - 1,
            },
            &border_style.bottom_left.to_string(),
            Some(border_style.style),
        );
        // BottomRight
        buffer.write_string(
            Vec2 {
                x: size.x - 1,
                y: size.y - 1,
            },
            &border_style.bottom_right.to_string(),
            Some(border_style.style),
        );
        // Top
        buffer.write_string(
            Vec2 { x: 1, y: 0 },
            &border_style
                .horizontal
                .to_string()
                .repeat(size.x as usize - 2),
            Some(border_style.style),
        );
        // Bottom
        buffer.write_string(
            Vec2 {
                x: 1,
                y: size.y - 1,
            },
            &border_style
                .horizontal
                .to_string()
                .repeat(size.x as usize - 2),
            Some(border_style.style),
        );

        let mut vert_text = border_style.vertical.to_string();
        vert_text.push('\n');
        vert_text = vert_text.repeat(size.y as usize - 2);
        // Left
        buffer.write_string(
            Vec2 { x: 0, y: 1 },
            &vert_text.to_string(),
            Some(border_style.style),
        );
        // Right
        buffer.write_string(
            Vec2 {
                x: size.x - 1,
                y: 1,
            },
            &vert_text,
            Some(border_style.style),
        );
    }
}
