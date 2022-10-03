use std::ptr;

pub struct SandSimulation {
    width: u16,
    height: u16,
    area: Vec<Vec<StoredCell>>,
}

impl SandSimulation {
    pub fn new(width: u16, height: u16) -> Self {
        let default_cell = StoredCell {
            material: CellMaterial::Air,
            velocity: 0,
        };
        Self {
            width,
            height,
            area: vec![vec![default_cell; width as usize]; height as usize],
        }
    }
    // fn get_cell(&self, x: i16, y: i16) -> Option<Cell>;
    // fn set_cell(&mut self, cell: Cell);
    // fn switch_cells(&mut self, cell1: Cell, cell2: Cell);

    fn update(&mut self) {
        // go from bottom to top
        for y in (0..self.height).rev() {
            for x in (0..self.width).rev() {
                // unwrap is safe here, as the loops are bound by own dimensions.
                // therefore we can't be outside bounds when calling get_cell() here
                let cell = self.get_cell(x, y).unwrap();

                match cell.material {
                    CellMaterial::Sand => self.update_sand(&cell),
                    CellMaterial::Air => {}
                    CellMaterial::SandGenerator => self.handle_sand_generator(&cell),
                    CellMaterial::Water => self.handle_water(&cell),
                    CellMaterial::WaterGenerator => self.handle_water_generator(&cell),
                }
            }
        }
    }
    /*
        fn handle_water(&self, cell: Cell) {
            if let Some(cell_below) = self.get_cell(cell.x, cell.y + 1) {
                match cell_below.material {
                    CellMaterial::Air => {
                        return self.switch_cells(cell, cell_below);
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
                        return self.switch_cells(cell, cell_below_right);
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
                        return self.switch_cells(cell, cell_below_left);
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
                        return self.switch_cells(cell, cell_right);
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
                        return self.switch_cells(cell, cell_left);
                    }
                    CellMaterial::Sand => {}
                    CellMaterial::SandGenerator => {}
                    CellMaterial::Water => {}
                    CellMaterial::WaterGenerator => {}
                }
            }
        }

        fn handle_water_generator(&self, cell: Cell) {
            if let Some(cell_below) = self.get_cell(cell.x, cell.y + 1) {
                match cell_below.material {
                    CellMaterial::Air => {
                        return self.set_cell(Cell {
                            material: CellMaterial::Water,
                            ..cell_below
                        });
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
                        return self.set_cell(Cell {
                            material: CellMaterial::Water,
                            ..cell_below_right
                        });
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
                        return self.set_cell(Cell {
                            material: CellMaterial::Water,
                            ..cell_below_left
                        });
                    }
                    CellMaterial::Sand => {}
                    CellMaterial::SandGenerator => {}
                    CellMaterial::Water => {}
                    CellMaterial::WaterGenerator => {}
                }
            }
        }

        fn handle_sand_generator(&self, cell: Cell) {
            if let Some(cell_below) = self.get_cell(cell.x, cell.y + 1) {
                match cell_below.material {
                    CellMaterial::Air => {
                        return self.set_cell(Cell {
                            material: CellMaterial::Sand,
                            ..cell_below
                        });
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
                        return self.set_cell(Cell {
                            material: CellMaterial::Sand,
                            ..cell_below_right
                        });
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
                        return self.set_cell(Cell {
                            material: CellMaterial::Sand,
                            ..cell_below_left
                        });
                    }
                    CellMaterial::Sand => {}
                    CellMaterial::SandGenerator => {}
                    CellMaterial::Water => {}
                    CellMaterial::WaterGenerator => {}
                }
            }
        }
    */

    fn get_cell(&self, x: u16, y: u16) -> Option<Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let stored = &self.area[y as usize][x as usize];

        Some(Cell {
            x,
            y,
            velocity: &mut stored.velocity,
            material: &mut stored.material,
        })
    }

    fn update_sand(&self, cell: &Cell) {
        if let Some(cell_below) = self.get_below(cell) {
            match cell_below.material {
                CellMaterial::Air => {
                    // keeps falling -> move without changing velocity
                    self.switch_cells(&cell, &cell_below);
                    return;
                }
                CellMaterial::Water => {}
                CellMaterial::Sand => {}
                CellMaterial::SandGenerator => {}
                CellMaterial::WaterGenerator => {}
            }
        }
        // if let Some(cell_below_right) = self.get_cell(cell.x + 1, cell.y + 1) {
        //     match cell_below_right.material {
        //         CellMaterial::Air => {
        //             return self.switch_cells(cell, cell_below_right);
        //         }
        //         CellMaterial::Water => {}
        //         CellMaterial::Sand => {}
        //         CellMaterial::SandGenerator => {}
        //         CellMaterial::WaterGenerator => {}
        //     }
        // }
        // if let Some(cell_below_left) = self.get_cell(cell.x - 1, cell.y + 1) {
        //     match cell_below_left.material {
        //         CellMaterial::Air => {
        //             return self.switch_cells(cell, cell_below_left);
        //         }
        //         CellMaterial::Water => {}
        //         CellMaterial::Sand => {}
        //         CellMaterial::SandGenerator => {}
        //         CellMaterial::WaterGenerator => {}
        //     }
        // }
    }

    fn get_below(&self, cell: &Cell) -> Option<Cell> {
        self.get_cell(cell.x, cell.y + 1)
    }

    fn switch_cells(&self, cell1: &Cell, cell2: &Cell) {
        let pa = ptr::addr_of_mut!(self.area[cell1.y as usize][cell1.x as usize]);
        let pb = ptr::addr_of_mut!(self.area[cell2.y as usize][cell2.x as usize]);
        unsafe {
            ptr::swap(pa, pb);
        }
    }

    fn handle_sand_generator(&self, p0: &Cell) {
        todo!()
    }
    fn handle_water(&self, p0: &Cell) {
        todo!()
    }
    fn handle_water_generator(&self, p0: &Cell) {
        todo!()
    }
}

pub struct Cell {
    x: u16,
    y: u16,
    velocity: u8,
    material: CellMaterial,
}

#[derive(Clone)]
struct StoredCell {
    velocity: u8,
    material: CellMaterial,
}

impl StoredCell {
    pub fn as_cell(&mut self, x: u16, y: u16) -> Cell {
        Cell {
            x,
            y,
            velocity: &mut self.velocity,
            material: &mut self.material,
        }
    }

    pub fn from_cell(cell: Cell) -> Self {
        Self {
            velocity: cell.velocity.clone(),
            material: cell.material.clone(),
        }
    }
}

#[derive(Clone)]
pub enum CellMaterial {
    Sand,
    SandGenerator,
    Water,
    WaterGenerator,
    Air,
    // Solid,
}
