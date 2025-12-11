use crate::style::Style;

// TODO: Refactor this later
#[derive(Clone)]
pub struct Cell {
    pub c: char,
    pub style: Style,
}

pub struct Buffer {
    pub width: u16,
    pub height: u16,
    cells: Vec<Vec<Cell>>,
    updated_cells: Vec<(u16, u16)>,
}

impl Buffer {
    pub fn new(width: u16, height: u16) -> Self {
        let default_cell = Cell {
            c: ' ',
            style: Style::default(),
        };

        Self {
            width,
            height,
            cells: vec![vec![default_cell; width as usize]; height as usize],
            updated_cells: vec![],
        }
    }

    pub fn new_fill(width: u16, height: u16, c: char) -> Self {
        let default_cell = Cell {
            c,
            style: Style::default(),
        };

        Self {
            width,
            height,
            cells: vec![vec![default_cell; width as usize]; height as usize],
            updated_cells: vec![],
        }
    }

    pub fn get(&self, x: u16, y: u16) -> &Cell {
        &self.cells[y as usize][x as usize]
    }

    fn set(&mut self, x: u16, y: u16, c: char, style: Style) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.cells[y as usize][x as usize].c = c;
        self.cells[y as usize][x as usize].style = style;
        self.updated_cells.push((x, y));
    }

    pub fn write_line(
        &mut self,
        start_pos_x: u16,
        start_pos_y: u16,
        text: &str,
        style: Option<Style>,
    ) {
        let s = style.unwrap_or_else(|| Style::default());
        for (i, c) in text.chars().enumerate() {
            self.set(start_pos_x + (i as u16), start_pos_y, c, s);
        }
    }

    pub fn render_node_buffer(&mut self, start_pos_x: u16, start_pos_y: u16, buf: &NodeBuffer) {
        for y in 0..buf.height{
            for x in 0..buf.width {
                let cell: &Cell = buf.get(x, y);
                self.set(start_pos_x + x, start_pos_y + y, cell.c, cell.style);
            }
        }
    }

    pub fn get_update_list(&self) -> Vec<(u16, u16)> {
        self.updated_cells.clone()
    }

    pub fn clear_update_list(&mut self) {
        self.updated_cells.clear();
    }
}

pub struct NodeBuffer {
    pub width: u16,
    pub height: u16,
    cells: Vec<Vec<Cell>>,
}
impl NodeBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        let default_cell = Cell {
            c: ' ',
            style: Style::default(),
        };

        Self {
            width,
            height,
            cells: vec![vec![default_cell; width as usize]; height as usize],
        }
    }
    pub fn get(&self, x: u16, y: u16) -> &Cell {
        &self.cells[y as usize][x as usize]
    }

    fn set(&mut self, x: u16, y: u16, c: char, style: Style) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.cells[y as usize][x as usize].c = c;
        self.cells[y as usize][x as usize].style = style;
    }

    pub fn write_line(
        &mut self,
        start_pos_x: u16,
        start_pos_y: u16,
        text: &str,
        style: Option<Style>,
    ) {
        let s = style.unwrap_or_else(|| Style::default());
        for (i, c) in text.chars().enumerate() {
            self.set(start_pos_x + (i as u16), start_pos_y, c, s);
        }
    }
}
