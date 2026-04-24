use std::{collections::HashMap, mem::replace};

use indexmap::IndexMap;
use rustc_hash::FxHashMap;

use crate::{rich_strings::RichString, style::Style, types::Vec2};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub c: char,
    pub style: Style,
    pub written: bool,
}
impl Default for Cell {
    fn default() -> Self {
        Self {
            c: ' ',
            style: Style::default(),
            written: false,
        }
    }
}

#[derive(Debug)]
pub struct Buffer {
    size: Vec2,
    cells: Vec<Vec<Cell>>,
    diffed_cells: FxHashMap<u16, IndexMap<u16, Cell>>,
}
impl Buffer {
    pub fn new(size: Vec2) -> Self {
        let mut cells: Vec<Vec<Cell>> = vec![];
        for _ in 0..size.y {
            let rows: Vec<Cell> = [Cell::default()].repeat(size.x as usize);
            cells.push(rows);
        }

        Self {
            size,
            cells,
            diffed_cells: Default::default(),
        }
    }

    pub fn get_size(&self) -> Vec2 {
        self.size
    }

    // pub fn shrink_size(&mut self, width: u16, height: u16) {
    // TODO: Reimplement this function to a more general resize function
    // pub fn resize(&mut self, new_size: Vec2) {
    //     if self.size.x > new_size.x && self.size.y > new_size.y {
    //         return;
    //     }

    //     let mut new_cells: Vec<Vec<Cell>> = vec![];
    //     for (y, row) in self.cells.iter().enumerate() {
    //         if y as u16 > new_size.y - 1 {
    //             break;
    //         }

    //         let mut new_row: Vec<Cell> = vec![];
    //         for (x, cell) in row.iter().enumerate() {
    //             if x as u16 > new_size.x - 1 {
    //                 break;
    //             }

    //             new_row.push(*cell);
    //         }
    //         new_cells.push(new_row);
    //     }

    //     self.size = new_size;
    //     self.cells = replace(&mut self.cells, new_cells);
    // }

    pub fn resize(&mut self, new_size: Vec2) {
        if self.size.x > new_size.x {
            self.cells.iter_mut().for_each(|row| {
                row.truncate(new_size.x as usize);
            });
        } else if self.size.x < new_size.x {
            self.cells.iter_mut().for_each(|row| {
                row.resize(new_size.x as usize, Cell::default());
            });
        }

        if self.size.y > new_size.y {
            self.cells.truncate(new_size.y as usize);
        } else if self.size.y < new_size.y {
            self.cells.resize_with(new_size.y as usize, || {
                vec![Cell::default(); new_size.x as usize]
            });
        }

        self.size = new_size;
    }

    pub fn write_string(&mut self, position: Vec2, text: String, style: Option<Style>) {
        let rich_string: RichString = match style {
            Some(style) => RichString::to_rich(&text, style),
            None => RichString::new(&text),
        };
        self.write_rich_string(position, rich_string);
    }

    pub fn write_rich_string(&mut self, position: Vec2, rich_string: RichString) {
        let mut offset = Vec2::zero();
        for (c, style) in rich_string.iter() {
            if c == '\n' {
                offset.y += 1;
                offset.x = offset.x.saturating_sub(1);
                continue;
            }

            if position.x + offset.x > self.size.x - 1 || position.y + offset.y > self.size.y - 1 {
                continue;
            }

            let cell =
                &mut self.cells[(position.y + offset.y) as usize][(position.x + offset.x) as usize];

            cell.c = c;
            cell.style = style;
            cell.written = true;

            offset.x += 1;
        }
    }

    // pub fn render_buffer(&mut self, offset_x: u16, offset_y: u16, buf: &mut Buffer) {
    pub fn render_buffer(&mut self, offset: Vec2, buf: &mut Buffer) {
        for (y, row) in buf.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if offset.x + x as u16 >= self.size.x || offset.y + y as u16 >= self.size.y {
                    continue;
                }

                if *cell == self.cells[y + offset.y as usize][x + offset.x as usize] {
                    continue;
                }

                if !cell.written {
                    continue;
                }

                self.cells[y + offset.y as usize][x + offset.x as usize] = *cell;
                self.diffed_cells
                    .entry(y as u16 + offset.y)
                    .or_default()
                    .insert(x as u16 + offset.x, *cell);
            }
        }
    }

    pub fn get_cell(&self, position: Vec2) -> &Cell {
        &self.cells[position.y as usize][position.x as usize]
    }

    pub fn get_diffed_cells(&self) -> &FxHashMap<u16, IndexMap<u16, Cell>> {
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
    fn test_write() {
        let mut buffer = Buffer::new(Vec2 { x: 10, y: 1 });
        buffer.write_rich_string(Vec2 { x: 5, y: 0 }, RichString::new("He</bold/>llo"));

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
                    },
                    written: true
                },
                Cell {
                    c: 'e',
                    style: Style {
                        ..Default::default()
                    },
                    written: true
                },
                Cell {
                    c: 'l',
                    style: Style {
                        flags: StyleFlags::BOLD,
                        ..Default::default()
                    },
                    written: true
                },
                Cell {
                    c: 'l',
                    style: Style {
                        flags: StyleFlags::BOLD,
                        ..Default::default()
                    },
                    written: true
                },
                Cell {
                    c: 'o',
                    style: Style {
                        flags: StyleFlags::BOLD,
                        ..Default::default()
                    },
                    written: true
                },
            ]
        );
    }

    #[test]
    fn test_resize() {
        let mut buffer = Buffer::new(Vec2 { x: 10, y: 1 });
        buffer.write_string(Vec2::zero(), "HelloWorld".to_string(), None);

        buffer.resize(Vec2 { x: 20, y: 1 });
        assert_eq!(
            buffer.cells[0],
            [
                Cell {
                    c: 'H',
                    style: Default::default(),
                    written: true
                },
                Cell {
                    c: 'e',
                    style: Default::default(),
                    written: true
                },
                Cell {
                    c: 'l',
                    style: Default::default(),
                    written: true
                },
                Cell {
                    c: 'l',
                    style: Default::default(),
                    written: true
                },
                Cell {
                    c: 'o',
                    style: Default::default(),
                    written: true
                },
                Cell {
                    c: 'W',
                    style: Default::default(),
                    written: true
                },
                Cell {
                    c: 'o',
                    style: Default::default(),
                    written: true
                },
                Cell {
                    c: 'r',
                    style: Default::default(),
                    written: true
                },
                Cell {
                    c: 'l',
                    style: Default::default(),
                    written: true
                },
                Cell {
                    c: 'd',
                    style: Default::default(),
                    written: true
                },
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default(),
                Cell::default(),
            ]
        );
    }
}
