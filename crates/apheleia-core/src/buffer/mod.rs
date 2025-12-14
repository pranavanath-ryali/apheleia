use crate::style::Style;

// TODO: Refactor this later
#[derive(Clone)]
pub struct Cell {
    pub ch: char,
    pub style: Style,
}

pub struct Line {
    pub text: String,
    pub position: (u16, u16),
    pub style: Style,
}

pub struct Buffer {
    pub width: u16,
    pub height: u16,
    cells: Vec<Vec<Cell>>,
    line_buffer: Vec<Line>,
}

impl Buffer {
    pub fn new(width: u16, height: u16) -> Self {
        let default_cell = Cell {
            ch: ' ',
            style: Style::default(),
        };

        Self {
            width,
            height,
            cells: vec![vec![default_cell; width as usize]; height as usize],
            line_buffer: vec![],
        }
    }

    pub fn new_fill(width: u16, height: u16, c: char) -> Self {
        let default_cell = Cell {
            ch: c,
            style: Style::default(),
        };

        Self {
            width,
            height,
            cells: vec![vec![default_cell; width as usize]; height as usize],
            line_buffer: vec![],
        }
    }

    pub fn get(&self, x: u16, y: u16) -> &Cell {
        &self.cells[y as usize][x as usize]
    }

    fn set(&mut self, x: u16, y: u16, c: char, style: Style) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        self.cells[y as usize][x as usize].ch = c;
        self.cells[y as usize][x as usize].style = style;

        true
    }

    pub fn write_line(
        &mut self,
        start_pos_x: u16,
        start_pos_y: u16,
        text: &str,
        style: Option<Style>,
    ) {
        let s = style.unwrap_or_else(|| Style::default());
        let mut t: String = "".to_string();
        for (i, c) in text.chars().enumerate() {
            let is_set = self.set(start_pos_x + (i as u16), start_pos_y, c, s);

            if !is_set {
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
            self.write_line(start_pos_x + line.position.0, start_pos_y + line.position.1, &line.text, Some(line.style));
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
