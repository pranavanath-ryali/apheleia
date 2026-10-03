use crate::{
    buffer::Buffer,
    cell::{Cell, layered::MultiLayerCellTrait},
    style::Style,
};

impl Buffer {
    pub fn write(&mut self, text: &str, position: (u16, u16), z: i8, style: Option<Style>) {
        for (offset_x, c) in text.chars().enumerate() {
            let i = position.1 * self.size.0 + position.0 + offset_x as u16;
            if position.1 >= self.size.1 {
                return;
            }

            if position.0 + offset_x as u16 >= self.size.0 {
                return;
            }

            let (_, z_cells) = &mut self.cells[i as usize];
            let cell = Cell {
                c,
                style: style.unwrap_or_default(),
                transparent: false,
            };
            z_cells.add_update_cell(z, &cell);

            self.changed_cells
                .push((position.0 + offset_x as u16, position.1));
        }
    }

    pub fn write_no_update(
        &mut self,
        text: &str,
        position: (u16, u16),
        z: i8,
        style: Option<Style>,
    ) {
        for (offset_x, c) in text.chars().enumerate() {
            let i = position.1 * self.size.0 + position.0 + offset_x as u16;
            if position.1 >= self.size.1 {
                return;
            }

            if position.0 + offset_x as u16 >= self.size.0 {
                return;
            }

            let (_, z_cells) = &mut self.cells[i as usize];
            z_cells.add_cell(
                z,
                Cell {
                    c,
                    style: style.unwrap_or_default(),
                    transparent: false,
                },
            );

            self.changed_cells
                .push((position.0 + offset_x as u16, position.1));
        }
    }

    pub fn clear_rect(&mut self, position: (u16, u16), size: (u16, u16)) {
        for y in position.1..(position.1 + size.1) {
            for x in position.0..(position.0 + size.0) {
                self.clear_cell((x, y));
            }
        }
    }

    pub fn clear_rect_on_z(&mut self, position: (u16, u16), size: (u16, u16), z: i8) {
        for y in position.1..(position.1 + size.1) {
            for x in position.0..(position.0 + size.0) {
                self.clear_cell_on_z((x, y), z);
            }
        }
    }

    pub fn clear_cell(&mut self, position: (u16, u16)) {
        self.cells[(position.1 * self.size.0 + position.0) as usize]
            .1
            .clear();
    }

    pub fn clear_cell_on_z(&mut self, position: (u16, u16), z: i8) {
        self.cells[(position.1 * self.size.0 + position.0) as usize]
            .1
            .clear_on_z(z);
    }
}
