use smallvec::SmallVec;

use crate::cell::Cell;

pub type MultiLayerCell = SmallVec<[(i8, Cell); 2]>;

pub trait MultiLayerCellTrait {
    fn result(&mut self) -> Cell;
}
impl MultiLayerCellTrait for MultiLayerCell {
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
