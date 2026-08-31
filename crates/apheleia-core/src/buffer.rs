use smallvec::{self, SmallVec};

use crate::cell::{
    Cell,
    Grapheme::{self, Char},
    Modifiers, Style, update_cell,
};

pub trait MultiLayerCellTrait {
    fn result(&mut self) -> Cell;
}
impl MultiLayerCellTrait for SmallVec<[(u8, Cell); 2]> {
    fn result(&mut self) -> Cell {
        self.sort_by_key(|(z, _)| *z);

        let mut result_cell = Cell::Transparent;
        for (_, cell) in self.iter() {
            result_cell = update_cell(&result_cell, cell)
        }

        result_cell
    }
}

pub struct Buffer {
    pub size: (u16, u16),

    pub cells: Vec<(Cell, SmallVec<[(u8, Cell); 2]>)>, // Current view of cell
    pub changed_cells: Vec<(u16, u16)>,
}
impl Buffer {
    pub fn new(size: (u16, u16)) -> Self {
        // index = y * width + x
        let mut cells: Vec<(Cell, SmallVec<[(u8, Cell); 2]>)> = vec![];
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

    pub fn get_cell_mut(&mut self, position: (u16, u16)) -> &mut (Cell, SmallVec<[(u8, Cell); 2]>) {
        &mut self.cells[(position.1 * self.size.0 + position.0) as usize]
    }

    pub fn set_cell(&mut self, position: (u16, u16), cell: Cell) {
        self.cells[(position.1 * self.size.0 + position.0) as usize].0 = cell;
    }

    pub fn get_result_cell(&mut self, position: (u16, u16)) -> Cell {
        self.cells[(position.1 * self.size.0 + position.0) as usize]
            .1
            .result()
    }

    pub fn write(
        &mut self,
        text: &str,
        position: (u16, u16),
        z: u8,
        style: Style,
        alpha: Option<u8>,
    ) {
        let mut offset_x: u16 = 0;
        for c in text.chars() {
            let i = position.1 * self.size.0 + position.0 + offset_x;
            if position.1 >= self.size.1 {
                return;
            }

            if position.0 + offset_x >= self.size.0 {
                return;
            }

            let (_, z_cells) = &mut self.cells[i as usize];
            let mut found: bool = false;
            for (cell_z, cell) in z_cells.iter_mut() {
                if *cell_z == z {
                    *cell = if alpha.unwrap_or(255) == 255 {
                        Cell::Opaque {
                            grapheme: Grapheme::Char(c),
                            style,
                        }
                    } else {
                        Cell::Translucent {
                            grapheme: Grapheme::Char(c),
                            style,
                            alpha: alpha.unwrap(),
                        }
                    };
                    found = true;
                    break;
                }
            }

            if !found {
                z_cells.push((
                    z,
                    (if alpha.unwrap_or(255) == 255 {
                        Cell::Opaque {
                            grapheme: Grapheme::Char(c),
                            style,
                        }
                    } else {
                        Cell::Translucent {
                            grapheme: Grapheme::Char(c),
                            style,
                            alpha: alpha.unwrap(),
                        }
                    }),
                ));
            }

            self.changed_cells.push((i, position.1));
            offset_x += 1;
        }
    }

    #[inline]
    pub fn clear_changed(&mut self) {
        self.changed_cells.clear();
    }
}
