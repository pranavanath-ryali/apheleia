use crate::{
    buffer::Buffer,
    cell::{Cell, layered::MultiLayerCellTrait},
    grapheme::Grapheme,
    style::Style,
};

impl Buffer {
    pub fn write(
        &mut self,
        text: &str,
        position: (u16, u16),
        z: i8,
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
            z_cells.add_update_cell(z, {
                if alpha.unwrap_or(255) == 255 {
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
                }
            });

            self.changed_cells.push((position.0 + offset_x, position.1));
            offset_x += 1;
        }
    }

    pub fn write_no_update(
        &mut self,
        text: &str,
        position: (u16, u16),
        z: i8,
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
            z_cells.add_cell(z, {
                if alpha.unwrap_or(255) == 255 {
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
                }
            });

            self.changed_cells.push((position.0 + offset_x, position.1));
            offset_x += 1;
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
