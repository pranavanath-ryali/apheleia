use smallvec::SmallVec;

use crate::cell::Cell;

pub type MultiLayerCell = SmallVec<[(i8, Cell); 2]>;

pub trait MultiLayerCellTrait {
    fn add_update_cell(&mut self, z: i8, cell: Cell);
    fn add_cell(&mut self, z: i8, cell: Cell);

    fn clear(&mut self);
    fn clear_on_z(&mut self, z: i8);

    fn result(&mut self) -> Cell;
}
impl MultiLayerCellTrait for MultiLayerCell {
    fn add_update_cell(&mut self, z: i8, cell: Cell) {
        for (cell_z, c) in self.iter_mut() {
            if *cell_z == z {
                *c = c.clone().update_cell(&cell);

                return;
            }
        }

        self.push((z, cell));
    }
    fn add_cell(&mut self, z: i8, cell: Cell) {
        for (cell_z, c) in self.iter_mut() {
            if *cell_z == z {
                *c = cell.clone();

                return;
            }
        }

        self.push((z, cell));
    }

    fn clear(&mut self) {
        self.clear();
    }
    fn clear_on_z(&mut self, z: i8) {
        let Some((index, _)) = self
            .iter()
            .enumerate()
            .find(|(_, (cell_z, _))| *cell_z == z)
        else {
            return;
        };

        self.remove(index);
    }

    fn result(&mut self) -> Cell {
        self.sort_by_key(|(z, _)| *z);

        let mut result_cell = Cell::Transparent;
        for (_, cell) in self.iter() {
            result_cell = result_cell.clone().update_cell(cell);
        }

        match result_cell {
            Cell::Translucent {
                grapheme,
                style,
                alpha: _,
            } => Cell::Opaque { grapheme, style },
            _ => result_cell,
        }
    }
}
