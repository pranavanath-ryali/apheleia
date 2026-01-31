use apheleia_core::{Color, style::{self, Style}};
use apheleia_ui::{
    contexts::Context,
    node::{data::NodeData, node::NodeTrait},
};

#[derive(Clone)]
pub struct BorderStyle {
    pub corners: [char; 4], // topleft, topright, bottomleft, bottomright
    pub horizontal: char,
    pub vertical: char,

    pub corners_style: Option<Style>,
    pub horizontal_style: Option<Style>,
    pub vertical_style: Option<Style>,
}
impl Default for BorderStyle {
    fn default() -> Self {
        BorderStyle {
            corners: ['╭', '╰', '╮', '╯'],
            horizontal: '─',
            vertical: '│',
            corners_style: None,
            horizontal_style: None,
            vertical_style: None,
        }
    }
}

pub struct Block {
    pub border_style: BorderStyle,
}
impl NodeTrait for Block {
    fn render(&self, buf: &mut apheleia_core::buffer::Buffer, ctx: &Context, data: &NodeData) {
        let size = data.size.unwrap();
        // TODO: Come back to this later
        let style = &self.border_style;
        for y in 1..(size.1 - 1) {
            buf.write_line(0, y, &style.vertical.to_string(), None);
            buf.write_line(size.0 - 1, y, &style.vertical.to_string(), None);
        }

        let horizontal_border = style.horizontal.to_string().repeat((size.0 - 2) as usize + 1);
        buf.write_line(1, 0, &horizontal_border, style.horizontal_style);
        buf.write_line(1, size.1 - 1, &horizontal_border, style.horizontal_style);
            
        buf.write_line(0, 0, &style.corners[0].to_string(), style.corners_style);
        buf.write_line(
            0,
            size.1 - 1,
            &style.corners[1].to_string(),
            style.corners_style,
        );
        buf.write_line(
            size.0 - 1,
            0,
            &style.corners[2].to_string(),
            style.corners_style,
        );
        buf.write_line(
            size.0 - 1,
            size.1 - 1,
            &style.corners[3].to_string(),
            style.corners_style,
        );
    }

    fn initial_setup(&mut self, ctx: &mut Context, data: &NodeData) {}

    fn event(&mut self, ctx: &mut Context, data: &NodeData) {}

    fn update(&mut self, ctx: &mut Context, data: &NodeData) {}
}
impl Block {
    pub fn new() -> Box<Self> {
        Box::new(Block {
            border_style: BorderStyle::default(),
        })
    }
}
