pub trait SandSimulation<Cell: TCell> {
    fn get_neighbor(&self, cell: &Cell, dir: Direction) -> Option<Cell>;
    // fn move_cell(&mut self, cell: &Cell, dir: Direction);
    fn swap_cells(&mut self, a: &mut Cell, b: &mut Cell);
    fn mod_velocity(&mut self, velocity: i8, cell: &mut Cell);

    fn handle_cell(&mut self, mut cell: Cell) {
        self.mod_velocity(1, &mut cell);
        match cell.material() {
            CellMaterial::Sand => self.handle_sand(cell),
            _ => {}
        }
    }

    fn handle_sand(&mut self, mut cell: Cell) {
        let mut steps = cell.velocity().abs();

        while steps > 0 {
            steps -= 1;

            let directions = [Direction::Down, Direction::DownRight, Direction::DownLeft];
            for d in directions {
                if let Some(mut other) = self.get_neighbor(&cell, d) {
                    if self.handle_sand_collision(&mut cell, &mut other) {
                        continue;
                    }
                }
            }

            // if let Some(mut other) = valid_target {
            //     self.handle_sand_collision(&mut cell, &mut other);
            // } else {
            //     // nowhere to go -> stop velocity
            //     self.mod_velocity(cell.velocity() * -1, &mut cell);
            // }
        }
    }

    fn handle_sand_collision(&mut self, cell: &mut Cell, other_cell: &mut Cell) -> bool {
        match other_cell.material() {
            CellMaterial::Air => {
                self.swap_cells(cell, other_cell);
                true
            }
            CellMaterial::Water => {
                self.mod_velocity(-1, cell);
                self.mod_velocity(1, other_cell);

                self.swap_cells(cell, other_cell);
                // self.handle_sand(cell);
                // self.handle_water(below);
                true
            }
            _ => false,
        }
    }

    // fn handle_water(&mut self, cell: Cell) {
    //     if !cell.has_velocity() {
    //         ()
    //     }
    //
    //     if let Some(below) = self.get_neighbor(&cell, Direction::Down) {
    //         self.handle_water_collision(cell, below)
    //     }
    // }
    //
    // fn handle_water_collision(&mut self, mut cell: Cell, mut below: Cell) {
    //     match below.material() {
    //         CellMaterial::Air => {
    //             self.mod_velocity(cell.velocity() + 1, &mut cell);
    //             self.swap_cells(&mut cell, &mut below);
    //         }
    //         _ => {}
    //     }
    // }

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
        *self.velocity() != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellMaterial {
    Sand,
    SandGenerator,
    Water,
    WaterGenerator,
    Air,
    // Solid,
}

#[derive(Clone, Copy)]
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
