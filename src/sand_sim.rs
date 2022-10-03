pub trait SandSimulation<Cell: TCell> {
    fn get_neighbor(&self, cell: &Cell, dir: Direction) -> Option<Cell>;
    // fn move_cell(&mut self, cell: &Cell, dir: Direction);
    fn swap_cells(&mut self, a: &mut Cell, b: &mut Cell);
    fn mod_velocity(&mut self, velocity: i8, cell: &mut Cell);

    fn handle_cell(&mut self, cell: Cell) {
        match cell.material() {
            CellMaterial::Sand => self.handle_sand(cell),
            _ => {}
        }
    }

    fn handle_sand(&mut self, cell: Cell) {
        if !cell.has_velocity() {
            ()
        }

        if let Some(below) = self.get_neighbor(&cell, Direction::Down) {
            self.handle_sand_collision(cell, below)
        }
    }

    fn handle_sand_collision(&mut self, mut cell: Cell, mut below: Cell) {
        match below.material() {
            CellMaterial::Air => {
                self.mod_velocity(cell.velocity() + 1, &mut cell);
                self.swap_cells(&mut cell, &mut below);
            }
            CellMaterial::Water => {
                self.mod_velocity(-1, &mut cell);
                self.mod_velocity(1, &mut below);

                self.swap_cells(&mut cell, &mut below);
                self.handle_sand(cell);
                self.handle_water(below);
            }
            _ => {}
        }
    }

    fn handle_water(&mut self, cell: Cell) {
        if !cell.has_velocity() {
            ()
        }

        if let Some(below) = self.get_neighbor(&cell, Direction::Down) {
            self.handle_water_collision(cell, below)
        }
    }

    fn handle_water_collision(&mut self, cell: Cell, below: Cell) {
        todo!();
        // match below.material() {
        //     CellMaterial::Air => {
        //         self.swap_cells(cell, below);
        //     }
        //     CellMaterial::Water => {
        //         self.mod_velocity(cell, -1);
        //         self.mod_velocity(below, 1);
        //
        //         self.swap_cells(cell, below);
        //         self.handle_sand(cell);
        //         self.handle_water(below);
        //     }
        //     _ => {}
        // }
    }

    /*
    fn handle_water(&mut self, cell: Cell) {
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

    fn handle_water_generator(&mut self, cell: Cell) {
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

    fn handle_sand_generator(&mut self, cell: Cell) {
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
}

pub trait TCell {
    fn velocity(&self) -> &i8;
    fn material(&self) -> &CellMaterial;

    fn has_velocity(&self) -> bool {
        *self.velocity() == 0
    }
}

#[derive(Clone, Copy)]
pub enum CellMaterial {
    Sand,
    SandGenerator,
    Water,
    WaterGenerator,
    Air,
    // Solid,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
