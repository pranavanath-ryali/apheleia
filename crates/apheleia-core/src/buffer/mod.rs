use crate::style::Style;

pub struct Line {
    pub text: String,
    pub position: (u16, u16),
    pub style: Style,
}

pub struct Buffer {
    pub width: u16,
    pub height: u16,
    line_buffer: Vec<Line>,
}

impl Buffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            line_buffer: vec![],
        }
    }

    pub fn new_fill(width: u16, height: u16, c: char) -> Self {
        Self {
            width,
            height,
            line_buffer: vec![],
        }
    }

    pub fn write_line(
        &mut self,
        start_pos_x: u16,
        start_pos_y: u16,
        text: &str,
        style: Option<Style>,
    ) {
        let mut t: String = "".to_string();
        for (i, c) in text.chars().enumerate() {
            if start_pos_x + (i as u16) >= self.width || start_pos_y >= self.height {
                continue;
            }
            t += &c.to_string();
        }
        self.line_buffer.push(Line {
            text: t,
            position: (start_pos_x, start_pos_y),
            style: style.unwrap_or_else(|| Style::default()),
        });
    }

    pub fn render_buffer(&mut self, start_pos_x: u16, start_pos_y: u16, buf: &mut Self) {
        for line in buf.get_update_list() {
            self.write_line(
                start_pos_x + line.position.0,
                start_pos_y + line.position.1,
                &line.text,
                Some(line.style),
            );
        }
        buf.clear_update_list();
    }

    pub fn get_update_list(&self) -> &Vec<Line> {
        &self.line_buffer
    }

    pub fn clear_update_list(&mut self) {
        self.line_buffer.clear();
    }
}
