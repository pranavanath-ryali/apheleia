use indexmap::IndexMap;
use log::info;
use rustc_hash::FxHashMap;

use crate::{buffer::Cell, rich_strings::RichString, style::Style, types::Vec2};

#[derive(Debug)]
pub struct NodeBuffer {
    pub global_position: Vec2,
    pub size: Vec2,
    pub(crate) diffed_cells: FxHashMap<u32, IndexMap<u32, Cell>>,
}
impl NodeBuffer {
    pub fn new(global_position: Vec2, size: Vec2) -> Self {
        Self {
            global_position,
            size,
            diffed_cells: Default::default(),
        }
    }

    pub fn get_size(&self) -> Vec2 {
        self.size
    }

    pub fn write_rich_string(&mut self, position: Vec2, text: &RichString) {
        let mut offset = Vec2::zero();
        for (c, style) in text.iter() {
            if c == '\n' {
                offset.y += 1;
                offset.x = offset.x.saturating_sub(1);
                continue;
            }

            if position.x + offset.x > self.size.x - 1 || position.y + offset.y > self.size.y - 1 {
                continue;
            }

            // TODO: Work on multi-width characters
            let cell = Cell {
                c,
                style,
                written: true,
            };
            self.diffed_cells
                .entry(position.y + offset.y)
                .and_modify(|map| {
                    map.insert(position.x + offset.x, cell);
                })
                .or_insert_with(|| {
                    let mut map: IndexMap<u32, Cell> = IndexMap::default();
                    map.insert(position.x + offset.x, cell);
                    map
                });
            offset.x += 1;
        }
    }

    pub fn write_string(&mut self, position: Vec2, text: &str, style: Option<Style>) {
        let mut offset = Vec2::zero();
        for c in text.chars() {
            if c == '\n' {
                offset.y += 1;
                offset.x = offset.x.saturating_sub(1);
                continue;
            }

            if position.x + offset.x > self.size.x - 1 || position.y + offset.y > self.size.y - 1 {
                continue;
            }

            // TODO: Work on multi-width characters
            let cell = Cell {
                c,
                style: style.unwrap_or_default(),
                written: true,
            };
            self.diffed_cells
                .entry(position.y + offset.y)
                .and_modify(|map| {
                    map.insert(position.x + offset.x, cell);
                })
                .or_insert_with(|| {
                    let mut map: IndexMap<u32, Cell> = IndexMap::default();
                    map.insert(position.x + offset.x, cell);
                    map
                });
            offset.x += 1;
        }
    }
}
