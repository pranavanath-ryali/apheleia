use apheleia_core::{Color, style::Style};
use apheleia_ui::{contexts::Context, node::{data::NodeData, node::NodeTrait}};

#[derive(Clone)]
pub struct BorderStyle {
    pub corners: [char; 4], // topleft, topright, bottomleft, bottomright
    pub horizontal_pattern: String,
    pub vertical_pattern: String,

    pub corners_style: Option<Style>,
    pub horizontal_pattern_style: Option<Style>,
    pub veritcal_pattern_style: Option<Style>,
}
impl Default for BorderStyle {
    fn default() -> Self {
        BorderStyle {
            corners: ['+', '+', '+', '+'],
            horizontal_pattern: "-".to_string(),
            vertical_pattern: "|".to_string(),

            corners_style: None,
            horizontal_pattern_style: None,
            veritcal_pattern_style: None,
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
        let mut i: u8 = 0;
        for y in 1..(size.1 - 1) {
            i += 1;
            if i >= style.vertical_pattern.len() as u8 {
                i = 0;
            }
            buf.write_line(
                0,
                y,
                &style
                    .vertical_pattern
                    .chars()
                    .nth(i as usize)
                    .unwrap_or_else(|| ' ')
                    .to_string(),
                None,
            );
            buf.write_line(
                size.0 - 1,
                y,
                &style
                    .vertical_pattern
                    .chars()
                    .nth(i as usize)
                    .unwrap_or_else(|| ' ')
                    .to_string(),
                None,
            );
        }

        if style.horizontal_pattern.len() > 0 {
            let mut horizontal_border = style
                .horizontal_pattern
                .repeat((((size.0 - 2) as usize) / style.horizontal_pattern.len()) + 1);
            buf.write_line(1, 0, &horizontal_border, style.horizontal_pattern_style);
            buf.write_line(
                1,
                size.1 - 1,
                &horizontal_border,
                style.horizontal_pattern_style,
            );
        }

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

    fn initial_setup(&mut self, ctx: &mut Context, data: &NodeData) {
        todo!()
    }

    fn event(&mut self, ctx: &mut Context, data: &NodeData) {
        todo!()
    }

    fn update(&mut self, ctx: &mut Context, data: &NodeData) {
        todo!()
    }
}

impl Block {
    pub fn new() -> Box<Self> {
        Box::new(Block { border_style: BorderStyle::default() })
    }
}
