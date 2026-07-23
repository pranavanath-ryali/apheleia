use chrono::offset;
use indexmap::{IndexMap, IndexSet};
use log::info;
use rustc_hash::FxHashMap;

use crate::{node_buffer::NodeBuffer, rich_strings::RichString, style::Style, types::Vec2};

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
    pub size: Vec2,
    cells: Vec<Vec<Cell>>,
    diffed_cells: FxHashMap<u32, Vec<u32>>,
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

    pub fn clear_rect(&mut self, position: Vec2, size: Vec2) {
        for y in (position.y)..(position.y + size.y) {
            for x in (position.x)..(position.x + size.x) {
                let Some(row) = self.cells.get_mut(y as usize) else {
                    continue;
                };
                let Some(cell) = row.get_mut(x as usize) else {
                    continue;
                };

                *cell = Cell {
                    ..Default::default()
                };
                self.diffed_cells
                    .entry(y)
                    .and_modify(|v| v.push(x))
                    .or_insert(vec![x]);
            }
        }
    }

    pub fn render_buffer(&mut self, buf: &mut NodeBuffer) {
        let offset = buf.global_position;
        for (y, map) in buf.diffed_cells.iter() {
            for (x, cell) in map.iter() {
                info!("[BUFFER] written {} {}", x, y);
                let pos_x = offset.x + x;
                let pos_y = offset.y + y;

                if pos_x >= self.size.x || pos_y >= self.size.y {
                    continue;
                }

                if *cell == self.cells[(offset.y + y) as usize][(x + offset.x) as usize] {
                    continue;
                }

                self.cells[pos_y as usize][pos_x as usize] = *cell;
                self.diffed_cells
                    .entry(pos_y)
                    .and_modify(|v| {
                        v.push(pos_x);
                    })
                    .or_insert(vec![pos_x]);
            }
        }
    }

    pub fn get_cell(&self, position: Vec2) -> &Cell {
        &self.cells[position.y as usize][position.x as usize]
    }

    pub fn get_diffed_cells(&mut self) -> &mut FxHashMap<u32, Vec<u32>> {
        for y in self.diffed_cells.values_mut() {
            y.sort_unstable();
            y.dedup();
        }

        &mut self.diffed_cells
    }

    pub fn clear_diff(&mut self) {
        self.diffed_cells.clear();
    }
}
//
// #[cfg(test)]
// mod test_buffer {
//     use crate::style::StyleFlags;
//
//     use super::*;
//
//     #[test]
//     fn test_write() {
//         let mut buffer = Buffer::new(Vec2 { x: 10, y: 1 });
//         buffer.write_rich_string(Vec2 { x: 5, y: 0 }, RichString::new("He</bold/>llo"));
//
//         assert_eq!(
//             buffer.cells[0],
//             vec![
//                 Cell::default(),
//                 Cell::default(),
//                 Cell::default(),
//                 Cell::default(),
//                 Cell::default(),
//                 Cell {
//                     c: 'H',
//                     style: Style {
//                         ..Default::default()
//                     },
//                     written: true
//                 },
//                 Cell {
//                     c: 'e',
//                     style: Style {
//                         ..Default::default()
//                     },
//                     written: true
//                 },
//                 Cell {
//                     c: 'l',
//                     style: Style {
//                         flags: StyleFlags::BOLD,
//                         ..Default::default()
//                     },
//                     written: true
//                 },
//                 Cell {
//                     c: 'l',
//                     style: Style {
//                         flags: StyleFlags::BOLD,
//                         ..Default::default()
//                     },
//                     written: true
//                 },
//                 Cell {
//                     c: 'o',
//                     style: Style {
//                         flags: StyleFlags::BOLD,
//                         ..Default::default()
//                     },
//                     written: true
//                 },
//             ]
//         );
//     }
//
//     #[test]
//     fn test_resize() {
//         let mut buffer = Buffer::new(Vec2 { x: 10, y: 1 });
//         buffer.write_string(Vec2::zero(), "HelloWorld".to_string(), None);
//
//         buffer.resize(Vec2 { x: 20, y: 1 });
//         assert_eq!(
//             buffer.cells[0],
//             [
//                 Cell {
//                     c: 'H',
//                     style: Default::default(),
//                     written: true
//                 },
//                 Cell {
//                     c: 'e',
//                     style: Default::default(),
//                     written: true
//                 },
//                 Cell {
//                     c: 'l',
//                     style: Default::default(),
//                     written: true
//                 },
//                 Cell {
//                     c: 'l',
//                     style: Default::default(),
//                     written: true
//                 },
//                 Cell {
//                     c: 'o',
//                     style: Default::default(),
//                     written: true
//                 },
//                 Cell {
//                     c: 'W',
//                     style: Default::default(),
//                     written: true
//                 },
//                 Cell {
//                     c: 'o',
//                     style: Default::default(),
//                     written: true
//                 },
//                 Cell {
//                     c: 'r',
//                     style: Default::default(),
//                     written: true
//                 },
//                 Cell {
//                     c: 'l',
//                     style: Default::default(),
//                     written: true
//                 },
//                 Cell {
//                     c: 'd',
//                     style: Default::default(),
//                     written: true
//                 },
//                 Cell::default(),
//                 Cell::default(),
//                 Cell::default(),
//                 Cell::default(),
//                 Cell::default(),
//                 Cell::default(),
//                 Cell::default(),
//                 Cell::default(),
//                 Cell::default(),
//                 Cell::default(),
//             ]
//         );
//     }
// }
