pub trait SandSimulation<Cell: TCell> {
    fn get_neighbor(&mut self, cell: &Cell, dir: Direction) -> Option<Cell>;
    fn swap_cells(&mut self, a: &mut Cell, b: &mut Cell);

    fn get_random_bool(&self) -> bool;

    fn handle_cell(&mut self, mut cell: Cell) {
        *cell.velocity() += 1;
        match cell.material() {
            CellMaterial::Sand => self.handle_sand(cell),
            _ => {}
        }
    }

    fn handle_sand(&mut self, mut cell: Cell) {
        let mut steps = cell.velocity().abs();

        while steps > 0 {
            if let Some(mut other) = self.get_neighbor(&cell, Direction::Down) {
                if self.handle_sand_collision(&mut cell, &mut other) {
                    steps -= 1;
                    continue;
                }
            }

            match self.get_random_bool() {
                true => {
                    if let Some(mut other) = self.get_neighbor(&cell, Direction::DownRight) {
                        if self.handle_sand_collision(&mut cell, &mut other) {
                            steps -= 1;
                            continue;
                        }
                    }
                }
                false => {
                    if let Some(mut other) = self.get_neighbor(&cell, Direction::DownLeft) {
                        if self.handle_sand_collision(&mut cell, &mut other) {
                            steps -= 1;
                            continue;
                        }
                    }
                }
            }

            *cell.velocity() = 0;
            break;
        }
    }

    fn handle_sand_collision(&mut self, cell: &mut Cell, other_cell: &mut Cell) -> bool {
        match other_cell.material() {
            CellMaterial::Air => {
                self.swap_cells(cell, other_cell);
                true
            }
            CellMaterial::Water => {
                *cell.velocity() -= 1;
                *other_cell.velocity() += 1;

                self.swap_cells(cell, other_cell);
                true
            }
            _ => false,
        }
    }
}

pub trait TCell {
    fn velocity(&mut self) -> &mut i8;
    fn material(&self) -> &CellMaterial;
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
