use smallvec::SmallVec;

use crate::cell::Cell;

pub type MultiLayerCell = SmallVec<[(i8, Cell); 2]>;
pub trait MultiLayerCellTrait {
    fn add_update_cell(&mut self, z: i8, cell: &Cell);
    fn add_cell(&mut self, z: i8, cell: Cell);

    fn clear_on_z(&mut self, z: i8);

    fn result(&mut self) -> Option<Cell>;
}
impl MultiLayerCellTrait for MultiLayerCell {
    fn add_update_cell(&mut self, z: i8, cell: &Cell) {
        let c_cell = self.iter_mut().find(|(cell_z, _)| *cell_z == z);
        if let Some((_, c)) = c_cell {
            c.update(cell);
            return;
        }

        self.push((z, cell.clone()));
    }
    fn add_cell(&mut self, z: i8, cell: Cell) {
        let c_cell = self.iter_mut().find(|(cell_z, _)| *cell_z == z);
        if let Some((_, c)) = c_cell {
            *c = cell;
            return;
        }

        self.push((z, cell));
    }

    fn clear_on_z(&mut self, z: i8) {
        if let Some(index) = self.iter().position(|(cell_z, _)| *cell_z == z) {
            self.swap_remove(index);
        }
    }

    fn result(&mut self) -> Option<Cell> {
        self.sort_by_key(|(z, _)| *z);

        let (_, mut result_cell) = self.first().cloned()?;
        for (_, cell) in &self[1..] {
            result_cell.update(cell);
        }
        Some(result_cell)
    }
}
