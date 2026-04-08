use crate::{rich_strings::RichString, style::Style};

// pub struct Line {
//     pub text: String,
//     pub position: (u16, u16),
//     pub style: Style,
// }

// pub struct Buffer {
//     pub width: u16,
//     pub height: u16,
//     line_buffer: Vec<Line>,
// }

// impl Buffer {
//     pub fn new(width: u16, height: u16) -> Self {
//         Self {
//             width,
//             height,
//             line_buffer: vec![],
//         }
//     }

//     pub fn new_fill(width: u16, height: u16, c: char) -> Self {
//         Self {
//             width,
//             height,
//             line_buffer: vec![],
//         }
//     }

//     pub fn write_string(
//         &mut self,
//         start_pos_x: u16,
//         start_pos_y: u16,
//         text: &str,
//         style: Option<Style>,
//     ) {
//         let mut t: String = "".to_string();
//         for (i, c) in text.chars().enumerate() {
//             if start_pos_x + (i as u16) >= self.width || start_pos_y >= self.height {
//                 continue;
//             }
//             t += &c.to_string();
//         }
//         self.line_buffer.push(Line {
//             text: t,
//             position: (start_pos_x, start_pos_y),
//             style: style.unwrap_or_else(|| Style::default()),
//         });
//     }

//     pub fn render_buffer(&mut self, start_pos_x: u16, start_pos_y: u16, buf: &mut Self) {
//         for line in buf.get_update_list() {
//             self.write_string(
//                 start_pos_x + line.position.0,
//                 start_pos_y + line.position.1,
//                 &line.text,
//                 Some(line.style),
//             );
//         }
//         buf.clear_update_list();
//     }

//     pub fn get_update_list(&self) -> &Vec<Line> {
//         &self.line_buffer
//     }

//     pub fn clear_update_list(&mut self) {
//         self.line_buffer.clear();
//     }
// }

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
    diffed_cells: Vec<(u16, u16)>,
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
            diffed_cells: vec![],
        }
    }

    pub fn write_string(&mut self, x: u16, y: u16, text: String, style: Style) {
        let rich_string = RichString::to_rich(&text, style);
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

    pub fn render_buffer(&mut self, buf: &mut Buffer) {
        for (y, row) in buf.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == self.cells[y][x] {
                    continue;
                }

                self.cells[y][x] = *cell;
                self.diffed_cells.push((x as u16, y as u16));
            }
        }
    }

    pub fn get_cell(&self, x: u16, y: u16) -> &Cell {
        &self.cells[y as usize][x as usize]
    }

    pub fn get_diffed_cells(&self) -> &Vec<(u16, u16)> {
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
