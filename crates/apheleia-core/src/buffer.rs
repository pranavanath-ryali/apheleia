use std::collections::HashMap;

use indexmap::IndexMap;

use crate::{rich_strings::RichString, style::Style};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub c: char,
    pub style: Style,
}
impl Default for Cell {
    fn default() -> Self {
        Self {
            c: ' ',
            style: Style::default(),
        }
    }
}

pub struct Buffer {
    size: (u16, u16),
    cells: Vec<Vec<Cell>>,
    diffed_cells: HashMap<u16, IndexMap<u16, Cell>>,
}
impl Buffer {
    pub fn new(width: u16, height: u16) -> Self {
        let mut cells: Vec<Vec<Cell>> = vec![];
        for _ in 0..height {
            let rows: Vec<Cell> = [Cell {
                c: ' ',
                style: Style::default(),
            }]
            .repeat(width as usize);
            cells.push(rows);
        }

        Self {
            size: (width, height),
            cells,
            diffed_cells: HashMap::new(),
        }
    }

    pub fn write_string(&mut self, x: u16, y: u16, text: String, style: Option<Style>) {
        let rich_string: RichString = match style {
            Some(style) => RichString::to_rich(&text, style),
            None => RichString::new(&text),
        };
        self.write_rich_string(x, y, rich_string);
    }

    pub fn write_rich_string(&mut self, x: u16, y: u16, rich_string: RichString) {
        for (offset, (c, style)) in rich_string.iter().enumerate() {
            if x as usize + offset > self.size.0 as usize - 1 {
                break;
            }
            self.cells[y as usize][x as usize + offset].c = c;
            self.cells[y as usize][x as usize + offset].style = style;
        }
    }

    pub fn render_buffer(&mut self, offset_x: u16, offset_y: u16, buf: &mut Buffer) {
        for (y, row) in buf.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == self.cells[y + offset_y as usize][x + offset_x as usize] {
                    continue;
                }

                if offset_x + x as u16 >= self.size.0 || offset_y + y as u16 >= self.size.1 {
                    continue;
                }

                self.cells[y + offset_y as usize][x + offset_x as usize] = *cell;
                self.diffed_cells
                    .entry(y as u16 + offset_y)
                    .or_default()
                    .insert(x as u16 + offset_x, *cell);
            }
        }
    }

    pub fn get_cell(&self, x: u16, y: u16) -> &Cell {
        &self.cells[y as usize][x as usize]
    }

    pub fn get_diffed_cells(&self) -> &HashMap<u16, IndexMap<u16, Cell>> {
        &self.diffed_cells
    }

    pub fn clear_diff(&mut self) {
        self.diffed_cells.clear();
    }
}

#[cfg(test)]
mod test_buffer {
    use crate::style::StyleFlags;

    use super::*;

    #[test]
    fn test_write_functions() {
        let mut buffer = Buffer::new(10, 1);
        buffer.write_rich_string(5, 0, RichString::new("He<bold>llo"));

        let mut text = "".to_string();
        for cell in buffer.cells[0].iter() {
            text.push(cell.c);
        }

        assert_eq!(
            buffer.cells[0],
            vec![
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell {
                    c: 'H',
                    style: Style {
                        ..Default::default()
                    }
                },
                Cell {
                    c: 'e',
                    style: Style {
                        ..Default::default()
                    }
                },
                Cell {
                    c: 'l',
                    style: Style {
                        flags: StyleFlags::BOLD,
                        ..Default::default()
                    }
                },
                Cell {
                    c: 'l',
                    style: Style {
                        flags: StyleFlags::BOLD,
                        ..Default::default()
                    }
                },
                Cell {
                    c: 'o',
                    style: Style {
                        flags: StyleFlags::BOLD,
                        ..Default::default()
                    }
                },
            ]
        );
    }
}
