pub mod render;

use smallvec::SmallVec;

use crate::{cell::{Cell, layered::MultiLayerCell}, grapheme::Grapheme, style::{Style, color::Color}};

pub struct Buffer {
    pub size: (u16, u16),

    pub cells: Vec<(Cell, MultiLayerCell)>, // Current view of cell
    pub changed_cells: Vec<(u16, u16)>,
}
impl Buffer {
    pub fn new(size: (u16, u16)) -> Self {
        // index = y * width + x
        let mut cells: Vec<(Cell, MultiLayerCell)> = vec![];
        for _ in 0..size.1 {
            for _ in 0..size.0 {
                cells.push((Cell::Transparent, SmallVec::new()));
            }
        }

        Self {
            size,
            cells,
            changed_cells: vec![],
        }
    }

    #[inline]
    pub fn clear_changed(&mut self) {
        self.changed_cells.clear();
    }
}
