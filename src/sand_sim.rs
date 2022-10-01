pub trait SandSimulation {
    fn width(&self) -> i16;
    fn height(&self) -> i16;

    fn get_cell(&self, x: i16, y: i16) -> Option<Cell>;
    fn set_cell(&mut self, cell: Cell);
    fn switch_cells(&mut self, cell1: Cell, cell2: Cell);

    fn update(&mut self) {
        // go from bottom to top
        for y in (0..self.height()).rev() {
            for x in (0..self.width()).rev() {
                // unwrap is safe here, as the loops are bound by own dimensions.
                // therefore we can't be outside bounds when calling get_cell() here
                let cell = self.get_cell(x, y).unwrap();
                match cell.material {
                    CellMaterial::Sand => self.handle_sand(cell),
                    CellMaterial::Air => {}
                    CellMaterial::SandGenerator => self.handle_sand_generator(cell),
                    CellMaterial::Water => self.handle_water(cell),
                    CellMaterial::WaterGenerator => self.handle_water_generator(cell),
                }
            }
        }
    }

    fn handle_water(&mut self, cell: Cell) {
        if let Some(cell_below) = self.get_cell(cell.x, cell.y + 1) {
            match cell_below.material {
                CellMaterial::Air => {
                    self.switch_cells(cell, cell_below);
                    return;
                }
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::Water => {}
                CellMaterial::WaterGenerator => {}
            }
        }

        if let Some(cell_below_right) = self.get_cell(cell.x + 1, cell.y + 1) {
            match cell_below_right.material {
                CellMaterial::Air => {
                    self.switch_cells(cell, cell_below_right);
                    return;
                }
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::Water => {}
                CellMaterial::WaterGenerator => {}
            }
        }
        if let Some(cell_below_left) = self.get_cell(cell.x - 1, cell.y + 1) {
            match cell_below_left.material {
                CellMaterial::Air => {
                    self.switch_cells(cell, cell_below_left);
                    return;
                }
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::Water => {}
                CellMaterial::WaterGenerator => {}
            }
        }
        if let Some(cell_right) = self.get_cell(cell.x + 1, cell.y) {
            match cell_right.material {
                CellMaterial::Air => {
                    self.switch_cells(cell, cell_right);
                    return;
                }
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::Water => {}
                CellMaterial::WaterGenerator => {}
            }
        }
        if let Some(cell_left) = self.get_cell(cell.x - 1, cell.y) {
            match cell_left.material {
                CellMaterial::Air => {
                    self.switch_cells(cell, cell_left);
                    return;
                }
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::Water => {}
                CellMaterial::WaterGenerator => {}
            }
        }
    }

    fn handle_water_generator(&mut self, cell: Cell) {
        if let Some(cell_below) = self.get_cell(cell.x, cell.y + 1) {
            match cell_below.material {
                CellMaterial::Air => {
                    let new_cell = Cell {
                        material: CellMaterial::Water,
                        ..cell_below
                    };
                    self.set_cell(new_cell);
                    return;
                }
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::Water => {}
                CellMaterial::WaterGenerator => {}
            }
        }
        if let Some(cell_below_right) = self.get_cell(cell.x + 1, cell.y + 1) {
            match cell_below_right.material {
                CellMaterial::Air => {
                    let new_cell = Cell {
                        material: CellMaterial::Water,
                        ..cell_below_right
                    };
                    self.set_cell(new_cell);
                    return;
                }
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::Water => {}
                CellMaterial::WaterGenerator => {}
            }
        }
        if let Some(cell_below_left) = self.get_cell(cell.x - 1, cell.y + 1) {
            match cell_below_left.material {
                CellMaterial::Air => {
                    let new_cell = Cell {
                        material: CellMaterial::Water,
                        ..cell_below_left
                    };
                    self.set_cell(new_cell);
                    return;
                }
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::Water => {}
                CellMaterial::WaterGenerator => {}
            }
        }
    }

    fn handle_sand_generator(&mut self, cell: Cell) {
        if let Some(cell_below) = self.get_cell(cell.x, cell.y + 1) {
            match cell_below.material {
                CellMaterial::Air => {
                    let new_cell = Cell {
                        material: CellMaterial::Sand,
                        ..cell_below
                    };
                    self.set_cell(new_cell);
                    return;
                }
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::Water => {}
                CellMaterial::WaterGenerator => {}
            }
        }
        if let Some(cell_below_right) = self.get_cell(cell.x + 1, cell.y + 1) {
            match cell_below_right.material {
                CellMaterial::Air => {
                    let new_cell = Cell {
                        material: CellMaterial::Sand,
                        ..cell_below_right
                    };
                    self.set_cell(new_cell);
                    return;
                }
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::Water => {}
                CellMaterial::WaterGenerator => {}
            }
        }
        if let Some(cell_below_left) = self.get_cell(cell.x - 1, cell.y + 1) {
            match cell_below_left.material {
                CellMaterial::Air => {
                    let new_cell = Cell {
                        material: CellMaterial::Sand,
                        ..cell_below_left
                    };
                    self.set_cell(new_cell);
                    return;
                }
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::Water => {}
                CellMaterial::WaterGenerator => {}
            }
        }
    }

    fn handle_sand(&mut self, cell: Cell) {
        if let Some(cell_below) = self.get_cell(cell.x, cell.y + 1) {
            match cell_below.material {
                CellMaterial::Air | CellMaterial::Water => {
                    self.switch_cells(cell, cell_below);
                    return;
                }
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::WaterGenerator => {}
            }
        }
        if let Some(cell_below_right) = self.get_cell(cell.x + 1, cell.y + 1) {
            match cell_below_right.material {
                CellMaterial::Air | CellMaterial::Water => {
                    self.switch_cells(cell, cell_below_right);
                    return;
                }
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::WaterGenerator => {}
            }
        }
        if let Some(cell_below_left) = self.get_cell(cell.x - 1, cell.y + 1) {
            match cell_below_left.material {
                CellMaterial::Air | CellMaterial::Water => {
                    self.switch_cells(cell, cell_below_left);
                    return;
                }
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::WaterGenerator => {}
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub x: i16,
    pub y: i16,
    pub material: CellMaterial,
}

#[derive(Copy, Clone, Debug)]
pub enum CellMaterial {
    Sand,
    SandGenerator,
    Water,
    WaterGenerator,
    Air,
    // Solid,
}
